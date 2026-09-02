use bevy::{
    color::palettes::css::{AQUA, YELLOW},
    prelude::*,
};

#[derive(Resource, Default)]
struct DebugOverlay {
    enabled: bool,
}

#[derive(Resource, Default)]
struct Heartbeat {
    elapsed_seconds: f32,
    reported_second: u64,
}

struct RuntimeDiagnosticsPlugin;

impl Plugin for RuntimeDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugOverlay>()
            .init_resource::<Heartbeat>()
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (toggle_debug_overlay, advance_heartbeat, draw_debug_overlay).chain(),
            );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RuntimeDiagnosticsPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("RuntimeDiagnosticsPlugin initialized its camera.");
}

fn toggle_debug_overlay(keyboard: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<DebugOverlay>) {
    if keyboard.just_pressed(KeyCode::KeyD) {
        overlay.enabled = !overlay.enabled;
        info!(enabled = overlay.enabled, "Debug overlay toggled");
    }
}

fn advance_heartbeat(time: Res<Time>, mut heartbeat: ResMut<Heartbeat>) {
    heartbeat.elapsed_seconds += time.delta_secs();

    let elapsed_second = heartbeat.elapsed_seconds.floor() as u64;
    if elapsed_second > heartbeat.reported_second {
        heartbeat.reported_second = elapsed_second;
        info!(
            elapsed_seconds = heartbeat.elapsed_seconds,
            "Plugin systems are running"
        );
    }
}

fn draw_debug_overlay(mut gizmos: Gizmos, overlay: Res<DebugOverlay>, heartbeat: Res<Heartbeat>) {
    if !overlay.enabled {
        return;
    }

    let radius = 25.0 + 10.0 * (heartbeat.elapsed_seconds * 3.0).sin().abs();
    gizmos.circle_2d(Vec2::ZERO, radius, AQUA);
    gizmos.line_2d(Vec2::new(-220.0, 0.0), Vec2::new(220.0, 0.0), YELLOW);
}
