use crate::bevy_app::{animate_materials, setup};
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
}

fn init_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    // 添加自定义的 canvas 窗口插件
    app.add_plugins(canvas_view::CanvasViewPlugin);

    app.add_systems(Startup, setup)
        .add_systems(Update, animate_materials);

    app
}

#[wasm_bindgen]
pub fn init_bevy_app() -> u64 {
    let mut app = init_app();

    // 完成自定义 canvas 窗口的创建
    let canvas = Canvas::new("app-canvas", 1);
    // let offscreen_canvas = OffscreenCanvas::from(&canvas);
    app.insert_non_send_resource(canvas);
    create_canvas_window(&mut app);

    // 包装成无生命周期的指针
    Box::into_raw(Box::new(app)) as u64
}

#[wasm_bindgen]
pub fn finish_init(ptr: u64) -> u32 {
    let app = unsafe { &mut *(ptr as *mut App) };

    // 完成插件初始化
    // 创建 device/queue 是异步的，不确定创建完成的时机
    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();

        return 1;
    }
    0
}

#[wasm_bindgen]
pub fn enter_frame(ptr: u64) {
    // 获取到指针指代的 Rust 对象的可变借用
    let app = unsafe { &mut *(ptr as *mut App) };
    app.update();
}
