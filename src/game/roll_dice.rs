use bevy::{prelude::*, window::{WindowCreated, WindowResized}};
use bevy_rapier3d::prelude::*;
use super::asset_loader::*;
use super::ui::*;
use super::wall::*;
use super::constant::*;
use super::dice::*;

/// 게임 설정  
/// mute: false면 소리 나고, true 이면 무음, 디폴트는 false
/// motion: false면 기기 가속도계 미사용, true면 가속도계 사용, 디폴트는 true
#[derive(Resource, Debug)]
pub struct GameConfig {
    pub mute: bool,
    pub motion: bool,
}

/// 카메라 3d
#[derive(Component)]
pub struct MyGameCamera;

/// 앱 빌드  
/// AssetLoaderPlugin: 주사위 소리 및 텍스쳐 로더  
/// RapierPhysicsPlugin: 레이피어 엔진 적용  
/// RapierDebugRenderPlugin: 레이피어 엔진 디버그용, 필요시 주석 해제  
/// UiPlugin: ui 레이아웃 플러그인  
/// DicePlugin: 주사위 플러그인  
/// WallPlugin: 벽 플러그인  
/// GameConfig: 게임 설정정보
/// setup_roll_dice: 롤다이스 앱을 셋업한다.
/// on_resize_system: PC에서 창 변경 이벤트 처리(모바일에서는 호출이 안되더라)
/// window_created: 윈도우가 만들어지면 해야하는 설정
pub fn build_app(app: &mut App) {
    app
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(UiPlugin)
        .add_plugins(DicePlugin)
        .add_plugins(WallPlugin)
        
        .insert_resource::<GameConfig>(GameConfig { mute: false, motion: true, })
        .add_systems(Startup, setup_roll_dice)
        .add_systems(Update, on_resize_system)
        .add_systems(Update, window_created);
}

fn setup_roll_dice(
  mut commands: Commands,
) {
    // 3D Camera
    commands
        .spawn(
            (
                Camera3dBundle {
                    transform: Transform::from_xyz(0., 70., 70.0).looking_at(Vec3::ZERO, Vec3::Y),
                    ..Default::default()
                },
                MyGameCamera
            )
        );

    // Light
    // 그림자를 활성화하면 리소스 사용량이 급증 해버린다.
    commands
        .spawn(
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    illuminance: 10000.0,
                    // shadows_enabled: true,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 200.0, 0.0),
                    rotation: Quat::from_rotation_x(-PI / 4.),
                    ..default()
                },
                ..default()
            }
        );
}


/// 윈도우 리사이즈 처리  
/// 주사위를 감싸고 있는 왼쪽벽과 오른쪽벽  
/// 윈도우 사이즈가 변경되더라도 항상 보일수 있도록 조정하기 위함이다.  
/// PC 에서만 호출되는걸 확인  
/// Mobile 에서는 호출이 안되서 wincreated 이벤트에서 처리하기로 한다.  
fn on_resize_system(
    q_camera: Query<(&Camera, &GlobalTransform), With<MyGameCamera>>,
    mut er_win_resize: EventReader<WindowResized>,
    mut q_walls: Query<(&mut Transform, &Wall)>,
) {
    for e in er_win_resize.read() {
        debug!("resize {} x {}", e.width, e.height);
        let (camera, camera_transform) = q_camera.single();

        // x=0 인 지점은 창의 가장 왼쪽
        // x=e.width 인 지점은 창의 가장 오른쪽
        // 각점에서 90 거리에 위치하는 정점을 구한다.
        let resized_bottom_x1 = camera.viewport_to_world(camera_transform, Vec2::new(0., 0.)).unwrap().get_point(70.);
        let resized_bottom_x2 = camera.viewport_to_world(camera_transform, Vec2::new(e.width, 0.)).unwrap().get_point(70.);

        debug!("resize block width1{:?}",resized_bottom_x1);
        debug!("resize block width2{:?}", resized_bottom_x2);
        
        for (mut tr, wall) in q_walls.iter_mut() {
            if wall.wall_type == WallType::Left {
                debug!("reisize left wall");
                // 왼쪽 벽의 x 값을 위에서 구한 x 값으로 세팅한다.
                tr.translation.x = resized_bottom_x1.x;
            }
            if wall.wall_type == WallType::Right {
                debug!("reisize right wall");
                // 오른쪽 벽의 x 값을 위에서 구한 x 값으로 세팅한다.
                tr.translation.x = resized_bottom_x2.x;
            }
        }
    }
}

/// 왼도우 생성시 처리
/// 윈도우가 생성되면  
/// 윈도우 가로 길이에 맞춰서  
/// 왼쪽벽과 오른쪽 벽의 위치를 조정  
/// 아이폰같은경우 스케일팩터를 고려해서 계산  
/// PC, Mobile 둘다 호출된다.
fn window_created(
    mut er_win_creat: EventReader<WindowCreated>,
    q_window: Query<&mut Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MyGameCamera>>,
    mut q_walls: Query<(&mut Transform, &Wall)>,
){
    for w in er_win_creat.read() {
        if let Ok(win) = q_window.get(w.window) {
            info!("windowCreated {:?}", win);
            let (camera, camera_transform) = q_camera.single();
            // 기기의 가로를 구한다. 아이폰은 스케일팩터가 3인경우도 있다.
            let width = win.resolution.physical_width() as f32 / win.resolution.scale_factor();
            // 70 거리에서의 정점을구한다.
            let resize_width = camera.viewport_to_world(camera_transform, Vec2::new(width, 0.)).unwrap().get_point(70.);
            info!("windowCreated resize_width {:?}", resize_width.x);
            for (mut tr, wall) in q_walls.iter_mut() {
                if wall.wall_type == WallType::Left {
                    debug!("reisize left wall");
                    tr.translation.x = resize_width.x * -1.;
                }
                if wall.wall_type == WallType::Right {
                    debug!("reisize right wall");
                    tr.translation.x = resize_width.x;
                }
            }
        };
    }
}
