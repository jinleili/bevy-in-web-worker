use bevy::window::WindowWrapper;
use std::{ops::Deref, ptr::NonNull};
use wasm_bindgen::{JsCast, JsValue};

// 封装 ViewObj 来同时支持 Canvas 与 Offscreen
#[derive(Debug)]
pub enum ViewObj {
    Canvas(WindowWrapper<CanvasWrapper>),
    Offscreen(WindowWrapper<OffscreenCanvasWrapper>),
}

impl ViewObj {
    pub fn from_canvas(canvas: Canvas) -> Self {
        ViewObj::Canvas(WindowWrapper::new(CanvasWrapper::new(canvas)))
    }

    pub fn from_offscreen_canvas(canvas: OffscreenCanvas) -> Self {
        ViewObj::Offscreen(WindowWrapper::new(OffscreenCanvasWrapper::new(canvas)))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct SendSyncWrapper<T>(pub(crate) T);

unsafe impl<T> Send for SendSyncWrapper<T> {}
unsafe impl<T> Sync for SendSyncWrapper<T> {}

#[derive(Debug)]
pub struct CanvasWrapper(SendSyncWrapper<Canvas>);
impl CanvasWrapper {
    pub fn new(canvas: Canvas) -> Self {
        CanvasWrapper(SendSyncWrapper(canvas))
    }
}

impl Deref for CanvasWrapper {
    type Target = Canvas;

    fn deref(&self) -> &Self::Target {
        &self.0 .0
    }
}

#[derive(Debug)]
pub struct OffscreenCanvasWrapper(SendSyncWrapper<OffscreenCanvas>);
impl OffscreenCanvasWrapper {
    pub fn new(canvas: OffscreenCanvas) -> Self {
        OffscreenCanvasWrapper(SendSyncWrapper(canvas))
    }
}

impl Deref for OffscreenCanvasWrapper {
    type Target = OffscreenCanvas;

    fn deref(&self) -> &Self::Target {
        &self.0 .0
    }
}

#[derive(Debug, Clone)]
pub struct Canvas {
    element: web_sys::HtmlCanvasElement,
    pub scale_factor: f32,
    handle: u32,
}

#[allow(dead_code)]
impl Canvas {
    pub fn new(selectors: &str, handle: u32) -> Self {
        // 0 is reserved for window itself.
        assert!(handle > 0);

        // 添加 `raw-window-handle` 需要的属性/值.
        let (element, scale_factor) = Self::get_canvas_element(selectors);
        element
            .set_attribute("data-raw-handle", handle.to_string().as_str())
            .unwrap();

        Self {
            element,
            scale_factor,
            handle,
        }
    }

    pub fn get_canvas_element(element_id: &str) -> (web_sys::HtmlCanvasElement, f32) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document
            .get_element_by_id(element_id)
            .expect("页面中找不到 canvas 元素 ");

        let canvas = element.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let scale_factor = window.device_pixel_ratio() as f32;

        (canvas, scale_factor)
    }

    #[inline]
    pub fn handle(&self) -> u32 {
        self.handle
    }

    pub fn logical_resolution(&self) -> (f32, f32) {
        let width = self.element.width();
        let height = self.element.height();
        (width as f32, height as f32)
    }
}

impl Deref for Canvas {
    type Target = web_sys::HtmlCanvasElement;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl raw_window_handle::HasWindowHandle for CanvasWrapper {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        use raw_window_handle::{RawWindowHandle, WebCanvasWindowHandle, WindowHandle};

        let value: &JsValue = &self.0 .0.element;
        let obj: NonNull<std::ffi::c_void> = NonNull::from(value).cast();
        let handle = WebCanvasWindowHandle::new(obj);
        let raw = RawWindowHandle::WebCanvas(handle);
        unsafe { Ok(WindowHandle::borrow_raw(raw)) }
    }
}

impl raw_window_handle::HasDisplayHandle for CanvasWrapper {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        use raw_window_handle::{DisplayHandle, RawDisplayHandle, WebDisplayHandle};
        let handle = WebDisplayHandle::new();
        let raw = RawDisplayHandle::Web(handle);
        unsafe { Ok(DisplayHandle::borrow_raw(raw)) }
    }
}

#[derive(Debug, Clone)]
pub struct OffscreenCanvas {
    inner: web_sys::OffscreenCanvas,
    pub scale_factor: f32,
    handle: u32,
}

#[allow(dead_code)]
impl OffscreenCanvas {
    pub const fn new(canvas: web_sys::OffscreenCanvas, scale_factor: f32, handle: u32) -> Self {
        Self {
            inner: canvas,
            scale_factor,
            handle,
        }
    }

    pub fn each(self) -> (web_sys::OffscreenCanvas, u32) {
        (self.inner, self.handle)
    }

    pub fn logical_resolution(&self) -> (f32, f32) {
        let width = self.inner.width();
        let height = self.inner.height();
        (width as f32, height as f32)
    }
}

impl From<&Canvas> for OffscreenCanvas {
    fn from(value: &Canvas) -> Self {
        let offscreen = value.element.transfer_control_to_offscreen().unwrap();
        let handle = value.handle;
        Self::new(offscreen, value.scale_factor, handle)
    }
}

impl raw_window_handle::HasWindowHandle for OffscreenCanvasWrapper {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        use raw_window_handle::{RawWindowHandle, WebOffscreenCanvasWindowHandle, WindowHandle};

        let value: &JsValue = &self.0 .0.inner;
        let obj: NonNull<std::ffi::c_void> = NonNull::from(value).cast();
        let handle = WebOffscreenCanvasWindowHandle::new(obj);
        let raw = RawWindowHandle::WebOffscreenCanvas(handle);
        unsafe { Ok(WindowHandle::borrow_raw(raw)) }
    }
}

impl raw_window_handle::HasDisplayHandle for OffscreenCanvasWrapper {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        use raw_window_handle::{DisplayHandle, RawDisplayHandle, WebDisplayHandle};
        let handle = WebDisplayHandle::new();
        let raw = RawDisplayHandle::Web(handle);
        unsafe { Ok(DisplayHandle::borrow_raw(raw)) }
    }
}
