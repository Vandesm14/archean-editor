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
  pub cube: Handle<Mesh>,
  pub slope: Handle<Mesh>,
  pub corner: Handle<Mesh>,
  pub pyramid: Handle<Mesh>,
  pub inv_corner: Handle<Mesh>,
  pub unselected: Handle<StandardMaterial>,
  pub selected: Handle<StandardMaterial>,
}

impl FromWorld for CommonAssets {
  fn from_world(world: &mut World) -> Self {
    let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

    let unselected = materials.add(Color::WHITE);
    let selected = materials.add(Color::from(css::BLUE));

    let asset_server = world.resource::<AssetServer>();

    let cube = asset_server.load("blocks/cube.obj");
    let slope = asset_server.load("blocks/slope.obj");
    let corner = asset_server.load("blocks/corner.obj");
    let pyramid = asset_server.load("blocks/pyramid.obj");
    let inv_corner = asset_server.load("blocks/inv_corner.obj");

    Self {
      cube,
      slope,
      corner,
      pyramid,
      inv_corner,
      unselected,
      selected,
    }
  }
}
