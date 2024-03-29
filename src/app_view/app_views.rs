use bevy::{utils::HashMap, ecs::entity::Entity};

#[cfg(target_os = "ios")]
pub use super::ios::*;

#[cfg(target_os = "android")]
pub use super::android::*;

use super::WindowId;

#[derive(Debug, Default)]
pub struct AppViews {
  views: HashMap<super::WindowId, AppView>,
  entitiy_to_window_id: HashMap<Entity, WindowId>,
}

impl AppViews {
    pub fn create_window(
      &mut self,
      #[cfg(target_os = "ios")]
      view_obj: super::ios::IOSViewObj,
      #[cfg(target_os = "android")]
      view_obj: super::android::AndroidViewObj,
      entity: Entity
    ) -> &AppView {
      let app_view = AppView::new(view_obj);
      let window_id = WindowId::new();
      self.entitiy_to_window_id.insert(entity, window_id);

      self.views.entry(window_id).insert(app_view).into_mut()
    }

    /// Get the AppView that associated with our entity.
    pub fn get_view(&self, entity: Entity) -> Option<&AppView> {
      self.entitiy_to_window_id
        .get(&entity)
        .and_then(|window_id| self.views.get(window_id))
    }

    /// This should mostly just be called when the window is closing.
    pub fn remove_view(&mut self, entity: Entity) -> Option<AppView> {
      let window_id = self.entitiy_to_window_id.remove(&entity)?;
      self.views.remove(&window_id)
    }
}