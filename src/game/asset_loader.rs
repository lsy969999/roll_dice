use bevy::prelude::*;

/// 각 주사위의 1~6 텍스쳐
/// 및 주사위 충돌 소리
#[derive(Resource, Debug, Default)]
pub struct DiceAssets {
  pub dice_hit: Handle<AudioSource>,
  pub dice_model: Handle<Scene>,
}

/// 주사위 충돌 소리
#[derive(Resource, Deref, Debug)]
pub struct DiceHitMusic(pub Handle<AudioSource>);

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
  fn build(&self, app: &mut App) {
      app
        .init_resource::<DiceAssets>()
        .add_systems(PreStartup, load_assets);
  }
}

/// Startup 전 PreStartup 스케쥴시점에 필요한 에셋들을 로드 한다.
fn load_assets(
    mut commands: Commands,
    mut dice_assets: ResMut<DiceAssets>,
    asset_server: Res<AssetServer>,
) {
    *dice_assets = DiceAssets {
        dice_hit: asset_server.load("mp3/dice/hit.ogg"),
        dice_model: asset_server.load("models/dice/scene.gltf#Scene0")
    };

    // 로드한 소리를 앱에 추가한다.
    commands.insert_resource(DiceHitMusic(dice_assets.dice_hit.clone()));
}