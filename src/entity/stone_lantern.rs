use super::{breakable::Breakable, gold::spawn_gold, EntityDepth};
use crate::{
    asset::GameAssets, audio::play_se, config::GameConfig, constant::*, states::GameState,
};
use bevy::{core::FrameCount, prelude::*, sprite::Anchor};
use bevy_aseprite_ultra::prelude::*;
use bevy_kira_audio::Audio;
use bevy_light_2d::light::{PointLight2d, PointLight2dBundle};
use bevy_rapier2d::prelude::*;
use rand::random;

#[derive(Default, Component, Reflect)]
struct StoneLantern;

#[derive(Component, Reflect)]
struct LanternParent(Entity);

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_stone_lantern(commands: &mut Commands, assets: &Res<GameAssets>, x: f32, y: f32) {
    let tx = x;
    let ty = y;

    let entity = commands.spawn_empty().id();

    commands.entity(entity).insert((
        Name::new("stone_lantern"),
        StateScoped(GameState::InGame),
        Breakable { life: 50 },
        StoneLantern,
        EntityDepth,
        AsepriteAnimationBundle {
            aseprite: assets.stone_lantern.clone(),
            transform: Transform::from_translation(Vec3::new(tx, ty, 0.0)),
            sprite: Sprite {
                anchor: Anchor::Custom(Vec2::new(0.0, -0.25)),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(8.0, 8.0),
        CollisionGroups::new(WALL_GROUP, ACTOR_GROUP | BULLET_GROUP),
    ));

    commands.spawn((
        LanternParent(entity),
        PointLight2dBundle {
            point_light: PointLight2d {
                radius: 64.0,
                intensity: 1.0,
                falloff: 10.0,
                color: Color::hsl(42.0, 1.0, 0.71),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(tx, ty, 0.0)),
            ..default()
        },
    ));
}

fn update_lantern(
    mut commands: Commands,
    parent_query: Query<Entity, With<StoneLantern>>,
    mut child_query: Query<(Entity, &LanternParent, &mut PointLight2d)>,
    frame_count: Res<FrameCount>,
) {
    // info!("update_lantern");
    for (entity, child, mut light) in child_query.iter_mut() {
        light.intensity = 1.0 + ((frame_count.0 as f32 * 0.5).cos()) * 0.1;

        if !parent_query.contains(child.0) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn break_stone_lantern(
    mut commands: Commands,
    query: Query<(Entity, &Breakable), With<StoneLantern>>,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    for (entity, breakabke) in query.iter() {
        if breakabke.life <= 0 {
            commands.entity(entity).despawn_recursive();
            play_se(&audio, &config, assets.kuzureru.clone());
        }
    }
}

pub struct StoneLanternPlugin;

impl Plugin for StoneLanternPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_lantern.run_if(in_state(GameState::InGame)));
        app.add_systems(
            FixedUpdate,
            break_stone_lantern.run_if(in_state(GameState::InGame)),
        );
        app.register_type::<StoneLantern>();
    }
}