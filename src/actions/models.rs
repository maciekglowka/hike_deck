use bevy::prelude::*;

use crate::board::{
    CurrentBoard,
    components::Position
};
use crate::pieces::components::{Occupier, Health};
use crate::vectors::Vector2Int;

use super::Action;


// pub struct MeleeHitAction{
//     pub attacker: Entity,
//     pub target: Vector2Int,
//     pub damage: u32
// }
// impl Action for MeleeHitAction {
//     fn execute(&self, world: &mut World) -> bool {
//         let Some(attacker_position) = world.get::<Position>(self.attacker) else { return false };
//         if attacker_position.v.manhattan(self.target) > 1 { return false };
//         let Some(target_entity)
//         info!("Hit!");
//         true
//     }
// }

pub struct WalkAction(pub Entity, pub Vector2Int);
impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> bool {
        let Some(board) = world.get_resource::<CurrentBoard>() else { return false };
        if !board.tiles.contains_key(&self.1) { return  false };
        if world.query_filtered::<&Position, With<Occupier>>().iter(world).any(|p| p.v == self.1) {
            return false
        };
        let Some(mut position) = world.get_mut::<Position>(self.0) else { return false };
        position.v = self.1;
        true
    }
}