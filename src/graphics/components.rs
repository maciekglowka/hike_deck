use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct PathAnimator(pub VecDeque<Vec3>);