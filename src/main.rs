use archean_editor::Blueprint;
use bevy::{
  color::palettes::css,
  input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
  math::Vec3,
  prelude::*,
};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_obj::ObjPlugin;

use std::{f32::consts::FRAC_PI_2, ops::Range};

const FRAME_SIZE: f32 = 12.0;

#[derive(Debug, Resource)]
struct CameraSettings {
  pub orbit_distance: f32,
  pub pitch_speed: f32,
  // Clamp pitch to this range
  pub pitch_range: Range<f32>,
  pub yaw_speed: f32,
  pub target: Vec3,
}

impl Default for CameraSettings {
  fn default() -> Self {
    const SPEED: f32 = 0.02;

    // Limiting pitch stops some unexpected rotation past 90° up or down.
    let pitch_limit = FRAC_PI_2 - 0.01;
    Self {
      // These values are completely arbitrary, chosen because they seem to produce
      // "sensible" results for this example. Adjust as required.
      orbit_distance: 10.0,
      pitch_speed: SPEED,
      pitch_range: -pitch_limit..pitch_limit,
      yaw_speed: SPEED * 0.5,
      target: Vec3::ZERO,
    }
  }
}

#[derive(Resource)]
struct BlockMap {
  map: [Handle<Mesh>; 53],
}

impl BlockMap {
  fn get(&self, r#type: u8) -> Handle<Mesh> {
    self
      .map
      .get(r#type as usize)
      .cloned()
      .unwrap_or_else(|| self.map[0].clone())
  }
}

impl FromWorld for BlockMap {
  fn from_world(world: &mut World) -> Self {
    let asset_server = world.resource::<AssetServer>();

    Self {
      map: [
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
    }
  }
}

#[derive(Debug, Deref, Resource)]
struct LoadedBlueprint(Handle<Blueprint>);

impl FromWorld for LoadedBlueprint {
  fn from_world(world: &mut World) -> Self {
    let asset_server = world.resource::<AssetServer>();
    Self(asset_server.load("blueprint.json"))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
enum BlueprintLoadState {
  #[default]
  Unloaded,
  Loaded,
}

fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins,
      MeshPickingPlugin,
      ObjPlugin,
      JsonAssetPlugin::<Blueprint>::new(&["json"]),
    ))
    .add_plugins(bevy::pbr::wireframe::WireframePlugin::default())
    .init_state::<BlueprintLoadState>()
    .insert_resource(bevy::pbr::wireframe::WireframeConfig {
      global: true,
      default_color: Color::from(css::GREEN),
    })
    .insert_resource(GlobalAmbientLight {
      brightness: 500.0,
      ..Default::default()
    })
    .init_resource::<CameraSettings>()
    .init_resource::<BlockMap>()
    .init_resource::<LoadedBlueprint>()
    .add_systems(Startup, setup)
    .add_systems(OnEnter(BlueprintLoadState::Loaded), setup_blueprint)
    .add_systems(Update, (blueprint_load_unload, orbit, axes))
    .run();
}

fn blueprint_load_unload(
  mut blueprint_state: ResMut<NextState<BlueprintLoadState>>,
  mut events: MessageReader<AssetEvent<Blueprint>>,
) {
  for event in events.read() {
    match event {
      AssetEvent::Modified { .. } | AssetEvent::Removed { .. } => {
        blueprint_state.set(BlueprintLoadState::Unloaded)
      }
      AssetEvent::LoadedWithDependencies { .. } => {
        blueprint_state.set(BlueprintLoadState::Loaded)
      }
      _ => {}
    }
  }
}

fn setup(mut commands: Commands) {
  commands
    .spawn((
      Name::new("Camera"),
      Camera3d::default(),
      Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ))
    .with_child((
      DirectionalLight {
        illuminance: 5000.0,
        shadows_enabled: true,
        ..Default::default()
      },
      Transform::from_xyz(10.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// # Panics
///
/// If `SaveFile.blueprint` is not fully loaded.
fn setup_blueprint(
  mut commands: Commands,
  mut materials: ResMut<Assets<StandardMaterial>>,
  blueprints: Res<Assets<Blueprint>>,
  blueprint: Res<LoadedBlueprint>,
  block_map: Res<BlockMap>,
) {
  let blueprint = blueprints.get(blueprint.id()).unwrap();

  let blank = materials.add(Color::from(css::WHITE));

  for frame in blueprint.data.frames.iter() {
    commands.spawn((
      DespawnOnExit(BlueprintLoadState::Unloaded),
      Mesh3d(block_map.get(0)),
      Transform::from_xyz(
        frame.frame_x as f32 * FRAME_SIZE + FRAME_SIZE * 0.5,
        frame.frame_y as f32 * FRAME_SIZE + FRAME_SIZE * 0.5,
        frame.frame_z as f32 * FRAME_SIZE + FRAME_SIZE * 0.5,
      )
      .with_scale(Vec3::splat(FRAME_SIZE)),
      Pickable::IGNORE,
    ));
  }

  for (i, block) in blueprint.data.blocks.iter().enumerate() {
    let size_x = block.size_x as f32 + 1.0;
    let size_y = block.size_y as f32 + 1.0;
    let size_z = block.size_z as f32 + 1.0;

    commands
      .spawn((
        DespawnOnExit(BlueprintLoadState::Unloaded),
        Mesh3d(block_map.get(block.r#type)),
        MeshMaterial3d(blank.clone()),
        Transform::from_xyz(
          block.frame_x as f32 * FRAME_SIZE + block.pos_x as f32 + size_x * 0.5,
          block.frame_y as f32 * FRAME_SIZE + block.pos_y as f32 + size_y * 0.5,
          block.frame_z as f32 * FRAME_SIZE + block.pos_z as f32 + size_z * 0.5,
        )
        .with_scale(Vec3::new(size_x, size_y, size_z)),
      ))
      .observe(
        move |event: On<Pointer<Click>>,
              blueprints: Res<Assets<Blueprint>>,
              blueprint: Res<LoadedBlueprint>| {
          if event.button == PointerButton::Primary {
            let blueprint = blueprints.get(blueprint.id()).unwrap();
            let block = blueprint.data.blocks.get(i).unwrap();
            info!("picked block: {i} with type {}", block.r#type);
          }
        },
      );
  }
}

fn axes(mut gizmos: Gizmos) {
  gizmos.axes(Transform::default(), 1.0);
}

fn orbit(
  mut camera: Single<&mut Transform, With<Camera>>,
  mut camera_settings: ResMut<CameraSettings>,
  mouse_motion: Res<AccumulatedMouseMotion>,
  mouse_buttons: Res<ButtonInput<MouseButton>>,
  mouse_scroll: Res<AccumulatedMouseScroll>,
  key_input: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
) {
  let zoom_delta = mouse_scroll.delta;
  camera_settings.orbit_distance *=
    1.0 - time.delta_secs() * zoom_delta.y * 15.0;

  if mouse_buttons.pressed(MouseButton::Left) {
    let delta = mouse_motion.delta;

    if key_input.pressed(KeyCode::ShiftLeft)
      || key_input.pressed(KeyCode::ShiftRight)
    {
      let (pitch, _roll, _yaw) = camera.rotation.to_euler(EulerRot::XYZ);
      let x = -camera.right() * delta.x * 0.1;
      let z = (camera.forward() * pitch.cos() + -camera.up() * pitch.sin())
        * delta.y
        * 0.1;
      camera_settings.target += x + z;
    } else {
      // Mouse motion is one of the few inputs that should not be multiplied by delta time,
      // as we are already receiving the full movement since the last frame was rendered. Multiplying
      // by delta time here would make the movement slower that it should be.
      let delta_pitch = -delta.y * camera_settings.pitch_speed;
      let delta_yaw = -delta.x * camera_settings.yaw_speed;

      // Obtain the existing pitch, yaw, and roll values from the transform.
      let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);

      // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
      let pitch = (pitch + delta_pitch).clamp(
        camera_settings.pitch_range.start,
        camera_settings.pitch_range.end,
      );
      let yaw = yaw + delta_yaw;
      camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);

      // Adjust the translation to maintain the correct orientation toward the orbit target.
      // In our example it's a static target, but this could easily be customized.
    }
  }

  camera.translation =
    camera_settings.target - camera.forward() * camera_settings.orbit_distance;
}
