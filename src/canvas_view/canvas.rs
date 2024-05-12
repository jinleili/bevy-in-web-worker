use std::{ops::Deref, ptr::NonNull};
use wasm_bindgen::{JsCast, JsValue};

use crate::web_ffi;

#[derive(Debug, Clone)]
pub struct Canvas {
    element: web_sys::HtmlCanvasElement,
    pub scale_factor: f32,
    handle: u32,
}

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

impl raw_window_handle::HasWindowHandle for Canvas {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        use raw_window_handle::{RawWindowHandle, WebCanvasWindowHandle, WindowHandle};

        let value: &JsValue = &self.element;
        let obj: NonNull<std::ffi::c_void> = NonNull::from(value).cast();
        let handle = WebCanvasWindowHandle::new(obj);
        let raw = RawWindowHandle::WebCanvas(handle);
        unsafe { Ok(WindowHandle::borrow_raw(raw)) }
    }
}

impl raw_window_handle::HasDisplayHandle for Canvas {
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
    scale_factor: f32,
    handle: u32,
}

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
}

impl Deref for OffscreenCanvas {
    type Target = web_sys::OffscreenCanvas;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<&Canvas> for OffscreenCanvas {
    fn from(value: &Canvas) -> Self {
        let offscreen = value.element.transfer_control_to_offscreen().unwrap();
        let handle = value.handle;
        Self::new(offscreen, value.scale_factor, handle)
    }
}

impl raw_window_handle::HasWindowHandle for OffscreenCanvas {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        use raw_window_handle::{RawWindowHandle, WebOffscreenCanvasWindowHandle, WindowHandle};

        let value: &JsValue = &self.inner;
        let obj: NonNull<std::ffi::c_void> = NonNull::from(value).cast();
        let handle = WebOffscreenCanvasWindowHandle::new(obj);
        let raw = RawWindowHandle::WebOffscreenCanvas(handle);
        unsafe { Ok(WindowHandle::borrow_raw(raw)) }
    }
}

impl raw_window_handle::HasDisplayHandle for OffscreenCanvas {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        use raw_window_handle::{DisplayHandle, RawDisplayHandle, WebDisplayHandle};
        let handle = WebDisplayHandle::new();
        let raw = RawDisplayHandle::Web(handle);
        unsafe { Ok(DisplayHandle::borrow_raw(raw)) }
    }
}
