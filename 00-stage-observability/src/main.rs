use bevy::{
    color::palettes::css::{LIME, ORANGE_RED, YELLOW},
    prelude::*,
};

#[derive(Component)]
struct DebugOverlay;

#[derive(Component)]
struct OverlayText;

#[derive(Component)]
struct DebugMarker;

#[derive(Resource)]
struct RuntimeStats {
    elapsed_seconds: f32,
    frame_count: u64,
    frames_since_sample: u32,
    frames_per_second: f32,
    frame_delta_milliseconds: f32,
    last_reported_second: u64,
}

impl Default for RuntimeStats {
    fn default() -> Self {
        Self {
            elapsed_seconds: 0.0,
            frame_count: 0,
            frames_since_sample: 0,
            frames_per_second: 0.0,
            frame_delta_milliseconds: 0.0,
            last_reported_second: 0,
        }
    }
}

fn main() {
    App::new()
        .init_resource::<RuntimeStats>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                toggle_debug_overlay,
                spawn_debug_marker,
                collect_runtime_stats,
                update_overlay_text,
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
    commands
        .spawn((
            DebugOverlay,
            Node {
                position_type: PositionType::Absolute,
                top: px(16),
                left: px(16),
                padding: UiRect::all(px(10)),
                border_radius: BorderRadius::all(px(4)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.04, 0.08, 0.88)),
        ))
        .insert((
            OverlayText,
            Text::new("collecting runtime metrics..."),
            TextFont {
                font_size: FontSize::Px(18.0),
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    info!("Stage 0 observability started. Press D to toggle the overlay and N to add a marker.");
}

fn toggle_debug_overlay(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut overlays: Query<&mut Visibility, With<DebugOverlay>>,
) {
    if keyboard.just_pressed(KeyCode::KeyD) {
        for mut visibility in &mut overlays {
            *visibility = match *visibility {
                Visibility::Hidden => Visibility::Inherited,
                _ => Visibility::Hidden,
            };
        }
        info!("Debug overlay visibility toggled");
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

fn collect_runtime_stats(
    time: Res<Time>,
    mut stats: ResMut<RuntimeStats>,
    entities: Query<Entity>,
) {
    let sample_completed = stats.record_frame(time.delta_secs());

    if sample_completed {
        info!(
            elapsed_seconds = stats.elapsed_seconds,
            frames_per_second = stats.frames_per_second,
            frame_delta_milliseconds = stats.frame_delta_milliseconds,
            entity_count = entities.iter().count(),
            "Observable runtime state"
        );
    }
}

impl RuntimeStats {
    fn record_frame(&mut self, delta_seconds: f32) -> bool {
        self.elapsed_seconds += delta_seconds;
        self.frame_count += 1;
        self.frames_since_sample += 1;
        self.frame_delta_milliseconds = delta_seconds * 1_000.0;

        if self.elapsed_seconds < self.last_reported_second as f32 + 1.0 {
            return false;
        }

        self.frames_per_second = self.frames_since_sample as f32;
        self.frames_since_sample = 0;
        self.last_reported_second = self.elapsed_seconds.floor() as u64;
        true
    }
}

fn update_overlay_text(
    stats: Res<RuntimeStats>,
    entities: Query<Entity>,
    markers: Query<Entity, With<DebugMarker>>,
    mut labels: Query<&mut Text, With<OverlayText>>,
) {
    for mut text in &mut labels {
        text.0 = format!(
            "STAGE 0 DEBUG\nframe: {:>5.2} ms\nfps:   {:>5.0}\nentities: {}\nmarkers:  {}\n\n[D] toggle overlay\n[N] spawn marker",
            stats.frame_delta_milliseconds,
            stats.frames_per_second,
            entities.iter().count(),
            markers.iter().count(),
        );
    }
}

fn report_runtime_state(
    stats: Res<RuntimeStats>,
    entities: Query<Entity>,
    markers: Query<Entity, With<DebugMarker>>,
) {
    if stats.elapsed_seconds.floor() as u64 == stats.last_reported_second
        && stats.frames_since_sample == 0
    {
        info!(
            frame_delta_milliseconds = stats.frame_delta_milliseconds,
            frames_per_second = stats.frames_per_second,
            entity_count = entities.iter().count(),
            marker_count = markers.iter().count(),
            "Overlay metrics refreshed"
        );
    }
}

fn draw_debug_gizmos(mut gizmos: Gizmos, markers: Query<&Transform, With<DebugMarker>>) {
    gizmos.line_2d(Vec2::new(-320.0, 0.0), Vec2::new(320.0, 0.0), LIME);
    gizmos.line_2d(Vec2::new(0.0, -180.0), Vec2::new(0.0, 180.0), ORANGE_RED);

    for transform in &markers {
        gizmos.circle_2d(transform.translation.truncate(), 12.0, YELLOW);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_stats_reports_once_after_one_second_of_frames() {
        let mut stats = RuntimeStats::default();

        assert!(!stats.record_frame(0.40));
        assert!(!stats.record_frame(0.59));
        assert!(stats.record_frame(0.01));

        assert_eq!(stats.frame_count, 3);
        assert_eq!(stats.frames_per_second, 3.0);
        assert_eq!(stats.frames_since_sample, 0);
        assert_eq!(stats.last_reported_second, 1);
    }

    #[test]
    fn runtime_stats_carries_frames_into_the_next_sample_window() {
        let mut stats = RuntimeStats::default();

        assert!(stats.record_frame(1.0));
        assert!(!stats.record_frame(0.25));
        assert!(!stats.record_frame(0.25));
        assert!(!stats.record_frame(0.25));
        assert!(stats.record_frame(0.25));

        assert_eq!(stats.frames_per_second, 4.0);
        assert_eq!(stats.last_reported_second, 2);
    }
}
