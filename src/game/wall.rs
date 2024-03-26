use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::constant::*;

/// 벽타입, 정사각형 6면 생각하면 된다.
#[derive(Debug, PartialEq)]
pub enum WallType {
    Top,
    Bottom,
    Left,
    Right,
    Back,
    Front
}

/// 벽컴포넌트
#[derive(Component, Debug)]
pub struct Wall {
    pub wall_type: WallType
}

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, gen_walls);
    }
}

/// 벽을 생성한다.
fn gen_walls(mut commands: Commands,) {
    // botoom wall
    commands
    .spawn((Collider::cuboid(400.0, 0.5, 400.0), Wall { wall_type: WallType::Bottom }))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

    // top wall
    commands
    .spawn((Collider::cuboid(400.0, 0.5, 400.0), Wall { wall_type: WallType::Top }))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 30.0, 0.0)));

    // right wall
    commands
    .spawn((Collider::cuboid(15.0, 0.5, 30.0), Wall { wall_type: WallType::Right }))
    .insert(TransformBundle::from(Transform {
        translation: Vec3::new(17.8, 15., 0.),
        scale: Vec3::new(1., 1., 1.),
        rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., PI / 2.)
    }));

    // left wall
    commands
    .spawn((Collider::cuboid(15.0, 0.5, 30.0), Wall { wall_type: WallType::Left }))
    .insert(TransformBundle::from(Transform {
        translation: Vec3::new(-17.8, 15., 0.),
        scale: Vec3::new(1., 1., 1.),
        rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., PI / 2.)
    }));

    // back wall
    commands
    .spawn((Collider::cuboid(300.0, 0.5, 15.0), Wall { wall_type: WallType::Back }))
    .insert(TransformBundle::from(Transform {
        translation: Vec3::new(0., 15., -30.),
        scale: Vec3::new(1., 1., 1.),
        rotation: Quat::from_euler(EulerRot::XYZ, PI / 2., 0.,0. )
    }));

    // front wall
    commands
    .spawn((Collider::cuboid(300.0, 0.5, 15.0), Wall { wall_type: WallType::Front }))
    .insert(TransformBundle::from(Transform {
        translation: Vec3::new(0., 15., 30.),
        scale: Vec3::new(1., 1., 1.),
        rotation: Quat::from_euler(EulerRot::XYZ, PI / 2., 0.,0. )
    }));
}