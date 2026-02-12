use archean_editor::Blueprint;
use bevy::{
  color::palettes::css,
  input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
  math::Vec3,
  prelude::*,
};

use std::{f32::consts::FRAC_PI_2, fs, ops::Range};

const FRAME_SIZE: f32 = 12.0;

#[derive(Debug, Resource)]
struct SaveFile {
  blueprint: Blueprint,
}

#[derive(Debug, Resource)]
struct CameraSettings {
  pub orbit_distance: f32,
  pub pitch_speed: f32,
  // Clamp pitch to this range
  pub pitch_range: Range<f32>,
  pub yaw_speed: f32,
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
    }
  }
}

fn main() {
  let file = fs::read_to_string("temp/test2.json").unwrap();
  let blueprint = serde_json::from_str::<Blueprint>(&file).unwrap();
  let save_file = SaveFile { blueprint };

  App::new()
    .insert_resource(GlobalAmbientLight {
      brightness: 500.0,
      ..Default::default()
    })
    .insert_resource(save_file)
    .init_resource::<CameraSettings>()
    .add_plugins((DefaultPlugins, MeshPickingPlugin))
    .add_plugins(bevy::pbr::wireframe::WireframePlugin::default())
    .insert_resource(bevy::pbr::wireframe::WireframeConfig {
      global: true,
      default_color: Color::WHITE,
    })
    .add_systems(Startup, setup)
    .add_systems(Update, orbit)
    .run();
}

/// set up a simple 3D scene
fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  save_file: Res<SaveFile>,
) {
  commands.spawn((
    Name::new("Camera"),
    Camera3d::default(),
    Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
  ));

  // Mesh Primitives.
  let tri_prism = Mesh::from(Extrusion::new(
    Triangle2d::new(
      Vec2::new(-0.5, -0.5),
      Vec2::new(0.5, -0.5),
      Vec2::new(0.5, 0.5),
    ),
    1.0,
  ));

  // Meshes.
  let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
  let type1prism = meshes.add(
    tri_prism.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians())),
    ),
  );
  let type2prism = meshes.add(
    tri_prism.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians())),
    ),
  );
  let type6prism = meshes.add(tri_prism);

  let blank = materials.add(Color::from(css::WHITE));

  for frame in save_file.blueprint.data.frames.iter() {
    commands.spawn((
      Mesh3d(cube.clone()),
      // MeshMaterial3d(blank.clone()),
      Transform::from_xyz(
        frame.frame_x as f32 * FRAME_SIZE + FRAME_SIZE * 0.5,
        frame.frame_y as f32 * FRAME_SIZE + FRAME_SIZE * 0.5,
        frame.frame_z as f32 * FRAME_SIZE + FRAME_SIZE * 0.5,
      )
      .with_scale(Vec3::splat(FRAME_SIZE)),
      Pickable::IGNORE,
    ));
  }

  for (i, block) in save_file.blueprint.data.blocks.iter().enumerate() {
    let size_x = block.size_x as f32 + 1.0;
    let size_y = block.size_y as f32 + 1.0;
    let size_z = block.size_z as f32 + 1.0;

    let mesh = match block.r#type {
      0 => cube.clone(),
      1 => type1prism.clone(),
      2 => type2prism.clone(),
      6 => type6prism.clone(),
      255 => {
        // Component
        cube.clone()
      }
      _ => cube.clone(),
    };
    commands
      .spawn((
        Mesh3d(mesh),
        MeshMaterial3d(blank.clone()),
        Transform::from_xyz(
          block.frame_x as f32 * FRAME_SIZE + block.pos_x as f32 + size_x * 0.5,
          block.frame_y as f32 * FRAME_SIZE + block.pos_y as f32 + size_y * 0.5,
          block.frame_z as f32 * FRAME_SIZE + block.pos_z as f32 + size_z * 0.5,
        )
        .with_scale(Vec3::new(size_x, size_y, size_z)),
      ))
      .observe(move |event: On<Pointer<Click>>, save_file: Res<SaveFile>| {
        if event.button == PointerButton::Primary
          && let Some(block) = save_file.blueprint.data.blocks.get(i)
        {
          info!("picked block: i\n{block:?}");
        }
      });
  }
}

fn orbit(
  mut camera: Single<&mut Transform, With<Camera>>,
  mut camera_settings: ResMut<CameraSettings>,
  mouse_motion: Res<AccumulatedMouseMotion>,
  mouse_buttons: Res<ButtonInput<MouseButton>>,
  mouse_scroll: Res<AccumulatedMouseScroll>,
  time: Res<Time>,
) {
  let zoom_delta = mouse_scroll.delta;
  camera_settings.orbit_distance *=
    1.0 - time.delta_secs() * zoom_delta.y * 15.0;

  if mouse_buttons.pressed(MouseButton::Left) {
    let delta = mouse_motion.delta;

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

  let target = Vec3::ZERO;
  camera.translation =
    target - camera.forward() * camera_settings.orbit_distance;
}
