use app_surface::{CanvasWrapper, OffscreenCanvasWrapper};
use bevy::window::WindowWrapper;
use uuid::Uuid;

pub(crate) use app_surface::{Canvas, OffscreenCanvas};

mod canvas_view_plugin;
pub(crate) use canvas_view_plugin::*;

mod canvas_views;
use canvas_views::CanvasViews;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct WindowId(Uuid);

impl WindowId {
    pub fn new() -> Self {
        WindowId(Uuid::new_v4())
    }
}

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
