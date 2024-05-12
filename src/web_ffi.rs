use crate::bevy_app::init_app;
use crate::{canvas::*, canvas_view, create_canvas_window};
use bevy::app::PluginsState;
use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    // 在 app 初始化完成前，info! 无法使用，打印日志得用它
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
    // #[wasm_bindgen(js_namespace = console)]
    // pub(crate) fn log(s: &web_sys::OffscreenCanvas);
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
    let app = unsafe { &mut *(ptr as *mut App) };
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
    let app = unsafe { &mut *(ptr as *mut App) };

    let offscreen_canvas = OffscreenCanvas::new(canvas, scale_factor, 1);
    let view_obj = ViewObj::Offscreen(offscreen_canvas);
    app.insert_non_send_resource(view_obj);

    create_canvas_window(app);
}

/// 完成插件初始化
/// 初始化未完成之前不能调用帧绘制
#[wasm_bindgen]
pub fn finish_init(ptr: u64) -> u32 {
    let app = unsafe { &mut *(ptr as *mut App) };

    // 创建 device/queue 是异步的，不确定创建完成的时机
    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();

        return 1;
    }
    0
}

/// 帧绘制
/// 返回一个非 0 标记，表示当前帧的 cpu 端指令已执行完
/// render 运行在 worker 中时，主线程 post 绘制 msg 时可能 render 还没有完成当前帧的更新
///
/// TODO：需要检测帧依赖的资源是否已加载完成，否则可能提交的 update 累积会导致栈溢出
#[wasm_bindgen]
pub fn enter_frame(ptr: u64) -> i32 {
    // 获取到指针指代的 Rust 对象的可变借用
    let app = unsafe { &mut *(ptr as *mut App) };

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

    1
}
