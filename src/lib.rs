pub mod action;
pub mod block;
pub mod blueprint;

use bevy::{color::palettes::css, prelude::*};

use crate::action::{ActionMessage, CombinedAction, SelectionAction};

pub fn select_entity(
  event: On<Pointer<Click>>,
  keycode: Res<ButtonInput<KeyCode>>,
  query: Query<Entity, With<Selected>>,
  mut messages: MessageWriter<ActionMessage>,
) {
  // TODO: Make controls configurable.
  if event.button == PointerButton::Primary {
    // TODO: Make controls configurable.
    if keycode.pressed(KeyCode::ShiftLeft) {
      messages
        .write(ActionMessage::Push(Box::new(SelectionAction(event.entity))));
    } else {
      messages.write(ActionMessage::Push(Box::new(CombinedAction::from_iter(
        query
          .iter()
          .map(|entity| Box::new(SelectionAction(entity)) as _)
          .chain(core::iter::once(
            Box::new(SelectionAction(event.entity)) as _
          )),
      ))));
    }
  }
}

pub fn swap_to_selected_material(
  event: On<Add, Selected>,
  common_assets: Res<CommonAssets>,
  mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
  if let Ok(mut material) = query.get_mut(event.entity) {
    material.0 = common_assets.selected.clone();
  }
}

pub fn swap_to_deselected_material(
  event: On<Remove, Selected>,
  common_assets: Res<CommonAssets>,
  mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
  if let Ok(mut material) = query.get_mut(event.entity) {
    material.0 = common_assets.unselected.clone();
  }
}

/// Marks an entity as selected.
#[derive(Component)]
pub struct Selected;

#[derive(Resource)]
pub struct CommonAssets {
  pub unselected: Handle<StandardMaterial>,
  pub selected: Handle<StandardMaterial>,
}

impl FromWorld for CommonAssets {
  fn from_world(world: &mut World) -> Self {
    let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

    let unselected = materials.add(Color::WHITE);
    let selected = materials.add(Color::from(css::BLUE));

    Self {
      unselected,
      selected,
    }
  }
}
