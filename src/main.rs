// use bevy::{
//   input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
//   log::tracing,
//   math::Vec3,
//   prelude::*,
// };

// use std::{f32::consts::FRAC_PI_2, ops::Range};

// #[derive(Debug, Resource)]
// struct CameraSettings {
//   pub orbit_distance: f32,
//   pub pitch_speed: f32,
//   // Clamp pitch to this range
//   pub pitch_range: Range<f32>,
//   pub yaw_speed: f32,
// }

// impl Default for CameraSettings {
//   fn default() -> Self {
//     const SPEED: f32 = 0.01;

//     // Limiting pitch stops some unexpected rotation past 90° up or down.
//     let pitch_limit = FRAC_PI_2 - 0.01;
//     Self {
//       // These values are completely arbitrary, chosen because they seem to produce
//       // "sensible" results for this example. Adjust as required.
//       orbit_distance: 10.0,
//       pitch_speed: SPEED,
//       pitch_range: -pitch_limit..pitch_limit,
//       yaw_speed: SPEED * 0.5,
//     }
//   }
// }

// fn main() {
//   App::new()
//     .init_resource::<CameraSettings>()
//     .add_plugins(DefaultPlugins)
//     // .add_plugins(bevy::pbr::wireframe::WireframePlugin::default())
//     // .insert_resource(bevy::pbr::wireframe::WireframeConfig {
//     //   global: true,
//     //   default_color: Color::WHITE,
//     // })
//     .add_systems(Startup, setup)
//     .add_systems(Update, orbit)
//     .run();
// }

// /// set up a simple 3D scene
// fn setup(
//   mut commands: Commands,
//   mut meshes: ResMut<Assets<Mesh>>,
//   mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//   commands.spawn((
//     Name::new("Camera"),
//     Camera3d::default(),
//     Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//   ));

//   tracing::info!("Done.");
// }

// fn orbit(
//   mut camera: Single<&mut Transform, With<Camera>>,
//   mut camera_settings: ResMut<CameraSettings>,
//   mouse_motion: Res<AccumulatedMouseMotion>,
//   mouse_buttons: Res<ButtonInput<MouseButton>>,
//   mouse_scroll: Res<AccumulatedMouseScroll>,
//   time: Res<Time>,
// ) {
//   let zoom_delta = mouse_scroll.delta;
//   camera_settings.orbit_distance *=
//     1.0 - time.delta_secs() * zoom_delta.y * 15.0;

//   if mouse_buttons.pressed(MouseButton::Left) {
//     let delta = mouse_motion.delta;

//     // Mouse motion is one of the few inputs that should not be multiplied by delta time,
//     // as we are already receiving the full movement since the last frame was rendered. Multiplying
//     // by delta time here would make the movement slower that it should be.
//     let delta_pitch = -delta.y * camera_settings.pitch_speed;
//     let delta_yaw = -delta.x * camera_settings.yaw_speed;

//     // Obtain the existing pitch, yaw, and roll values from the transform.
//     let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);

//     // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
//     let pitch = (pitch + delta_pitch).clamp(
//       camera_settings.pitch_range.start,
//       camera_settings.pitch_range.end,
//     );
//     let yaw = yaw + delta_yaw;
//     camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);

//     // Adjust the translation to maintain the correct orientation toward the orbit target.
//     // In our example it's a static target, but this could easily be customized.
//   }

//   let target = Vec3::ZERO;
//   camera.translation =
//     target - camera.forward() * camera_settings.orbit_distance;
// }

use std::fs;

use archean_editor::Blueprint;

pub fn main() {
  let file = fs::read_to_string("temp/test.json").unwrap();
  let bp = serde_json::from_str::<Blueprint>(&file);
  match bp {
    Ok(bp) => fs::write(
      "temp/test.rs.json",
      serde_json::to_string_pretty(&bp).unwrap(),
    )
    .unwrap(),
    Err(e) => panic!("{e:?}"),
  };
}
