pub mod action;
pub mod blueprint;

use bevy::prelude::*;

/// Marks an entity as selected.
#[derive(Component)]
pub struct Selected;
