use uuid::Uuid;

mod canvas_view_plugin;
pub(crate) use canvas_view_plugin::*;

pub(crate) mod canvas;

mod canvas_views;
use canvas_views::CanvasViews;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct WindowId(Uuid);

impl WindowId {
    pub fn new() -> Self {
        WindowId(Uuid::new_v4())
    }
}
