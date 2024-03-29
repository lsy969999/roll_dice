use bevy::{diagnostic::DiagnosticsPlugin, log::LogPlugin, prelude::*};
use game::roll_dice::build_app;

#[cfg(any(target_os = "android", target_os = "ios"))]
mod app_view;
#[cfg(any(target_os = "android", target_os = "ios"))]
mod ffi;

#[cfg(target_os = "android")]
mod android_asset_io;

mod game;

pub fn create_roll_dice_app(
    #[cfg(target_os = "android")]
    android_asset_manager: android_asset_io::AndroidAssetManager
) -> App {
    let mut bevy_app = App::new();
    
    #[cfg(debug_assertions)]
    let mut default_plugins = DefaultPlugins
        .set(LogPlugin{
            filter: "info,wgpu_core=warn,wgpu_hal=warn,roll_dice=debug".into(),
            update_subscriber: None,
            level: bevy::log::Level::DEBUG
        })
        .build();

    
    #[cfg(not(debug_assertions))]
    // release 시점에는 Log, Diagnostics 플러그인을 해제, 메모리증가율때문
    let mut default_plugins = DefaultPlugins
        .build()
        .disable::<LogPlugin>()
        .disable::<DiagnosticsPlugin>();

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
      use bevy::winit::WinitPlugin;
      default_plugins = default_plugins
        .disable::<WinitPlugin>()
        .set(WindowPlugin::default());
    }

    #[cfg(target_os = "android")]
    {
        bevy_app.insert_non_send_resource(android_asset_manager);

        use bevy::render::{
            settings::{RenderCreation, WgpuSettings},
            RenderPlugin,
        };
        default_plugins = default_plugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(wgpu::Backends::VULKAN), //wgpu::Backends::VULKAN
                ..default()
            }),
            synchronous_pipeline_compilation: false, // 13 new ?
        });

        // the custom asset io plugin must be inserted in-between the
        // `CorePlugin' and `AssetPlugin`. It needs to be after the
        // CorePlugin, so that the IO task pool has already been constructed.
        // And it must be before the `AssetPlugin` so that the asset plugin
        // doesn't create another instance of an asset server. In general,
        // the AssetPlugin should still run so that other aspects of the
        // asset system are initialized correctly.
        //
        // 2023/11/04, Bevy v0.12:
        // In the Android, Bevy's AssetPlugin relies on winit, which we are not using.
        // If a custom AssetPlugin plugin is not provided,  it will crash at runtime:
        // thread '<unnamed>' panicked at 'Bevy must be setup with the #[bevy_main] macro on Android'
        default_plugins = default_plugins
            .add_before::<bevy::asset::AssetPlugin, _>(android_asset_io::AndroidAssetIoPlugin);
    }

    bevy_app
        .insert_resource(ClearColor(Color::rgb(0.96, 0.87, 0.7)))
        .add_plugins(default_plugins);

    #[cfg(any(target_os = "android", target_os = "ios"))]
    bevy_app.add_plugins(app_view::AppViewPlugin);

    build_app(&mut bevy_app);

        // In this scenario, need to call the setup() of the plugins that have been registered
  // in the App manually.
  // https://github.com/bevyengine/bevy/issues/7576
  // bevy 0.11 changed: https://github.com/bevyengine/bevy/pull/8336
  #[cfg(any(target_os = "android", target_os = "ios"))]
  {
    use bevy::app::PluginsState;
    while bevy_app.plugins_state() != PluginsState::Ready {
        bevy::tasks::tick_global_task_pools_on_main_thread();
    }
    bevy_app.finish();
    bevy_app.cleanup();
    bevy_app.update();
  }

    bevy_app
}

#[cfg(any(target_os = "android", target_os = "ios"))]
pub(crate) fn close_bevy_window(mut app: Box<App>) {
  use bevy::{app::AppExit, ecs::system::SystemState};
  let mut windows_state: SystemState<(
    Commands,
    Query<(Entity, &mut Window)>,
    EventWriter<AppExit>
  )> = SystemState::from_world(&mut app.world);
  let (
    mut commands,
    windows,
    mut app_exit_events
  ) = windows_state.get_mut(&mut app.world);
  for (window, _focus) in windows.iter() {
    commands.entity(window).despawn();
  }
  app_exit_events.send(AppExit);
  windows_state.apply(&mut app.world);

  app.update();
}