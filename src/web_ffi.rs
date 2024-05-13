use crate::bevy_app::init_app;
use crate::{canvas::*, canvas_view, create_canvas_window, WorkerApp};
use bevy::app::PluginsState;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    // 在 app 初始化完成前，info! 无法使用，打印日志得用它
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
}

#[wasm_bindgen]
pub fn init_bevy_app() -> u64 {
    let mut app = init_app();
    // 添加自定义的 canvas 窗口插件
    app.add_plugins(canvas_view::CanvasViewPlugin);

    info!("init_bevy_app");

    // 包装成无生命周期的指针
    Box::into_raw(Box::new(app)) as u64
}

// 创建 Canvas 窗口
#[wasm_bindgen]
pub fn create_window_by_canvas(ptr: u64) {
    let app = unsafe { &mut *(ptr as *mut WorkerApp) };
    // 完成自定义 canvas 窗口的创建
    let canvas = Canvas::new("app-canvas", 1);
    let view_obj = ViewObj::Canvas(canvas);
    // let offscreen_canvas = OffscreenCanvas::from(&canvas);
    // let view_obj = ViewObj::Offscreen(offscreen_canvas);
    app.insert_non_send_resource(view_obj);

    create_canvas_window(app);
}

/// 创建离屏窗口
#[wasm_bindgen]
pub fn create_window_by_offscreen_canvas(
    ptr: u64,
    canvas: web_sys::OffscreenCanvas,
    scale_factor: f32,
) {
    let app = unsafe { &mut *(ptr as *mut WorkerApp) };
    app.scale_factor = scale_factor;

    let offscreen_canvas = OffscreenCanvas::new(canvas, scale_factor, 1);
    let view_obj = ViewObj::Offscreen(offscreen_canvas);
    app.insert_non_send_resource(view_obj);

    create_canvas_window(app);
}

/// 是否已完成插件初始化
/// 初始化未完成之前不能调用帧绘制
#[wasm_bindgen]
pub fn is_preparation_completed(ptr: u64) -> u32 {
    let app = unsafe { &mut *(ptr as *mut WorkerApp) };

    // 创建 device/queue 是异步的，不确定创建完成的时机
    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();

        // 将 window 对象直接存到 app 上, 避免后继查询
        let mut windows_system_state: SystemState<Query<(Entity, &Window)>> =
            SystemState::from_world(app.world_mut());
        let (entity, _) = windows_system_state.get(app.world_mut()).single();
        app.window = entity.clone();

        return 1;
    }
    0
}

/// 包装一个鼠标事件发送给 app
#[wasm_bindgen]
pub fn mouse_move(ptr: u64, x: f32, y: f32) {
    let app = unsafe { &mut *(ptr as *mut WorkerApp) };
    // 提前将逻辑像转换成物理像素
    let scale_factor = app.scale_factor;
    let cursor_move = CursorMoved {
        window: app.window,
        position: Vec2::new(x * scale_factor, y * scale_factor),
        delta: None,
    };
    app.world_mut().send_event(cursor_move);
}

/// 帧绘制
/// render 运行在 worker 中时，主线程 post 绘制 msg 时可能 render 还没有完成当前帧的更新
///
/// TODO：需要检测帧依赖的资源是否已加载完成，否则可能提交的 update 累积会导致栈溢出
#[wasm_bindgen]
pub fn enter_frame(ptr: u64) {
    // 获取到指针指代的 Rust 对象的可变借用
    let app = unsafe { &mut *(ptr as *mut WorkerApp) };

    if app.plugins_state() != PluginsState::Cleaned {
        if app.plugins_state() != PluginsState::Ready {
            // #[cfg(not(target_arch = "wasm32"))]
            // tick_global_task_pools_on_main_thread();
        } else {
            app.finish();
            app.cleanup();
        }
    } else {
        app.update();
    }
}
