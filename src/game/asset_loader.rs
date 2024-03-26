use bevy::prelude::*;

/// 각 주사위의 1~6 텍스쳐
/// 및 주사위 충돌 소리
#[derive(Resource, Debug, Default)]
pub struct DiceAssets {
  pub dice1_texture: Handle<Image>,
  pub dice2_texture: Handle<Image>,
  pub dice3_texture: Handle<Image>,
  pub dice4_texture: Handle<Image>,
  pub dice5_texture: Handle<Image>,
  pub dice6_texture: Handle<Image>,
  pub dice_hit: Handle<AudioSource>
}

#[derive(Resource, Debug, Default)]
pub struct DiceMaterialAssets {
    pub dice1_material: Handle<StandardMaterial>,
    pub dice2_material: Handle<StandardMaterial>,
    pub dice3_material: Handle<StandardMaterial>,
    pub dice4_material: Handle<StandardMaterial>,
    pub dice5_material: Handle<StandardMaterial>,
    pub dice6_material: Handle<StandardMaterial>,
}

#[derive(Resource, Debug, Default)]
pub struct DiceMeshAssets {
    pub dice_mesh: Handle<Mesh>,
}

/// 주사위 충돌 소리
#[derive(Resource, Deref, Debug)]
pub struct DiceHitMusic(pub Handle<AudioSource>);

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
  fn build(&self, app: &mut App) {
      app
        .init_resource::<DiceAssets>()
        .init_resource::<DiceMeshAssets>()
        .init_resource::<DiceMaterialAssets>()
        .add_systems(PreStartup, load_assets);
  }
}

/// Startup 전 PreStartup 스케쥴시점에 필요한 에셋들을 로드 한다.
fn load_assets(
    mut commands: Commands,
    mut dice_assets: ResMut<DiceAssets>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut dice_material_assets: ResMut<DiceMaterialAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut dice_mesh_assets: ResMut<DiceMeshAssets>
) {
    *dice_assets = DiceAssets { 
        dice1_texture: asset_server.load("texture/dice/dice1.png"),
        dice2_texture: asset_server.load("texture/dice/dice2.png"),
        dice3_texture: asset_server.load("texture/dice/dice3.png"),
        dice4_texture: asset_server.load("texture/dice/dice4.png"),
        dice5_texture: asset_server.load("texture/dice/dice5.png"),
        dice6_texture: asset_server.load("texture/dice/dice6.png"),
        dice_hit: asset_server.load("mp3/dice/hit.ogg"),
    };

    // 로드한 소리를 앱에 추가한다.
    commands.insert_resource(DiceHitMusic(dice_assets.dice_hit.clone()));


    let dice_1_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(dice_assets.dice1_texture.clone()),
        ..default()
    });
    let dice_2_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(dice_assets.dice2_texture.clone()),
        ..default()
    });
    let dice_3_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(dice_assets.dice3_texture.clone()),
        ..default()
    });
    let dice_4_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(dice_assets.dice4_texture.clone()),
        ..default()
    });
    let dice_5_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(dice_assets.dice5_texture.clone()),
        ..default()
    });
    let dice_6_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(dice_assets.dice6_texture.clone()),
        ..default()
    });

    // 주사위 메테리얼 
    *dice_material_assets = DiceMaterialAssets {
        dice1_material: dice_1_material_handle,
        dice2_material: dice_2_material_handle,
        dice3_material: dice_3_material_handle,
        dice4_material: dice_4_material_handle,
        dice5_material: dice_5_material_handle,
        dice6_material: dice_6_material_handle,
    };

    *dice_mesh_assets = DiceMeshAssets {
        dice_mesh: meshes.add(Plane3d::default())
    };
}