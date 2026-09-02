use bevy::{
    color::palettes::css::{LIME, ORANGE_RED, YELLOW},
    prelude::*,
};

#[derive(Component)]
struct DebugMarker;

#[derive(Resource)]
struct DebugState {
    enabled: bool,
    elapsed_seconds: f32,
    frame_count: u64,
    last_reported_second: u64,
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            enabled: true,
            elapsed_seconds: 0.0,
            frame_count: 0,
            last_reported_second: 0,
        }
    }
}

fn main() {
    App::new()
        .init_resource::<DebugState>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                toggle_debug_tools,
                spawn_debug_marker,
                report_runtime_state,
                draw_debug_gizmos,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((DebugMarker, Transform::from_xyz(-120.0, 70.0, 0.0)));
    commands.spawn((DebugMarker, Transform::from_xyz(80.0, -50.0, 0.0)));
    info!("Debug experiment initialized. Press D to toggle tools and N to add a marker.");
}

fn toggle_debug_tools(keyboard: Res<ButtonInput<KeyCode>>, mut state: ResMut<DebugState>) {
    if keyboard.just_pressed(KeyCode::KeyD) {
        state.enabled = !state.enabled;
        info!(enabled = state.enabled, "Debug tools toggled");
    }
}

fn spawn_debug_marker(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    markers: Query<Entity, With<DebugMarker>>,
) {
    if keyboard.just_pressed(KeyCode::KeyN) {
        let marker_count = markers.iter().count() as f32;
        let position = Vec2::new(-180.0 + marker_count * 45.0, -120.0 + marker_count * 25.0);
        commands.spawn((
            DebugMarker,
            Transform::from_translation(position.extend(0.0)),
        ));
        info!(
            marker_count = marker_count as u64 + 1,
            ?position,
            "Debug marker queued"
        );
    }
}

fn report_runtime_state(
    time: Res<Time>,
    mut state: ResMut<DebugState>,
    entities: Query<Entity>,
    markers: Query<Entity, With<DebugMarker>>,
) {
    state.elapsed_seconds += time.delta_secs();
    state.frame_count += 1;

    let elapsed_second = state.elapsed_seconds.floor() as u64;
    if state.enabled && elapsed_second > state.last_reported_second {
        state.last_reported_second = elapsed_second;
        info!(
            elapsed_seconds = state.elapsed_seconds,
            frame_count = state.frame_count,
            frame_delta_milliseconds = time.delta_secs() * 1_000.0,
            entity_count = entities.iter().count(),
            marker_count = markers.iter().count(),
            "Observable runtime state"
        );
    }
}

fn draw_debug_gizmos(
    mut gizmos: Gizmos,
    state: Res<DebugState>,
    markers: Query<&Transform, With<DebugMarker>>,
) {
    if !state.enabled {
        return;
    }

    gizmos.line_2d(Vec2::new(-320.0, 0.0), Vec2::new(320.0, 0.0), LIME);
    gizmos.line_2d(Vec2::new(0.0, -180.0), Vec2::new(0.0, 180.0), ORANGE_RED);

    for transform in &markers {
        gizmos.circle_2d(transform.translation.truncate(), 12.0, YELLOW);
    }
}
