use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

pub struct BlueprintPlugin;

impl Plugin for BlueprintPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_asset::<Blueprint>()
      .init_state::<BlueprintState>()
      .init_resource::<LoadedBlueprint>()
      .add_systems(PostUpdate, update_blueprint_state);
  }
}

#[derive(Deref, DerefMut, Resource)]
pub struct LoadedBlueprint(pub Handle<Blueprint>);

impl FromWorld for LoadedBlueprint {
  fn from_world(world: &mut World) -> Self {
    let asset_server = world.resource::<AssetServer>();
    Self(asset_server.load("blueprint.json"))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum BlueprintState {
  #[default]
  Unloaded,
  Loaded,
}

pub fn update_blueprint_state(
  mut blueprint_state: ResMut<NextState<BlueprintState>>,
  mut events: MessageReader<AssetEvent<Blueprint>>,
) {
  for event in events.read() {
    match event {
      AssetEvent::Modified { .. } | AssetEvent::Removed { .. } => {
        blueprint_state.set(BlueprintState::Unloaded)
      }
      AssetEvent::LoadedWithDependencies { .. } => {
        blueprint_state.set(BlueprintState::Loaded)
      }
      _ => {}
    }
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coords {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CoordsW {
  pub w: f64,
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
  pub colors: (u8, u8, u8, u8, u8, u8, u8),
  pub extra: u8,
  pub frame_x: i8,
  pub frame_y: i8,
  pub frame_z: i8,
  pub material: u8,
  pub pos_x: u8,
  pub pos_y: u8,
  pub pos_z: u8,
  pub size_x: u8,
  pub size_y: u8,
  pub size_z: u8,
  pub r#type: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorMaterial {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub metallic: u8,
  pub opacity: u8,
  pub roughness: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorRGB {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorARGB {
  pub a: u8,
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorOrZero {
  Color(ColorMaterial),
  Zero(u8),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
  Bool(bool),
  Int(isize),
  Float(f64),
  String(String),
  Map(HashMap<String, Value>),
  Vec(Vec<Value>),
  Null(()),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Occupancy {
  pub frame_x: i8,
  pub frame_y: i8,
  pub frame_z: i8,
  pub pos_x: u8,
  pub pos_y: u8,
  pub pos_z: u8,
  pub size_x: u8,
  pub size_y: u8,
  pub size_z: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
  #[serde(default)]
  pub alias: Option<String>,
  pub colors: HashMap<String, ColorMaterial>,
  pub data: HashMap<String, Value>,
  pub module: String,
  pub occupancies: Vec<Occupancy>,
  pub orientation: CoordsW,
  pub position: Coords,
  pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
  pub beams: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),
  pub frame_x: i8,
  pub frame_y: i8,
  pub frame_z: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
  pub align_center: u8,
  pub dir_x: u8,
  pub dir_y: u8,
  pub dir_z: u8,
  pub panel_color: ColorARGB,
  pub position: Coords,
  #[serde(default)]
  pub metallic: Option<u8>,
  pub roughness: u8,
  pub size: f32,
  pub text: String,
  pub text_color: ColorRGB,
  pub up_x: u8,
  pub up_y: u8,
  pub up_z: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipeSegment {
  pub dir: u8,
  pub flexible: bool,
  pub length: f64,
  pub start: Coords,
  pub a: u8,
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub chrome: bool,
  pub glossy: bool,
  pub metal: bool,
  pub striped: bool,
  pub r#box: bool,
  pub rounded_caps: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipe {
  pub a_component: u8,
  pub a_port: String,
  pub b_component: u8,
  pub b_port: String,
  pub radius: f64,
  pub segments: Vec<PipeSegment>,
  pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeBuild {
  pub component: u8,
  #[serde(rename = "slaveBuildId")]
  pub slave_build_id: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintData {
  pub alias: String,
  pub blocks: Vec<Block>,
  pub colors: Vec<ColorOrZero>,
  pub components: Vec<Component>,
  pub composite_builds: Vec<CompositeBuild>,
  pub doors: Vec<()>,
  pub frames: Vec<Frame>,
  pub labels: Vec<Label>,
  pub pipes: Vec<Pipe>,
  pub symmetry_axis: u8,
  pub symmetry_axis_offset: Coords,
  pub version: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
pub struct Blueprint {
  pub author: String,
  pub box_max: Coords,
  pub box_min: Coords,
  pub box_size: Coords,
  pub data: BlueprintData,
  pub datetime: String,
  pub mass: f32,
  pub r#type: String,
  pub version: u8,
}
