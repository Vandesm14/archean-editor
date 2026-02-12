use archean_editor::Blueprint;
use bevy::{
  color::palettes::css,
  input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
  math::Vec3,
  prelude::*,
};
use bevy_obj::{ObjPlugin, ObjSettings, mesh::load_obj_as_mesh};

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

fn main() {
  let file = fs::read_to_string("temp/test2.json").unwrap();
  let blueprint = serde_json::from_str::<Blueprint>(&file).unwrap();
  let save_file = SaveFile { blueprint };

  App::new()
    // .insert_resource(GlobalAmbientLight {
    //   brightness: 50.0,
    //   ..Default::default()
    // })
    .insert_resource(save_file)
    .init_resource::<CameraSettings>()
    .add_plugins((DefaultPlugins, MeshPickingPlugin, ObjPlugin))
    .add_plugins(bevy::pbr::wireframe::WireframePlugin::default())
    .insert_resource(bevy::pbr::wireframe::WireframeConfig {
      global: true,
      default_color: Color::from(css::GREEN),
    })
    .insert_resource(GlobalAmbientLight {
      brightness: 500.0,
      ..Default::default()
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
  asset_server: Res<AssetServer>,
  save_file: Res<SaveFile>,
) {
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

  // commands.spawn();

  // Mesh Primitives.
  let slope = Mesh::from(Extrusion::new(
    Triangle2d::new(
      Vec2::new(-0.5, -0.5),
      Vec2::new(0.5, -0.5),
      Vec2::new(0.5, 0.5),
    ),
    1.0,
  ));

  // OBJ Assets.
  let obj_settings = ObjSettings::default();
  let corner =
    load_obj_as_mesh(&fs::read("assets/corner.obj").unwrap(), &obj_settings)
      .unwrap();
  let pyramid =
    load_obj_as_mesh(&fs::read("assets/pyramid.obj").unwrap(), &obj_settings)
      .unwrap();
  let inverted_corner = load_obj_as_mesh(
    &fs::read("assets/inverted_corner.obj").unwrap(),
    &obj_settings,
  )
  .unwrap();

  // Meshes.
  let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

  // Row 1
  let type_01 = meshes.add(
    slope.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians())),
    ),
  );
  let type_02 = meshes.add(
    slope.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians())),
    ),
  );
  let type_03 = meshes.add(slope.clone().transformed_by(
    Transform::default().with_rotation(
      Quat::from_rotation_y(-90.0_f32.to_radians())
        * Quat::from_rotation_x(180.0_f32.to_radians()),
    ),
  ));
  let type_04 = meshes.add(slope.clone().transformed_by(
    Transform::default().with_rotation(
      Quat::from_rotation_y(90.0_f32.to_radians())
        * Quat::from_rotation_x(180.0_f32.to_radians()),
    ),
  ));
  let type_05 = meshes.add(slope.clone().transformed_by(
    Transform::default().with_rotation(
      Quat::from_rotation_y(180.0_f32.to_radians())
        * Quat::from_rotation_x(-90.0_f32.to_radians()),
    ),
  ));
  let type_06 = meshes.add(
    slope.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_y(180.0_f32.to_radians())),
    ),
  );
  let type_07 = meshes.add(slope.clone().transformed_by(
    Transform::default().with_rotation(
      Quat::from_rotation_y(180.0_f32.to_radians())
        * Quat::from_rotation_x(90.0_f32.to_radians()),
    ),
  ));
  let type_08 = meshes.add(
    slope.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_z(180.0_f32.to_radians())),
    ),
  );
  let type_09 = meshes.add(
    slope.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_x(90.0_f32.to_radians())),
    ),
  );
  let type_10 = meshes.add(
    slope.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_z(90.0_f32.to_radians())),
    ),
  );
  let type_11 = meshes.add(
    slope.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_x(-90.0_f32.to_radians())),
    ),
  );
  let type_12 = meshes.add(slope.clone());
  let type_13 = meshes.add(
    corner.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians())),
    ),
  );
  let type_14 = meshes.add(
    corner.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_y(180.0_f32.to_radians())),
    ),
  );
  let type_15 = meshes.add(corner.clone().transformed_by(
    Transform::default().with_rotation(
      Quat::from_rotation_y(180.0_f32.to_radians())
        * Quat::from_rotation_z(-90.0_f32.to_radians()),
    ),
  ));
  let type_16 = meshes.add(corner.clone().transformed_by(
    Transform::default().with_rotation(
      Quat::from_rotation_y(-90.0_f32.to_radians())
        * Quat::from_rotation_x(90.0_f32.to_radians()),
    ),
  ));
  let type_17 = meshes.add(corner.clone());
  let type_18 = meshes.add(
    corner.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians())),
    ),
  );
  let type_19 = meshes.add(
    corner.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_x(90.0_f32.to_radians())),
    ),
  );
  let type_20 = meshes.add(
    corner.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_x(180.0_f32.to_radians())),
    ),
  );
  let type_21 = meshes.add(
    pyramid.clone().transformed_by(
      Transform::default()
        .with_rotation(Quat::from_rotation_y(-90.0_f32.to_radians())),
    ),
  );
  let type_22 = meshes.add(pyramid.clone().transformed_by(
    Transform::default().with_rotation(
      Quat::from_rotation_x(-90.0_f32.to_radians())
        * Quat::from_rotation_y(-90.0_f32.to_radians()),
    ),
  ));
  let type_23 = meshes.add(pyramid.clone().transformed_by(
    Transform::default().with_rotation(
      Quat::from_rotation_x(180.0_f32.to_radians())
        * Quat::from_rotation_y(-90.0_f32.to_radians()),
    ),
  ));
  let type_24 = meshes.add(pyramid.clone().transformed_by(
    Transform::default().with_rotation(
      Quat::from_rotation_x(90.0_f32.to_radians())
        * Quat::from_rotation_y(-90.0_f32.to_radians()),
    ),
  ));
  let type_25 = meshes.add(pyramid.clone().transformed_by(
    Transform::default().with_rotation(
      Quat::from_rotation_x(-90.0_f32.to_radians())
        * Quat::from_rotation_z(-90.0_f32.to_radians()),
    ),
  ));
  let type_26 = meshes.add(pyramid.clone().transformed_by(
    Transform::default().with_rotation(
      Quat::from_rotation_x(180.0_f32.to_radians())
        * Quat::from_rotation_z(-90.0_f32.to_radians()),
    ),
  ));
  let type_27 = meshes.add(pyramid.clone().transformed_by(
    Transform::default().with_rotation(
      Quat::from_rotation_x(90.0_f32.to_radians())
        * Quat::from_rotation_z(-90.0_f32.to_radians()),
    ),
  ));
  let type_28 = cube.clone();
  let type_29 = cube.clone();
  let type_30 = cube.clone();
  let type_31 = cube.clone();
  let type_32 = cube.clone();
  let type_33 = cube.clone();
  let type_34 = cube.clone();
  let type_35 = cube.clone();
  let type_36 = cube.clone();
  let type_37 = cube.clone();
  let type_38 = cube.clone();
  let type_39 = cube.clone();
  let type_40 = cube.clone();
  let type_41 = cube.clone();
  let type_42 = cube.clone();
  let type_43 = cube.clone();
  let type_44 = cube.clone();
  let type_45 = cube.clone();
  let type_46 = cube.clone();
  let type_47 = cube.clone();
  let type_48 = cube.clone();
  let type_49 = cube.clone();
  let type_50 = cube.clone();
  let type_51 = cube.clone();
  let type_52 = cube.clone();
  let type_53 = cube.clone();
  let type_54 = cube.clone();
  let type_55 = cube.clone();
  let type_56 = cube.clone();
  let type_57 = cube.clone();
  let type_58 = cube.clone();
  let type_59 = cube.clone();

  let mesh_map: Vec<Handle<Mesh>> = vec![
    cube.clone(),    // 00
    type_01.clone(), // 01
    type_02.clone(), // 02
    type_03.clone(), // 03
    type_04.clone(), // 04
    type_05.clone(), // 05
    type_06.clone(), // 06
    type_07.clone(), // 07
    type_08.clone(), // 08
    type_09.clone(), // 09
    type_10.clone(), // 10
    type_11.clone(), // 11
    type_12.clone(), // 12
    type_13.clone(), // 13
    type_14.clone(), // 14
    type_15.clone(), // 15
    type_16.clone(), // 16
    type_17.clone(), // 17
    type_18.clone(), // 18
    type_19.clone(), // 19
    type_20.clone(), // 20
    type_21.clone(), // 21
    type_22.clone(), // 22
    type_23.clone(), // 23
    type_24.clone(), // 24
    type_25.clone(), // 25
    type_26.clone(), // 26
    type_27.clone(), // 27
    type_28.clone(), // 28
    type_29.clone(), // 29
    type_30.clone(), // 30
    type_31.clone(), // 31
    type_32.clone(), // 32
    type_33.clone(), // 33
    type_34.clone(), // 34
    type_35.clone(), // 35
    type_36.clone(), // 36
    type_37.clone(), // 37
    type_38.clone(), // 38
    type_39.clone(), // 39
    type_40.clone(), // 40
    type_41.clone(), // 41
    type_42.clone(), // 42
    type_43.clone(), // 43
    type_44.clone(), // 44
    type_45.clone(), // 45
    type_46.clone(), // 46
    type_47.clone(), // 47
    type_48.clone(), // 48
    type_49.clone(), // 49
    type_50.clone(), // 50
    type_51.clone(), // 51
    type_52.clone(), // 52
    type_53.clone(), // 53
    type_54.clone(), // 54
    type_55.clone(), // 55
    type_56.clone(), // 56
    type_57.clone(), // 57
    type_58.clone(), // 58
    type_59.clone(), // 59
    cube.clone(),    // 60
    cube.clone(),    // 61
    cube.clone(),    // 62
    cube.clone(),    // 63
    cube.clone(),    // 64
    cube.clone(),    // 65
    cube.clone(),    // 66
    cube.clone(),    // 67
    cube.clone(),    // 68
    cube.clone(),    // 69
    cube.clone(),    // 70
    cube.clone(),    // 71
    cube.clone(),    // 72
    cube.clone(),    // 73
    cube.clone(),    // 74
    cube.clone(),    // 75
    cube.clone(),    // 76
    cube.clone(),    // 77
    cube.clone(),    // 78
    cube.clone(),    // 79
  ];

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
    let mesh = mesh_map
      .get(block.r#type as usize)
      .cloned()
      .ok_or_else(|| error!("no mesh found for type: {}", block.r#type))
      .unwrap();

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
        if event.button == PointerButton::Primary {
          let block = save_file.blueprint.data.blocks.get(i).unwrap();
          info!("picked block: {i} with type {}", block.r#type);
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
      let (pitch, roll, yaw) = camera.rotation.to_euler(EulerRot::XYZ);
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
