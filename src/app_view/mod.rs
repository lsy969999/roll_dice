use bevy::{prelude::*, window::{WindowClosed, exit_on_all_closed, WindowCreated, RawHandleWrapper}, ecs::system::SystemState};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use uuid::Uuid;
use bevy::log::info;
use app_views::*;

pub mod app_views;
pub mod ios;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub(crate) struct WindowId(Uuid);
impl WindowId {
    pub fn new() -> Self {
      WindowId(Uuid::new_v4())
    }
}

pub struct AppViewPlugin;

impl Plugin for AppViewPlugin  {
  fn build(&self, app: &mut App) {
    app.init_non_send_resource::<AppViews>()
    .add_systems(
      bevy::app::Last,
      (
        changed_window.ambiguous_with(exit_on_all_closed),
        despawn_window.after(changed_window),
      )
    );
  }
}

pub fn create_bevy_window(app: &mut App) {
  #[cfg(target_os = "ios")]
  let view_obj = app.world.remove_non_send_resource::<IOSViewObj>().unwrap();

  let mut create_window_system_state: SystemState<(
    Commands,
    Query<(Entity, &mut Window), Added<Window>>,
    EventWriter<WindowCreated>,
    NonSendMut<AppViews>,
  )> = SystemState::from_world(&mut app.world);

  let (
    mut commands,
    mut new_windows,
    mut created_window_writer,
    mut app_views
  ) = create_window_system_state.get_mut(&mut app.world);

  for (entity, mut bevy_window) in new_windows.iter_mut() {
    if app_views.get_view(entity).is_some() {
      continue;
    }
    let app_view = app_views.create_window(view_obj, entity);
    let logical_res = app_view.logical_resolution();
    bevy_window
    .resolution
    .set_scale_factor_override(Some(app_view.scale_factor));
    bevy_window
      .resolution
      .set_scale_factor(app_view.scale_factor);

    bevy_window
      .resolution
      .set(logical_res.0, logical_res.1);

    info!("bevy_window: {:?}", bevy_window);
    info!("bevy_window resolution: {:?}", bevy_window.resolution);

    commands
      .entity(entity)
      .insert(
        RawHandleWrapper {
          window_handle: app_view.window_handle().unwrap().as_raw(),
          display_handle: app_view.display_handle().unwrap().as_raw(),
        }
      );
    
    created_window_writer.send(
      WindowCreated {
        window: entity,
      }
    );
  }

  create_window_system_state.apply(&mut app.world);
}

pub(crate) fn despawn_window(
  mut closed: RemovedComponents<Window>,
  window_entities: Query<&Window>,
  mut close_events: EventWriter<WindowClosed>,
  mut app_views: NonSendMut<AppViews>,
) {
  for entity in closed.read() {
    info!("Closing window {:?}", entity);
    if !window_entities.contains(entity) {
      app_views.remove_view(entity);
      close_events.send(WindowClosed { window: entity });
    }
  }
}

pub(crate) fn changed_window(
  mut _changed_windows: Query<(Entity, &mut Window), Changed<Window>>
) {
  // TODO
}