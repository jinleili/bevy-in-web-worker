use super::{canvas::*, CanvasViews};
use bevy::app::{App, Plugin};
use bevy::ecs::{
    entity::Entity,
    event::EventWriter,
    prelude::*,
    system::{Commands, NonSendMut, Query, SystemState},
};
use bevy::window::{exit_on_all_closed, RawHandleWrapper, Window, WindowClosed, WindowCreated};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

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
        EventWriter<WindowCreated>,
        NonSendMut<CanvasViews>,
    )> = SystemState::from_world(app.world_mut());
    let (mut commands, mut new_windows, mut created_window_events, mut canvas_views) =
        create_window_system_state.get_mut(app.world_mut());

    for (entity, mut window) in new_windows.iter_mut() {
        if canvas_views.get_view(entity).is_some() {
            continue;
        }

        let app_view = canvas_views.create_window(view_obj, entity);
        let (logical_res, scale_factor) = match app_view {
            ViewObj::Canvas(canvas) => (canvas.logical_resolution(), canvas.scale_factor),
            ViewObj::Offscreen(offscreen) => {
                (offscreen.logical_resolution(), offscreen.scale_factor)
            }
        };

        // Update resolution of bevy window
        window.resolution.set_scale_factor(scale_factor as f32);
        window.resolution.set(logical_res.0, logical_res.1);

        let (window_handle, display_handle) = match app_view {
            ViewObj::Canvas(app_view) => (
                app_view.window_handle().unwrap().as_raw(),
                app_view.display_handle().unwrap().as_raw(),
            ),
            ViewObj::Offscreen(app_view) => (
                app_view.window_handle().unwrap().as_raw(),
                app_view.display_handle().unwrap().as_raw(),
            ),
        };

        commands.entity(entity).insert(RawHandleWrapper {
            window_handle,
            display_handle,
        });

        created_window_events.send(WindowCreated { window: entity });
        break;
    }
    create_window_system_state.apply(app.world_mut());
}

pub(crate) fn despawn_window(
    mut closed: RemovedComponents<Window>,
    window_entities: Query<&Window>,
    mut close_events: EventWriter<WindowClosed>,
    mut app_views: NonSendMut<CanvasViews>,
) {
    for entity in closed.read() {
        crate::web_ffi::log("Closing window {:?entity}");
        if !window_entities.contains(entity) {
            app_views.remove_view(entity);
            close_events.send(WindowClosed { window: entity });
        }
    }
}

pub(crate) fn changed_window(
    mut _changed_windows: Query<(Entity, &mut Window), Changed<Window>>,
    _app_views: NonSendMut<CanvasViews>,
) {
    // TODO:
}
