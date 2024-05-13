use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

mod web_ffi;
pub use web_ffi::*;

mod canvas_view;
use canvas_view::*;

mod bevy_app;

struct WorkerApp {
    pub(crate) app: App,
    /// 手动包装事件需要
    pub(crate) window: Entity,
    pub(crate) scale_factor: f32,
}

impl Deref for WorkerApp {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl DerefMut for WorkerApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl WorkerApp {
    pub(crate) fn new(app: App) -> Self {
        Self {
            app,
            window: Entity::PLACEHOLDER,
            scale_factor: 1.0,
        }
    }
}
