use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use super::asset_loader::*;
use super::constant::*;
use super::roll_dice::*;
/**
 * TODO
 * 오디오의 DESPAWN, REMOVE 가 뭔지?
 */

/// 기기의 가속도계의값을 담을 구조체
#[derive(Event, Debug)]
pub struct AccelerometerEvent {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/// 주사위 마커컴포넌트
#[derive(Component)]
pub struct Dice;

/// 주사위 숫자 타입
#[derive(PartialEq, Debug)]
pub enum DiceNumType {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

/// 주사위 숫자 컴포넌트
#[derive(Component, Debug)]
pub struct DiceNum {
    pub dice_num_type: DiceNumType
}

/// 주사위 모델
#[derive(Component, Debug)]
pub struct DiceModel;

pub struct DicePlugin;

impl Plugin for DicePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_dice)
            .add_systems(Update, dice_click_system)
            .add_systems(Update, accelerometer_detect_system)
            .add_systems(Update, dice_collision_detect_and_audio_play)
            .add_event::<AccelerometerEvent>();
    }
}

/// Startup 스케쥴시점에 주사위 하나를 추가한다.
fn setup_dice(
    mut commands: Commands,
    mut dice_assets: ResMut<DiceAssets>,
){
    gen_dice(&mut commands, &mut dice_assets);
}

/// 주사위를 생성한다.
/// 생성시마다 에셋로더로부터 각 텍스쳐를 클론해서 사용한다.
/// 주사위 생성시 생성 Pos 와 Rot 값은 랜덤이다.
pub fn gen_dice(
    commands: &mut Commands,
    dice_assets: &mut ResMut<DiceAssets>,
) {
    let mut rng = rand::thread_rng();
    // 주사위의 생성 포지션 위치 값
    let rand_dice_x = rng.gen_range(-10.0..=10.0) as f32;
    let rand_dice_z = rng.gen_range(-10.0..=10.0) as f32;

    // 주사위의 생성시 회전 값
    let rand_dice_rot_x = rng.gen_range(0.0..=2. * PI) as f32;
    let rand_dice_rot_y = rng.gen_range(0.0..=2. * PI) as f32;
    let rand_dice_rot_z = rng.gen_range(0.0..=2. * PI) as f32;
    
    commands
        .spawn((RigidBody::Dynamic, Dice, SpatialBundle::default() ))
        .insert(Collider::cuboid(2., 2., 2.))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(GravityScale(2.0))
        .insert(
            TransformBundle::from(Transform {
                translation: Vec3::new(rand_dice_x, 10.0, rand_dice_z),
                rotation: Quat::from_euler(EulerRot::XYZ, rand_dice_rot_x, rand_dice_rot_y, rand_dice_rot_z),
                ..default()
            })
        )
        .insert(ExternalImpulse::default())
        .with_children(|parent| {
            parent
                .spawn(SceneBundle {
                    scene: dice_assets.dice_model.clone(),
                    transform: Transform {
                        scale: Vec3::new(180., 180., 180.),
                        
                        ..default()
                    },
                    ..default()
                })
                .insert(DiceModel)
                .with_children(|parent| {
                    parent
                        .spawn((SpatialBundle::from_transform(Transform {
                            translation: Vec3::new(0., -2., 0.),
                            rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., PI),
                            scale: Vec3::new(2., 2., 2.)
                        }), DiceNum{ dice_num_type: DiceNumType::One } ));
                    parent
                        .spawn((SpatialBundle::from_transform(Transform {
                            translation: Vec3::new(0., 2., 0.),
                            rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
                            scale: Vec3::new(2., 2., 2.)
                        }), DiceNum{ dice_num_type: DiceNumType::Six } ));
                    parent
                        .spawn((SpatialBundle::from(Transform {
                            translation: Vec3::new(-2., 0., 0.),
                            rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., PI / 2.),
                            scale: Vec3::new(2., 2., 2.)
                        }), DiceNum{ dice_num_type: DiceNumType::Three } ));
                    parent
                        .spawn((SpatialBundle::from_transform(Transform {
                            translation: Vec3::new(2., 0., 0.),
                            rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., -PI / 2.),
                            scale: Vec3::new(2., 2., 2.)
                        }), DiceNum{ dice_num_type: DiceNumType::Four } ));

                    parent
                        .spawn((SpatialBundle::from_transform(Transform {
                            translation: Vec3::new(0., 0., -2.),
                            rotation: Quat::from_euler(EulerRot::XYZ, -PI / 2., 0., 0.),
                            scale: Vec3::new(2., 2., 2.)
                        }), DiceNum{ dice_num_type: DiceNumType::Five } ));
                    parent
                        .spawn((SpatialBundle::from_transform(Transform {
                            translation: Vec3::new(0., 0., 2.),
                            rotation: Quat::from_euler(EulerRot::XYZ, PI / 2., 0., 0.),
                            scale: Vec3::new(2., 2., 2.)
                        }), DiceNum{ dice_num_type: DiceNumType::Two } ));
                    });
        });
}

pub fn impulse_dice(ext_impulse: Mut<'_, ExternalImpulse>) {
    let mut rng = rand::thread_rng();
    let random_impulse_x = rng.gen_range(-100..=100) as f32;
    let random_impulse_y = rng.gen_range(1000..=1300) as f32;
    let random_impulse_z = rng.gen_range(-100..=100) as f32;
    let random_torque_impulse_x = rng.gen_range(-1500..=1500) as f32;
    let random_torque_impulse_y = rng.gen_range(-1500..=1500) as f32;
    let random_torque_impulse_z = rng.gen_range(-1500..=1500) as f32;
    impulse_dice_common(
        ext_impulse,
        random_impulse_x,
        random_impulse_y,
        random_impulse_z,
        random_torque_impulse_x,
        random_torque_impulse_y,
        random_torque_impulse_z
    )
}

/// 가속도계로부터 값을 읽어온경우의 주사위에가할 impulse
pub fn impulse_dice_accelerometer(
    ext_impulse: Mut<'_, ExternalImpulse>,
    impulse_x: f32,
    impulse_y: f32,
    impulse_z: f32,
) {
    let mut rng = rand::thread_rng();
    let random_torque_impulse_x = rng.gen_range(-100..=100) as f32;
    let random_torque_impulse_y = rng.gen_range(-100..=100) as f32;
    let random_torque_impulse_z = rng.gen_range(-100..=100) as f32;
    impulse_dice_common(
        ext_impulse,
        impulse_x,
        impulse_y,
        impulse_z,
        random_torque_impulse_x,
        random_torque_impulse_y,
        random_torque_impulse_z
    )
}

/// impulse_dice, impulse_dice_accelerometer 가 사용하는  
/// impuse 값 결정
fn impulse_dice_common(
    mut ext_impulse: Mut<'_, ExternalImpulse>,
    impulse_x: f32,
    impulse_y: f32,
    impulse_z: f32,
    torque_impulse_x: f32,
    torque_impulse_y: f32,
    torque_impulse_z: f32,
) {
    debug!(
        "random_impulse_x: {} random_impulse_y: {} random_impulse_z: {}
        random_number_x: {} random_number_y: {} random_number_z: {}",
        impulse_x, impulse_y, impulse_z, torque_impulse_x, torque_impulse_y, torque_impulse_z
    );
    ext_impulse.impulse = Vec3::new(impulse_x, impulse_y, impulse_z);
    ext_impulse.torque_impulse = Vec3::new(torque_impulse_x, torque_impulse_y, torque_impulse_z);
}

/// 주사위 충돌 감지 및 오디오 출력  
/// 충돌 시작과 종료를 캐치할수 있는데  
/// 충돌시점에만 오디오 출력을해준다. 종료시점에까지하면 너무 많다.  
fn dice_collision_detect_and_audio_play(
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<DiceHitMusic>,
    r_game_config: Res<GameConfig>,
    mut commands: Commands,
) {
    // 게임 설정값의 뮤트가 비활성화된경우만
    // 소리 출력을 진행한다.
    if !r_game_config.mute {
        for collision_event in collision_events.read() {
            match collision_event {
                CollisionEvent::Started(_entity1, _entity2, _flags) => {
                    // info!(
                    //     "Received collision event: Started entity1: {:?}, entity2: {:?}, flags: {:?}",
                    //     entity1, entity2, flags
                    // );
                    commands.spawn(AudioBundle {
                        source: sound.clone(),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            ..default()
                        },
                        ..default()
                    });
                }
                CollisionEvent::Stopped(_entity1, _entity2, _flags) => {
                    // info!(
                    //     "Received collision event: Stopped entity1: {:?}, entity2: {:?}, flags: {:?}",
                    //     entity1, entity2, flags
                    // );
                    // commands.spawn(AudioBundle {
                    //     source: sound.clone(),
                    //     settings: PlaybackSettings {
                    //         mode: PlaybackMode::Despawn,
                    //         ..default()
                    //     },
                    //     ..default()
                    // });
                }
            }
        }
    }
}

/// 가속도 측정 시스템
fn accelerometer_detect_system(
    mut acc: EventReader<AccelerometerEvent>,
    mut ext_impulses: Query<&mut ExternalImpulse>,
    r_game_config: Res<GameConfig>,
) {
    if r_game_config.motion {
        for a in acc.read() {
            if a.x.abs() > 1.3
            || a.y.abs() > 1.3
            || a.z.abs() > 1.3 {
                // info!("!accelerometer_detect_system, {:?}", a);
                for ext_impulse in ext_impulses.iter_mut() {
                    impulse_dice_accelerometer(ext_impulse, a.x * 300., a.y * 300., a.z * 300.)
                }
            }
        }
    }
}

/// 주사위 클릭 시스템
fn dice_click_system(
    q_camera: Query<(&Camera, &GlobalTransform), With<MyGameCamera>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
    mut ext_impulses: Query<&mut ExternalImpulse>,
    mouse_button_input: ResMut<'_, ButtonInput<MouseButton>>,
    touches: Res<Touches>,
) {
    // info!("click cnt {:?}", touches.iter().count());
    // Mobile 터치시 작동
    for finger in touches.iter() {
        if touches.just_pressed(finger.id()) {
            // debug!("A new touch with ID {} just began.", finger.id());
            debug!(
                "Finger {} is at position ({},{}), started from ({},{}).",
                finger.id(),
                finger.position().x,
                finger.position().y,
                finger.start_position().x,
                finger.start_position().y,
            );
            
            let (camera, camera_transform) = q_camera.single();
            let curosr_position = Vec2::new(finger.position().x, finger.position().y);
    
            let Some(ray) = camera.viewport_to_world(camera_transform, curosr_position) else {
                return;
            };
            
            let ray_pos = ray.origin;
            let ray_dir = Vec3::new(ray.direction.x, ray.direction.y, ray.direction.z);
            let max_toi = 10000.;
            let solid = true;
            let filter = QueryFilter::default();
            
            rapier_context.intersections_with_ray(
            ray_pos, ray_dir, max_toi, solid, filter,
            |entity, _intersection| {
                if let Ok(ext_impulse) = ext_impulses.get_mut(entity) {
                    impulse_dice(ext_impulse);
                }
                true
            });
        }
    }

    // PC 마우스 왼쪽 버튼 클릭시 작동
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();
        let Some(curosr_position) = window.cursor_position() else {
        return;
        };
    
        let Some(ray) = camera.viewport_to_world(camera_transform, curosr_position) else {
        return;
        };
        debug!("cursor_position; {}, ray: {:?} wtv: {:?}", curosr_position, ray, camera.world_to_viewport(camera_transform, Vec3::new(60., 0., 0.)));
        
        let ray_pos = ray.origin;
        let ray_dir = Vec3::new(ray.direction.x, ray.direction.y, ray.direction.z);
        let max_toi = 10000.;
        let solid = true;
        let filter = QueryFilter::default();
        
        rapier_context.intersections_with_ray(
        ray_pos, ray_dir, max_toi, solid, filter,
        |entity, _intersection| {
            if let Ok(ext_impulse) = ext_impulses.get_mut(entity) {
                impulse_dice(ext_impulse);
            }
            true
        });
    }
}