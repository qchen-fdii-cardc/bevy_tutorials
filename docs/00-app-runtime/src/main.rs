use bevy::{
    color::palettes::css::{LIME, ORANGE_RED},
    prelude::*,
};

#[derive(Resource)]
struct RuntimeStats {
    elapsed_seconds: f32,
    frame_count: u64,
    last_reported_second: u64,
}

impl Default for RuntimeStats {
    fn default() -> Self {
        Self {
            elapsed_seconds: 0.0,
            frame_count: 0,
            last_reported_second: 0,
        }
    }
}

fn main() {
    App::new()
        .insert_resource(RuntimeStats::default())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (advance_runtime, draw_debug_axes).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Startup completed: the camera entity now exists.");
}

fn advance_runtime(time: Res<Time>, mut stats: ResMut<RuntimeStats>) {
    stats.elapsed_seconds += time.delta_secs();
    stats.frame_count += 1;

    let elapsed_second = stats.elapsed_seconds.floor() as u64;
    if elapsed_second > stats.last_reported_second {
        stats.last_reported_second = elapsed_second;
        info!(
            elapsed_seconds = stats.elapsed_seconds,
            frame_count = stats.frame_count,
            "Update is advancing the RuntimeStats resource"
        );
    }
}

fn draw_debug_axes(mut gizmos: Gizmos, stats: Res<RuntimeStats>) {
    let pulse = 1.0 + 0.25 * (stats.elapsed_seconds * 2.0).sin();

    gizmos.line_2d(Vec2::new(-300.0, 0.0), Vec2::new(300.0, 0.0), LIME);
    gizmos.line_2d(Vec2::new(0.0, -180.0), Vec2::new(0.0, 180.0), ORANGE_RED);
    gizmos.circle_2d(Vec2::ZERO, 30.0 * pulse, Color::WHITE);
}
