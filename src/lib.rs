pub mod action;
pub mod blueprint;

use bevy::prelude::*;

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
  blocks: [Handle<Mesh>; 53],
  unselected: Handle<StandardMaterial>,
  selected: Handle<StandardMaterial>,
}

impl CommonAssets {
  pub fn block(&self, id: u8) -> Handle<Mesh> {
    self
      .blocks
      .get(id as usize)
      .cloned()
      .unwrap_or_else(|| self.blocks[0].clone())
  }
}

impl FromWorld for CommonAssets {
  fn from_world(world: &mut World) -> Self {
    let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
    let unselected = materials.add(Color::WHITE);
    let selected = materials.add(Color::BLACK);

    let asset_server = world.resource::<AssetServer>();

    Self {
      blocks: [
        asset_server.load("blocks/00.obj"),
        asset_server.load("blocks/01.obj"),
        asset_server.load("blocks/02.obj"),
        asset_server.load("blocks/03.obj"),
        asset_server.load("blocks/04.obj"),
        asset_server.load("blocks/05.obj"),
        asset_server.load("blocks/06.obj"),
        asset_server.load("blocks/07.obj"),
        asset_server.load("blocks/08.obj"),
        asset_server.load("blocks/09.obj"),
        asset_server.load("blocks/10.obj"),
        asset_server.load("blocks/11.obj"),
        asset_server.load("blocks/12.obj"),
        asset_server.load("blocks/13.obj"),
        asset_server.load("blocks/14.obj"),
        asset_server.load("blocks/15.obj"),
        asset_server.load("blocks/16.obj"),
        asset_server.load("blocks/17.obj"),
        asset_server.load("blocks/18.obj"),
        asset_server.load("blocks/19.obj"),
        asset_server.load("blocks/20.obj"),
        asset_server.load("blocks/21.obj"),
        asset_server.load("blocks/22.obj"),
        asset_server.load("blocks/23.obj"),
        asset_server.load("blocks/24.obj"),
        asset_server.load("blocks/25.obj"),
        asset_server.load("blocks/26.obj"),
        asset_server.load("blocks/27.obj"),
        asset_server.load("blocks/28.obj"),
        asset_server.load("blocks/29.obj"),
        asset_server.load("blocks/30.obj"),
        asset_server.load("blocks/31.obj"),
        asset_server.load("blocks/32.obj"),
        asset_server.load("blocks/33.obj"),
        asset_server.load("blocks/34.obj"),
        asset_server.load("blocks/35.obj"),
        asset_server.load("blocks/36.obj"),
        asset_server.load("blocks/37.obj"),
        asset_server.load("blocks/38.obj"),
        asset_server.load("blocks/39.obj"),
        asset_server.load("blocks/40.obj"),
        asset_server.load("blocks/41.obj"),
        asset_server.load("blocks/42.obj"),
        asset_server.load("blocks/43.obj"),
        asset_server.load("blocks/44.obj"),
        asset_server.load("blocks/45.obj"),
        asset_server.load("blocks/46.obj"),
        asset_server.load("blocks/47.obj"),
        asset_server.load("blocks/48.obj"),
        asset_server.load("blocks/49.obj"),
        asset_server.load("blocks/50.obj"),
        asset_server.load("blocks/51.obj"),
        asset_server.load("blocks/52.obj"),
      ],
      unselected,
      selected,
    }
  }
}
