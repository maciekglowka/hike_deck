use bevy::prelude::*;
use std::collections::VecDeque;

use crate::actions::{
    ActionExecutedEvent,
    models::{MeleeHitAction, WalkAction}
};
use crate::board::components::Position;
use crate::pieces::components::Piece;
use super::{
    GraphicsAssets, TILE_SIZE, PIECE_Z, PIECE_SPEED, POSITION_TOLERANCE,
    components::PathAnimator
};

pub fn walk_animation(
    mut commands: Commands,
    mut ev_action: EventReader<ActionExecutedEvent>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>
) {
    for ev in ev_action.iter() {
        let action = ev.0.as_any();
        if let Some(action) = action.downcast_ref::<WalkAction>() {
            let target = super::get_world_vec(action.1, PIECE_Z);
            commands.entity(action.0)
                .insert(PathAnimator(VecDeque::from([target])));
            ev_wait.send(super::GraphicsWaitEvent);
        }
    }
}

pub fn melee_animation(
    mut commands: Commands,
    query: Query<&Position>,
    mut ev_action: EventReader<ActionExecutedEvent>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>
) {
    for ev in ev_action.iter() {
        let action = ev.0.as_any();
        if let Some(action) = action.downcast_ref::<MeleeHitAction>() {
            let Ok(base_position) = query.get(action.attacker) else { continue };
            let base = super::get_world_position(base_position, PIECE_Z);
            let target = 0.5 * (base + super::get_world_vec(action.target, PIECE_Z));
            commands.entity(action.attacker)
                .insert(PathAnimator(VecDeque::from([target, base])));
            ev_wait.send(super::GraphicsWaitEvent);
        }
    }
}

pub fn path_animator_update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PathAnimator, &mut Transform)>,
    time: Res<Time>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>
) {
    for (entity, mut animator, mut transform) in query.iter_mut() {
        if animator.0.len() == 0 {
            // this entity has completed it's animation
            commands.entity(entity).remove::<PathAnimator>();
            continue;
        }
        ev_wait.send(super::GraphicsWaitEvent);
        let target = *animator.0.get(0).unwrap();
        let d = (target - transform.translation).length();
        if d > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(
                target,
                PIECE_SPEED * time.delta_seconds()
            );
        } else {
            // the entity is at the desired path position
            transform.translation = target;
            animator.0.pop_front();
        }
    }
}

pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Piece), Added<Piece>>,
    assets: Res<GraphicsAssets>
) {
    for (entity, position, piece) in query.iter() {
        let sprite_idx = match piece.kind.as_str() {
            "Player" => 1,
            _ => 63
        };
        let mut sprite = TextureAtlasSprite::new(sprite_idx);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = Color::WHITE;
        let v = super::get_world_position(&position, PIECE_Z);
        commands.entity(entity)
            .insert(
                SpriteSheetBundle {
                    sprite,
                    texture_atlas: assets.sprite_texture.clone(),
                    transform: Transform::from_translation(v),
                    ..Default::default()
                }
            );
    }
}