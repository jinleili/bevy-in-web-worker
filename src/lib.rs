use bevy::{ecs::system::SystemState, prelude::*, utils::HashMap, window::WindowCloseRequested};
use std::ops::{Deref, DerefMut};

mod web_ffi;
pub use web_ffi::*;

mod canvas_view;
use canvas_view::*;

mod ray_pick;

mod bevy_app;

pub struct WorkerApp {
    pub app: App,
    /// 手动包装事件需要
    pub window: Entity,
    pub scale_factor: f32,
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
    pub fn new(app: App) -> Self {
        Self {
            app,
            window: Entity::PLACEHOLDER,
            scale_factor: 1.0,
        }
    }

    pub fn to_physical_size(&self, x: f32, y: f32) -> Vec2 {
        Vec2::new(x * self.scale_factor, y * self.scale_factor)
    }
}

#[derive(Debug, Resource)]
pub(crate) struct ActiveInfo {
    pub hover: HashMap<Entity, u64>,
    pub selection: HashMap<Entity, u64>,
    /// 响应拖动的对象
    pub drag: Entity,
    /// 上一帧的拖动位置
    pub last_drag_pos: Vec2,
    /// 是否运行在 worker 中
    pub is_in_worker: bool,
    /// 是否自动执行场景对象的旋转动画
    pub auto_animate: bool,
    /// 剩余帧数
    ///
    /// 当关闭了自动运行的帧动画之后，场景将仅由鼠标事件驱动更新。由于帧渲染需要由 requestAnimationFrame 驱动
    /// 来保持与浏览器显示刷新的同步，所以鼠标事件不会直接调用 app.update(), 而是重置此待更新的帧数
    pub remaining_frames: u32,
}

impl ActiveInfo {
    pub fn new() -> Self {
        ActiveInfo {
            hover: HashMap::new(),
            selection: HashMap::new(),
            drag: Entity::PLACEHOLDER,
            last_drag_pos: Vec2::ZERO,
            is_in_worker: false,
            auto_animate: true,
            remaining_frames: 0,
        }
    }
}

pub(crate) fn close_bevy_window(mut app: Box<App>) {
    let mut windows_state: SystemState<Query<(Entity, &mut Window)>> =
        SystemState::from_world(app.world_mut());
    let windows = windows_state.get_mut(app.world_mut());
    let (entity, _window) = windows.iter().last().unwrap();
    app.world_mut()
        .send_event(WindowCloseRequested { window: entity });
    windows_state.apply(app.world_mut());

    app.update();
}
