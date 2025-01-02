use crate::bevy_app::init_app;
use crate::{canvas_view::*, create_canvas_window, ActiveInfo, WorkerApp};
use bevy::app::PluginsState;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use js_sys::BigInt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    /// 在 app 初始化完成前，info! 无法使用，打印日志得用它
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);

    /// 发送 pick 列表
    ///
    /// 从 worker 环境发送
    #[wasm_bindgen(js_namespace = self)]
    pub(crate) fn send_pick_from_worker(list: js_sys::Array);
    /// 从主线程环境发送
    pub(crate) fn send_pick_from_rust(list: js_sys::Array);

    /// 执行阻塞
    /// 由于 wasm 环境不支持 std::thread, 交由 js 环境代为执行
    ///
    /// 在 worker 环境执行
    #[wasm_bindgen(js_namespace = self)]
    pub(crate) fn block_from_worker();
    /// 在主线程环境执行
    pub(crate) fn block_from_rust();
}

#[wasm_bindgen]
pub fn init_bevy_app() -> u64 {
    let mut app = init_app();
    // 添加自定义的 canvas 窗口插件
    app.add_plugins(CanvasViewPlugin);

    info!("init_bevy_app");

    // 包装成无生命周期的指针
    Box::into_raw(Box::new(app)) as u64
}

// 创建 Canvas 窗口
#[wasm_bindgen]
pub fn create_window_by_canvas(ptr: u64, canvas_id: &str, scale_factor: f32) {
    let app = unsafe { &mut *(ptr as *mut WorkerApp) };
    app.scale_factor = scale_factor;

    // 完成自定义 canvas 窗口的创建
    let canvas = Canvas::new(canvas_id, 1);
    let view_obj = ViewObj::from_canvas(canvas);

    create_window(app, view_obj, false);
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
    let view_obj = ViewObj::from_offscreen_canvas(offscreen_canvas);

    create_window(app, view_obj, true);
}

fn create_window(app: &mut WorkerApp, view_obj: ViewObj, is_in_worker: bool) {
    app.insert_non_send_resource(view_obj);

    let mut info = ActiveInfo::new();
    info.is_in_worker = is_in_worker;
    // 选中/高亮 资源
    app.insert_resource(info);

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
        app.window = entity;

        return 1;
    }
    0
}

/// 包装一个鼠标事件发送给 app
#[wasm_bindgen]
pub fn mouse_move(ptr: u64, x: f32, y: f32) {
    let app = unsafe { &mut *(ptr as *mut WorkerApp) };
    // 提前将逻辑像转换成物理像素
    let position = app.to_physical_size(x, y);
    let cursor_move = CursorMoved {
        window: app.window,
        position,
        delta: None,
    };
    app.world_mut().send_event(cursor_move);

    let mut active_info = app.world_mut().get_resource_mut::<ActiveInfo>().unwrap();
    active_info.remaining_frames = 10;
}

/// 鼠标左键按下
#[wasm_bindgen]
pub fn left_bt_down(ptr: u64, obj: JsValue, x: f32, y: f32) {
    let app = unsafe { &mut *(ptr as *mut WorkerApp) };
    let position = app.to_physical_size(x, y);
    let mut active_info = app.world_mut().get_resource_mut::<ActiveInfo>().unwrap();

    let value = bigint_to_u64(obj);
    if let Ok(v) = value {
        let entity = Entity::from_bits(v);
        active_info.drag = entity;
        active_info.last_drag_pos = position;
        // 当前要 drap 的对象同时也是 selection 对象
        let mut map: HashMap<Entity, u64> = HashMap::new();
        map.insert(entity, 0);
        active_info.selection = map;
    }
    active_info.remaining_frames = 10;
}

/// 鼠标左键松开
#[wasm_bindgen]
pub fn left_bt_up(ptr: u64) {
    let app = unsafe { &mut *(ptr as *mut WorkerApp) };
    let mut active_info = app.world_mut().get_resource_mut::<ActiveInfo>().unwrap();
    active_info.drag = Entity::PLACEHOLDER;

    active_info.remaining_frames = 10;
}

/// 设置 hover（高亮） 效果
#[wasm_bindgen]
pub fn set_hover(ptr: u64, arr: js_sys::Array) {
    let app = unsafe { &mut *(ptr as *mut WorkerApp) };
    let mut active_info = app.world_mut().get_resource_mut::<ActiveInfo>().unwrap();

    // 将 js hover 列表转换为 rust 对象
    let hover = to_map(arr);
    // 更新 hover 数据
    active_info.hover = hover;

    active_info.remaining_frames = 10;
}

/// 设置 选中 效果
#[wasm_bindgen]
pub fn set_selection(ptr: u64, arr: js_sys::Array) {
    let app = unsafe { &mut *(ptr as *mut WorkerApp) };
    let mut active_info = app.world_mut().get_resource_mut::<ActiveInfo>().unwrap();

    // 将 js selection 列表转换为 rust 对象
    let selection = to_map(arr);
    // 更新 hover 数据
    active_info.selection = selection;

    active_info.remaining_frames = 10;
}

/// 打开 / 关闭动画
#[wasm_bindgen]
pub fn set_auto_animation(ptr: u64, needs_animate: u32) {
    let app = unsafe { &mut *(ptr as *mut WorkerApp) };
    let mut active_info = app.world_mut().get_resource_mut::<ActiveInfo>().unwrap();
    active_info.auto_animate = needs_animate > 0;
}

/// 帧绘制
///
/// render 运行在 worker 中时，主线程 post 绘制 msg 时可能 render 还没有完成当前帧的更新
///
/// TODO：需要检测帧依赖的资源是否已加载完成，否则可能提交的 update 累积会导致栈溢出
#[wasm_bindgen]
pub fn enter_frame(ptr: u64) {
    // 获取到指针指代的 Rust 对象的可变借用
    let app = unsafe { &mut *(ptr as *mut WorkerApp) };
    {
        // 检查执行帧渲染的条件
        let mut active_info = app.world_mut().get_resource_mut::<ActiveInfo>().unwrap();
        if !active_info.auto_animate && active_info.remaining_frames == 0 {
            return;
        }
        if active_info.remaining_frames > 0 {
            active_info.remaining_frames -= 1;
        }
    }

    if app.plugins_state() != PluginsState::Cleaned {
        if app.plugins_state() != PluginsState::Ready {
            // #[cfg(not(target_arch = "wasm32"))]
            // tick_global_task_pools_on_main_thread();
        } else {
            app.finish();
            app.cleanup();
        }
    } else {
        // 模拟阻塞
        let active_info = app.world().get_resource::<ActiveInfo>().unwrap();
        if active_info.is_in_worker {
            block_from_worker();
        } else {
            block_from_rust();
        }

        app.update();
    }
}

// 释放 engine 实例
#[wasm_bindgen]
pub fn release_app(ptr: u64) {
    // 将指针转换为其指代的实际 Rust 对象，同时也拿回此对象的内存管理权
    let app: Box<App> = unsafe { Box::from_raw(ptr as *mut _) };
    crate::close_bevy_window(app);
}

/// 将 js 数组转换为 rust HashMap
fn to_map(arr: js_sys::Array) -> HashMap<Entity, u64> {
    let mut map: HashMap<Entity, u64> = HashMap::new();
    let length = arr.length();
    for i in 0..length {
        let value = bigint_to_u64(arr.get(i));
        if let Ok(v) = value {
            let entity = Entity::from_bits(v);
            map.insert(entity, v);
        }
    }
    map
}

/// 将 js BigInt 转换成 rust u64
/// 测试了几种方式，只有下边的能方式转换成功
fn bigint_to_u64(value: JsValue) -> Result<u64, JsValue> {
    if let Ok(big_int) = BigInt::new(&value) {
        // 转换为字符串，基数为10
        let big_int_str = big_int.to_string(10).unwrap().as_string();
        let big_int_u64: Result<u64, _> = big_int_str.unwrap().parse::<u64>();
        if let Ok(number) = big_int_u64 {
            return Ok(number);
        }
    }
    Err(JsValue::from_str("Value is not a valid u64"))
}
