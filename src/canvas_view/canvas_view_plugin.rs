use super::*;
use bevy::app::{App, Plugin};
use bevy::ecs::{
    entity::Entity,
    message::MessageWriter,
    prelude::*,
    system::{Commands, NonSendMut, Query, SystemState},
};
use bevy::window::{RawHandleWrapper, Window, WindowClosed, WindowCreated, exit_on_all_closed};

pub struct CanvasViewPlugin;

impl Plugin for CanvasViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<CanvasViews>().add_systems(
            bevy::app::Last,
            (
                changed_window.ambiguous_with(exit_on_all_closed),
                // Update the state of the window before attempting to despawn to ensure consistent event ordering
                despawn_window.after(changed_window),
            ),
        );
    }
}

#[allow(clippy::type_complexity)]
pub fn create_canvas_window(app: &mut App) {
    let view_obj = app
        .world_mut()
        .remove_non_send_resource::<ViewObj>()
        .unwrap();

    let mut create_window_system_state: SystemState<(
        Commands,
        Query<(Entity, &mut Window), Added<Window>>,
        MessageWriter<WindowCreated>,
        NonSendMut<CanvasViews>,
    )> = SystemState::from_world(app.world_mut());
    let (mut commands, mut new_windows, mut created_window_events, mut canvas_views) =
        create_window_system_state.get_mut(app.world_mut());

    for (entity, mut window) in new_windows.iter_mut() {
        if canvas_views.get_view(entity).is_some() {
            continue;
        }

        let app_view = canvas_views.create_window(view_obj, entity);
        let (logical_res, _scale_factor) = match app_view {
            ViewObj::Canvas(canvas) => (canvas.physical_resolution(), canvas.scale_factor),
            ViewObj::Offscreen(offscreen) => {
                (offscreen.physical_resolution(), offscreen.scale_factor)
            }
        };

        // Update resolution of bevy window
        // I think scale is already handled in index.js by devicePixelRatio
        window.resolution.set_scale_factor(1.0);
        window
            .resolution
            .set(logical_res.0 as f32, logical_res.1 as f32);

        let raw_window_wrapper = match app_view {
            ViewObj::Canvas(window_wrapper) => RawHandleWrapper::new(window_wrapper),
            ViewObj::Offscreen(window_wrapper) => RawHandleWrapper::new(window_wrapper),
        };

        commands.entity(entity).insert(raw_window_wrapper.unwrap());

        created_window_events.write(WindowCreated { window: entity });
        break;
    }
    create_window_system_state.apply(app.world_mut());
}

pub(crate) fn despawn_window(
    mut closed: RemovedComponents<Window>,
    window_entities: Query<&Window>,
    mut close_events: MessageWriter<WindowClosed>,
    mut app_views: NonSendMut<CanvasViews>,
) {
    for entity in closed.read() {
        crate::web_ffi::log("Closing window {:?entity}");
        if !window_entities.contains(entity) {
            app_views.remove_view(entity);
            close_events.write(WindowClosed { window: entity });
        }
    }
}

pub(crate) fn changed_window(
    mut _changed_windows: Query<(Entity, &mut Window), Changed<Window>>,
    _app_views: NonSendMut<CanvasViews>,
) {
    // TODO:
}
