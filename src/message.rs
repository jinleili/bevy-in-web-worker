use super::canvas::Canvas;
use std::mem;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsMessage(pub u64);

impl JsMessage {
    /// Common message group.
    const COMMON: u64 = 0;
    /// Window message group.
    const WINDOW: u64 = 1 << 32;
    /// Mouse message group.
    const MOUSE: u64 = 2 << 32;

    /// A common message requesting initialization of main object.
    pub const INIT_INNER: u64 = Self::COMMON | 1;
    pub const INIT: Self = Self(Self::INIT_INNER);

    /// Window resize message.
    pub const WINDOW_RESIZE_INNER: u64 = Self::WINDOW | 1;
    pub const WINDOW_RESIZE: Self = Self(Self::WINDOW_RESIZE_INNER);

    /// Mouse move message.
    pub const MOUSE_MOVE_INNER: u64 = Self::MOUSE | 1;
    pub const MOUSE_MOVE: Self = Self(Self::MOUSE_MOVE_INNER);
    /// Mouse click message.
    pub const MOUSE_CLICK_INNER: u64 = Self::MOUSE | 2;
    pub const MOUSE_CLICK: Self = Self(Self::MOUSE_CLICK_INNER);

    /// Reinterprets value to f64 in bit level.
    /// Then convert it into JsValue.
    /// Use [`Self::from_f64()`] to recover.
    #[inline]
    pub fn into_jsvalue(self) -> JsValue {
        let f: f64 = unsafe { mem::transmute(self) };
        JsValue::from(f)
    }

    /// Converts message given from JS side into original type.
    #[inline]
    pub fn from_f64(value: JsValue) -> Self {
        let f = value.as_f64().unwrap();
        Self(f.to_bits())
    }
}

#[derive(Debug, Clone)]
pub struct JsResizeMessage {
    pub scale_factor: f64,
    pub handle: f64,
    pub width: f64,
    pub height: f64,
}

impl JsResizeMessage {
    /// Returns number of fileds.
    /// Use this to prepare [`js_sys::Array`] buffer.
    #[inline]
    pub const fn field_num() -> u32 {
        4
    }

    /// Helper
    pub fn set_js_array(arr: &js_sys::Array, canvas: &Canvas, scale_factor: f64, offset: u32) {
        arr.set(offset, JsValue::from(scale_factor));
        arr.set(offset + 1, JsValue::from(canvas.handle()));
        arr.set(offset + 2, JsValue::from(canvas.client_width()));
        arr.set(offset + 3, JsValue::from(canvas.client_height()));
    }

    /// Helper
    pub fn from_js_array(arr: js_sys::Array, offset: u32) -> Self {
        // Safety: Infallible.
        unsafe {
            Self {
                scale_factor: arr.get(offset).as_f64().unwrap_unchecked(),
                handle: arr.get(offset + 1).as_f64().unwrap_unchecked(),
                width: arr.get(offset + 2).as_f64().unwrap_unchecked(),
                height: arr.get(offset + 3).as_f64().unwrap_unchecked(),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsMouseMessage {
    pub scale_factor: f64,
    pub button: f64,
    pub client_x: f64,
    pub client_y: f64,
    pub movement_x: f64,
    pub movement_y: f64,
    pub offset_x: f64,
    pub offset_y: f64,
}

impl JsMouseMessage {
    /// Returns number of fileds.
    /// Use this to prepare [`js_sys::Array`] buffer.
    #[inline]
    pub const fn field_num() -> u32 {
        8
    }

    /// Helper
    pub fn set_js_array(
        arr: &js_sys::Array,
        event: web_sys::MouseEvent,
        scale_factor: f64,
        offset: u32,
    ) {
        arr.set(offset, JsValue::from(scale_factor));
        arr.set(offset + 1, JsValue::from(event.button()));
        arr.set(offset + 2, JsValue::from(event.client_x()));
        arr.set(offset + 3, JsValue::from(event.client_y()));
        arr.set(offset + 4, JsValue::from(event.movement_x()));
        arr.set(offset + 5, JsValue::from(event.movement_y()));
        arr.set(offset + 6, JsValue::from(event.offset_x()));
        arr.set(offset + 7, JsValue::from(event.offset_y()));
    }

    /// Helper
    pub fn from_js_array(arr: js_sys::Array, offset: u32) -> Self {
        // Safety: Infallible.
        unsafe {
            Self {
                scale_factor: arr.get(offset).as_f64().unwrap_unchecked(),
                button: arr.get(offset + 1).as_f64().unwrap_unchecked(),
                client_x: arr.get(offset + 2).as_f64().unwrap_unchecked(),
                client_y: arr.get(offset + 3).as_f64().unwrap_unchecked(),
                movement_x: arr.get(offset + 4).as_f64().unwrap_unchecked(),
                movement_y: arr.get(offset + 5).as_f64().unwrap_unchecked(),
                offset_x: arr.get(offset + 6).as_f64().unwrap_unchecked(),
                offset_y: arr.get(offset + 7).as_f64().unwrap_unchecked(),
            }
        }
    }
}
