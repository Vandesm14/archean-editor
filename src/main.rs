use core::{f32::consts::FRAC_PI_2, ops::Range};

use archean_editor::{
  CommonAssets,
  action::{ActionHistory, ActionMessage, ActionPlugin},
  blueprint::{Blueprint, BlueprintPlugin, BlueprintState, LoadedBlueprint},
  select_entity, swap_to_deselected_material, swap_to_selected_material,
};
use bevy::{
  camera::{CameraOutputMode, visibility::RenderLayers},
  input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
  pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
  prelude::*,
  render::render_resource::BlendState,
};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_egui::prelude::*;
use bevy_obj::ObjPlugin;

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

fn main() -> AppExit {
  App::new()
    .add_plugins((
      DefaultPlugins,
      MeshPickingPlugin,
      WireframePlugin::default(),
    ))
    .add_plugins(EguiPlugin::default())
    .add_plugins((ObjPlugin, JsonAssetPlugin::<Blueprint>::new(&["json"])))
    .add_plugins((ActionPlugin, BlueprintPlugin))
    .insert_resource(MeshPickingSettings {
      require_markers: true,
      ..Default::default()
    })
    .insert_resource(WireframeConfig {
      global: false,
      ..Default::default()
    })
    .insert_resource(GlobalAmbientLight {
      brightness: 500.0,
      ..Default::default()
    })
    .insert_resource(EguiGlobalSettings {
      auto_create_primary_context: false,
      ..Default::default()
    })
    .init_resource::<CommonAssets>()
    .init_resource::<CameraSettings>()
    .add_systems(Startup, (setup_scene, setup_ui))
    .add_systems(EguiPrimaryContextPass, show_editor_ui)
    .add_systems(OnEnter(BlueprintState::Loaded), setup_blueprint)
    .add_systems(Update, (undo_redo, reload_blueprint, orbit))
    .run()
}

fn setup_scene(mut commands: Commands, common_assets: Res<CommonAssets>) {
  commands.spawn((
    Camera3d::default(),
    Transform::from_translation(Vec3::ONE * 10.0)
      .looking_at(Vec3::ZERO, Dir3::Y),
    MeshPickingCamera,
  ));

  commands.spawn((
    DirectionalLight {
      illuminance: 10000.0,
      shadows_enabled: true,
      ..Default::default()
    },
    Transform::from_translation(Vec3::ONE * 10.0)
      .looking_at(Vec3::ZERO, Dir3::Y),
  ));
}

fn setup_ui(mut commands: Commands) {
  commands.spawn((
    Camera2d,
    PrimaryEguiContext,
    RenderLayers::none(),
    Camera {
      order: 1,
      output_mode: CameraOutputMode::Write {
        blend_state: Some(BlendState::ALPHA_BLENDING),
        clear_color: ClearColorConfig::None,
      },
      clear_color: ClearColorConfig::Custom(Color::NONE),
      ..Default::default()
    },
  ));
}

fn show_editor_ui(mut contexts: EguiContexts) -> Result {
  let ctx = contexts.ctx_mut()?;

  // TODO: Read controls from wherever they end up being confugured.
  egui::Window::new("Controls").show(ctx, |ui| {
    ui.heading("Camera");
    ui.label("<MiddleMouse> to rotate.");
    ui.label("<Shift+MiddleMouse> to translate.");

    ui.separator();

    ui.heading("History");
    ui.label("<Control+Z> to undo.");
    ui.label("<Control+Shift+Z> or <Control+Y> to redo.");

    ui.separator();

    ui.heading("Selection");
    ui.label("<PrimaryMouse> to select hovered block.");
    ui.label("<Shift+PrimaryMouse> to add hovered block to selection.");

    ui.separator();

    ui.heading("Blueprint");
    ui.label("<Control+R> to reload the blueprint file.");
  });

  Ok(())
}

fn setup_blueprint(
  mut commands: Commands,
  blueprints: Res<Assets<Blueprint>>,
  blueprint: Res<LoadedBlueprint>,
  common_assets: Res<CommonAssets>,
) {
  let blueprint = blueprints.get(blueprint.id()).unwrap();

  for frame in blueprint.data.frames.iter() {
    commands.spawn((
      DespawnOnExit(BlueprintState::Unloaded),
      Mesh3d(common_assets.block(0)),
      Transform::from_xyz(
        frame.frame_x as f32 * FRAME_SIZE + FRAME_SIZE * 0.5,
        frame.frame_y as f32 * FRAME_SIZE + FRAME_SIZE * 0.5,
        frame.frame_z as f32 * FRAME_SIZE + FRAME_SIZE * 0.5,
      )
      .with_scale(Vec3::splat(FRAME_SIZE)),
      Wireframe,
    ));
  }

  for block in blueprint.data.blocks.iter() {
    let size_x = block.size_x as f32 + 1.0;
    let size_y = block.size_y as f32 + 1.0;
    let size_z = block.size_z as f32 + 1.0;

    commands
      .spawn((
        DespawnOnExit(BlueprintState::Unloaded),
        Mesh3d(common_assets.block(block.r#type)),
        MeshMaterial3d(common_assets.unselected.clone()),
        Transform::from_xyz(
          block.frame_x as f32 * FRAME_SIZE + block.pos_x as f32 + size_x * 0.5,
          block.frame_y as f32 * FRAME_SIZE + block.pos_y as f32 + size_y * 0.5,
          block.frame_z as f32 * FRAME_SIZE + block.pos_z as f32 + size_z * 0.5,
        )
        .with_scale(Vec3::new(size_x, size_y, size_z)),
        Pickable::default(),
      ))
      .observe(select_entity)
      .observe(swap_to_selected_material)
      .observe(swap_to_deselected_material);
  }
}

fn undo_redo(
  keycode: Res<ButtonInput<KeyCode>>,
  mut messages: MessageWriter<ActionMessage>,
) {
  // TODO: Make controls configurable.
  if keycode.pressed(KeyCode::ControlLeft) {
    // TODO: Make controls configurable.
    if keycode.just_pressed(KeyCode::KeyZ) {
      // TODO: Make controls configurable.
      messages.write(if keycode.pressed(KeyCode::ShiftLeft) {
        ActionMessage::Redo
      } else {
        ActionMessage::Undo
      });
    }

    // TODO: Make controls configurable.
    if keycode.just_pressed(KeyCode::KeyY) {
      messages.write(ActionMessage::Redo);
    }
  }
}

fn reload_blueprint(
  keycode: Res<ButtonInput<KeyCode>>,
  mut action_history: ResMut<ActionHistory>,
  asset_server: Res<AssetServer>,
  blueprint: Res<LoadedBlueprint>,
) {
  // TODO: Make controls configurable.
  if keycode.pressed(KeyCode::ControlLeft)
    && keycode.just_pressed(KeyCode::KeyR)
  {
    action_history.clear();
    if let Some(path) = blueprint.path() {
      asset_server.reload(path);
    }
  }
}

fn orbit(
  mut camera: Single<&mut Transform, With<Camera3d>>,
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

  // TODO: Make controls configurable.
  if mouse_buttons.pressed(MouseButton::Middle) {
    let delta = mouse_motion.delta;

    // TODO: Make controls configurable.
    if key_input.pressed(KeyCode::ShiftLeft)
      || key_input.pressed(KeyCode::ShiftRight)
    {
      let x = -camera.right() * delta.x * 0.1;
      let z = camera.up() * delta.y * 0.1;
      camera_settings.target += x + z;
    } else {
      let delta_pitch = -delta.y * camera_settings.pitch_speed;
      let delta_yaw = -delta.x * camera_settings.yaw_speed;

      let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);

      let pitch = (pitch + delta_pitch).clamp(
        camera_settings.pitch_range.start,
        camera_settings.pitch_range.end,
      );
      let yaw = yaw + delta_yaw;
      camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    }
  }

  camera.translation =
    camera_settings.target - camera.forward() * camera_settings.orbit_distance;
}
