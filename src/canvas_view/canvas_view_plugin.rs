use crate::{canvas::*, CanvasViews};
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
        app.init_non_send_resource::<CanvasViews>();
    }
}

#[allow(clippy::type_complexity)]
pub fn create_canvas_window(app: &mut App) {
    let canvas = app
        .world_mut()
        .remove_non_send_resource::<Canvas>()
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

        let app_view = canvas_views.create_window(canvas, entity);
        let logical_res = app_view.logical_resolution();

        // Update resolution of bevy window
        window
            .resolution
            .set_scale_factor(app_view.scale_factor as f32);
        window.resolution.set(logical_res.0, logical_res.1);

        commands.entity(entity).insert(RawHandleWrapper {
            window_handle: app_view.window_handle().unwrap().as_raw(),
            display_handle: app_view.display_handle().unwrap().as_raw(),
        });

        created_window_events.send(WindowCreated { window: entity });
        break;
    }
    create_window_system_state.apply(app.world_mut());
}
