use super::canvas::*;
use bevy::ecs::entity::Entity;
use bevy::utils::HashMap;

#[derive(Debug, Default)]
pub struct CanvasViews {
    views: HashMap<super::WindowId, Canvas>,
    entity_to_window_id: HashMap<Entity, super::WindowId>,
}

impl CanvasViews {
    pub fn create_window(&mut self, app_view: Canvas, entity: Entity) -> &Canvas {
        let window_id = super::WindowId::new();
        self.entity_to_window_id.insert(entity, window_id);

        self.views.entry(window_id).insert(app_view).into_mut()
    }

    /// 与 entity 关联的 Canvas.
    pub fn get_view(&self, entity: Entity) -> Option<&Canvas> {
        self.entity_to_window_id
            .get(&entity)
            .and_then(|window_id| self.views.get(window_id))
    }

    pub fn remove_view(&mut self, entity: Entity) -> Option<Canvas> {
        let window_id = self.entity_to_window_id.remove(&entity)?;
        self.views.remove(&window_id)
    }
}
