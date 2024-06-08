use crate::{
    bevy_app::{ActiveState, CurrentVolume},
    send_pick_from_rust, send_pick_from_worker, ActiveInfo,
};
use bevy::math::bounding::RayCast3d;
use bevy::utils::hashbrown::HashMap;
use bevy::{input::mouse::MouseWheel, prelude::*};
use wasm_bindgen::JsValue;

/// 基于 ray pick 的 hover / 选中 / 拖动
pub(crate) struct RayPickPlugin;

impl Plugin for RayPickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mouse_events_system, update_active));
    }
}

fn mouse_events_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut app_info: ResMut<ActiveInfo>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(Entity, &CurrentVolume, &mut Transform), With<ActiveState>>,
) {
    // 对于拖动，只使用最后一条 move 事件就能得到最终的移动偏移量
    if app_info.drag != Entity::PLACEHOLDER && !cursor_moved_events.is_empty() {
        let last_cursor_event: Option<&CursorMoved> = cursor_moved_events.read().last();
        if let Some(last_move) = last_cursor_event {
            let (camera, global_transform) = cameras.get_single().unwrap();

            for (entity, _, mut transform) in query.iter_mut() {
                if app_info.drag == entity {
                    let cur =
                        screen_to_world(last_move.position, camera, global_transform).unwrap();
                    let last =
                        screen_to_world(app_info.last_drag_pos, camera, global_transform).unwrap();
                    let offset = cur - last;
                    transform.translation += Vec3::new(offset.x, offset.y, 0.0);

                    app_info.last_drag_pos = last_move.position;
                }
            }
        }
        return;
    }

    // hover 列表
    // 鼠标事件的频率通常比 render 高，使用 HashMap 是为了避免 pick 结果有重复
    let mut list: HashMap<Entity, u64> = HashMap::new();

    for event in cursor_moved_events.read() {
        let (camera, transform) = cameras.get_single().unwrap();
        let ray = ray_from_screenspace(event.position, camera, transform).unwrap();
        let ray_cast = RayCast3d::from_ray(ray, 30.);
        // 计算射线拾取
        for (entity, volume, _) in query.iter_mut() {
            // 射线求交
            let toi = ray_cast.aabb_intersection_at(volume);

            // 刻意不在此时设置 hover，收集到所有 pick 到的 entity 发送给主线程，
            // 由主线程决定需要 hover 的对象后再发送回对应的 entity
            // status.hover = toi.is_some();

            if toi.is_some() {
                list.insert(entity, entity.to_bits());
            }
        }
    }

    if !list.is_empty() {
        // 通知 js pick 结果
        let js_array = js_sys::Array::new();
        for (_, &item) in list.iter() {
            js_array.push(&JsValue::from(item));
        }
        if app_info.is_in_worker {
            send_pick_from_worker(js_array);
        } else {
            send_pick_from_rust(js_array);
        }
    }

    // TODO: mouse wheel
    for _event in mouse_wheel_events.read() {}
}

/// 更新 选中/高亮
fn update_active(active_info: ResMut<ActiveInfo>, mut query: Query<(Entity, &mut ActiveState)>) {
    for (entity, mut status) in query.iter_mut() {
        status.hover = active_info.hover.contains_key(&entity);
        status.selected = active_info.selection.contains_key(&entity)
    }
}

/// 构造一条相机射线
fn ray_from_screenspace(
    cursor_pos_screen: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Ray3d> {
    let mut viewport_pos = cursor_pos_screen;
    if let Some(viewport) = &camera.viewport {
        viewport_pos -= viewport.physical_position.as_vec2();
    }
    camera
        .viewport_to_world(camera_transform, viewport_pos)
        .map(Ray3d::from)
}

fn screen_to_world(
    pixel_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec3> {
    let ray = ray_from_screenspace(pixel_pos, camera, camera_transform);
    if let Some(ray) = ray {
        // 射线与对像所有平面的交点
        let d = ray.intersect_plane(Vec3::new(0., 0., 2.), InfinitePlane3d::new(Vec3::Z));
        if let Some(d) = d {
            return Some(ray.origin + ray.direction * d);
        }
    }
    None
}
