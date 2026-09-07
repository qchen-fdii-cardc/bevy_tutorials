use std::fs;
use std::path::{Path, PathBuf};

use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings};
use bevy::prelude::*;
use bevy::ui::widget::Text;
use bevy::ui::{AlignItems, Display, FlexDirection, JustifyContent, Node, UiRect, Val};
use image::{ImageBuffer, Rgba};
use rand::Rng;

const HALF_WIDTH: f32 = 420.0;
const HALF_HEIGHT: f32 = 240.0;
const PLAYER_SPEED: f32 = 220.0;
const ENEMY_SPEED: f32 = 80.0;
const PICKUP_RADIUS: f32 = 18.0;
const FLOCK_SEPARATION_RADIUS: f32 = 46.0;
const FLOCK_SEPARATION_WEIGHT: f32 = 2.2;
const FLOCK_WANDER_WEIGHT: f32 = 0.55;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

#[derive(Resource)]
struct Settings {
    volume: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { volume: 0.7 }
    }
}

impl Settings {
    fn load() -> Self {
        let mut settings = Self::default();
        let path = settings_path();
        if let Ok(contents) = fs::read_to_string(path) {
            for line in contents.lines() {
                let mut items = line.splitn(2, '=');
                let key = items.next();
                let value = items.next();
                if let (Some("volume"), Some(raw)) = (key, value) {
                    settings.volume = raw.parse::<f32>().unwrap_or(settings.volume);
                }
            }
        }
        settings
    }
}

fn settings_path() -> PathBuf {
    let mut path = std::env::current_dir().expect("current dir should exist");
    path.push("settings.txt");
    path
}

#[derive(Resource)]
struct GameSession {
    score: u32,
    health: i32,
    wave: u32,
    last_enemy_spawn: f32,
    elapsed: f32,
}

impl Default for GameSession {
    fn default() -> Self {
        Self {
            score: 0,
            health: 5,
            wave: 1,
            last_enemy_spawn: 0.0,
            elapsed: 0.0,
        }
    }
}

#[derive(Resource)]
struct GameAssets {
    player: Handle<Image>,
    enemy: Handle<Image>,
    crystal: Handle<Image>,
    hit: Handle<AudioSource>,
    pickup: Handle<AudioSource>,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Crystal;

#[derive(Component)]
struct Background;

#[derive(Component)]
struct HudRoot;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct StateText;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct FlockWander {
    phase: f32,
    turn_rate: f32,
}

type ResettableEntities<'w, 's> =
    Query<'w, 's, Entity, Or<(With<Player>, With<Enemy>, With<Crystal>, With<Background>)>>;
type BoundedTransforms<'w, 's> =
    Query<'w, 's, &'static mut Transform, Or<(With<Player>, With<Enemy>)>>;
type EnemyMovers<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Transform,
        &'static mut Velocity,
        &'static mut FlockWander,
    ),
    (With<Enemy>, Without<Player>),
>;
type EnemyPositions<'w, 's> = Query<'w, 's, &'static Transform, With<Enemy>>;
type EnemyTransforms<'w, 's> =
    Query<'w, 's, (Entity, &'static Transform), (With<Enemy>, Without<Player>)>;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(Settings::load())
        .insert_resource(GameSession::default())
        .add_systems(Startup, setup_game)
        .add_systems(OnEnter(GameState::Playing), reset_game)
        .add_systems(
            Update,
            (
                handle_menu_input,
                handle_game_over_input,
                update_hud,
                camera_follow,
                apply_game_state,
            )
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            (
                player_move,
                enemy_ai,
                crystal_pickup,
                enemy_damage,
                spawn_wave,
                keep_in_bounds,
            )
                .chain(),
        )
        .run();
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    ensure_generated_assets();

    commands.spawn((Camera2d, Camera::default()));

    let bg = asset_server.load("generated/background.png");
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, -10.0).with_scale(Vec3::splat(1.4)),
        Sprite {
            image: bg,
            custom_size: Some(Vec2::new(1200.0, 700.0)),
            ..default()
        },
        Background,
    ));

    let assets = GameAssets {
        player: asset_server.load("generated/player.png"),
        enemy: asset_server.load("generated/enemy.png"),
        crystal: asset_server.load("generated/crystal.png"),
        hit: asset_server.load("generated/hit.wav"),
        pickup: asset_server.load("generated/pickup.wav"),
    };
    spawn_menu_entities(&mut commands, &assets);
    commands.insert_resource(assets);

    commands
        .spawn((
            HudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                TextFont::from_font_size(30.0),
                TextColor::WHITE,
                ScoreText,
            ));
            parent.spawn((
                Text::new(""),
                TextFont::from_font_size(22.0),
                TextColor(Color::srgba(0.9, 0.9, 0.8, 1.0)),
                StateText,
            ));
        });

    let _ = (&mut meshes, &mut materials);
}

fn spawn_menu_entities(commands: &mut Commands, assets: &GameAssets) {
    commands.spawn((
        Transform::from_xyz(-130.0, 40.0, 1.0).with_scale(Vec3::splat(2.0)),
        Sprite {
            image: assets.player.clone(),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Player,
    ));
    commands.spawn((
        Transform::from_xyz(130.0, -40.0, 1.0).with_scale(Vec3::splat(2.0)),
        Sprite {
            image: assets.enemy.clone(),
            ..default()
        },
        Enemy,
    ));
}

fn reset_game(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut session: ResMut<GameSession>,
    entities: ResettableEntities,
) {
    session.score = 0;
    session.health = 5;
    session.wave = 1;
    session.last_enemy_spawn = 0.0;
    session.elapsed = 0.0;

    for entity in &entities {
        commands.entity(entity).despawn();
    }

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 2.0),
        Sprite {
            image: assets.player.clone(),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Player,
        Collider,
        Velocity { x: 0.0, y: 0.0 },
    ));
    spawn_crystal(&mut commands, &assets, 160.0, 100.0);
    for i in 0..2 {
        let x = -220.0 + i as f32 * 180.0;
        let y = 150.0 - i as f32 * 120.0;
        spawn_enemy(&mut commands, &assets, x, y);
    }
}

fn spawn_crystal(commands: &mut Commands, assets: &Res<GameAssets>, x: f32, y: f32) {
    commands.spawn((
        Transform::from_xyz(x, y, 1.0),
        Sprite {
            image: assets.crystal.clone(),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Crystal,
        Collider,
    ));
}

fn spawn_enemy(commands: &mut Commands, assets: &Res<GameAssets>, x: f32, y: f32) {
    let mut rng = rand::rng();
    commands.spawn((
        Transform::from_xyz(x, y, 1.0),
        Sprite {
            image: assets.enemy.clone(),
            custom_size: Some(Vec2::new(28.0, 28.0)),
            ..default()
        },
        Enemy,
        Collider,
        Velocity { x: 0.0, y: 0.0 },
        FlockWander {
            phase: rng.random_range(0.0..std::f32::consts::TAU),
            turn_rate: rng.random_range(0.65..1.25),
        },
    ));
}

fn handle_menu_input(
    state: Res<State<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *state.get() == GameState::Menu && keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
}

fn handle_game_over_input(
    state: Res<State<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *state.get() == GameState::GameOver && keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
}

fn apply_game_state(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    session: Res<GameSession>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if *state.get() == GameState::Playing && session.health <= 0 {
        next_state.set(GameState::GameOver);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}

fn player_move(
    time: Res<Time>,
    state: Res<State<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    if *state.get() != GameState::Playing {
        return;
    }

    let move_x =
        (keyboard.pressed(KeyCode::KeyD) as i8 - keyboard.pressed(KeyCode::KeyA) as i8) as f32;
    let move_y =
        (keyboard.pressed(KeyCode::KeyW) as i8 - keyboard.pressed(KeyCode::KeyS) as i8) as f32;
    let dir = Vec2::new(move_x, move_y);
    let delta = time.delta_secs_f64() as f32;

    for (mut transform, mut velocity) in &mut query {
        if dir.length_squared() > 0.0 {
            let dir = dir.normalize();
            velocity.x = dir.x * PLAYER_SPEED;
            velocity.y = dir.y * PLAYER_SPEED;
            transform.translation.x += dir.x * PLAYER_SPEED * delta;
            transform.translation.y += dir.y * PLAYER_SPEED * delta;
        } else {
            velocity.x = 0.0;
            velocity.y = 0.0;
        }
    }
}

fn enemy_ai(
    time: Res<Time>,
    state: Res<State<GameState>>,
    mut queries: ParamSet<(EnemyPositions, EnemyMovers)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if *state.get() != GameState::Playing {
        return;
    }
    let Ok(player) = player_query.single() else {
        return;
    };
    let player_pos = player.translation.truncate();
    let positions: Vec<Vec2> = queries
        .p0()
        .iter()
        .map(|transform| transform.translation.truncate())
        .collect();
    let delta = time.delta_secs_f64() as f32;

    for (mut transform, mut velocity, mut wander) in &mut queries.p1() {
        let position = transform.translation.truncate();
        let seek_direction = (player_pos - position).normalize_or_zero();
        let mut separation = Vec2::ZERO;

        for other_position in &positions {
            let offset = position - *other_position;
            let distance = offset.length();
            if distance > 0.001 && distance < FLOCK_SEPARATION_RADIUS {
                let strength = 1.0 - distance / FLOCK_SEPARATION_RADIUS;
                separation += offset / distance * strength;
            }
        }

        wander.phase = (wander.phase + wander.turn_rate * delta) % std::f32::consts::TAU;
        let wander_direction = Vec2::new(wander.phase.cos(), wander.phase.sin());
        let flock_direction = (seek_direction
            + separation * FLOCK_SEPARATION_WEIGHT
            + wander_direction * FLOCK_WANDER_WEIGHT)
            .normalize_or_zero();
        if flock_direction.length_squared() > 0.0 {
            velocity.x = flock_direction.x * ENEMY_SPEED;
            velocity.y = flock_direction.y * ENEMY_SPEED;
            transform.translation.x += velocity.x * delta;
            transform.translation.y += velocity.y * delta;
        }
    }
}

fn crystal_pickup(
    state: Res<State<GameState>>,
    mut commands: Commands,
    players: Query<&Transform, With<Player>>,
    crystals: Query<(Entity, &Transform), With<Crystal>>,
    assets: Res<GameAssets>,
    mut session: ResMut<GameSession>,
) {
    if *state.get() != GameState::Playing {
        return;
    }
    let Ok(player) = players.single() else {
        return;
    };
    let player_pos = player.translation.truncate();
    for (entity, crystal_transform) in &crystals {
        if player_pos.distance(crystal_transform.translation.truncate()) < PICKUP_RADIUS + 12.0 {
            session.score += 10;
            commands_play_sound(&mut commands, &assets.pickup);
            commands.entity(entity).despawn();
            let mut rng = rand::rng();
            spawn_crystal(
                &mut commands,
                &assets,
                rng.random_range(-300.0..=300.0),
                rng.random_range(-160.0..=160.0),
            );
        }
    }
}

fn enemy_damage(
    state: Res<State<GameState>>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: EnemyTransforms,
    mut session: ResMut<GameSession>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *state.get() != GameState::Playing {
        return;
    }
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    for (entity, enemy_transform) in &enemy_query {
        if player_transform
            .translation
            .distance(enemy_transform.translation)
            < 26.0
        {
            session.health -= 1;
            commands_play_sound(&mut commands, &assets.hit);
            commands.entity(entity).despawn();
            let mut rng = rand::rng();
            let new_x = rng.random_range(-300.0..=300.0);
            let new_y = rng.random_range(-160.0..=160.0);
            spawn_enemy(&mut commands, &assets, new_x, new_y);
            if session.health <= 0 {
                next_state.set(GameState::GameOver);
            }
        }
    }
}

fn spawn_wave(
    state: Res<State<GameState>>,
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    assets: Res<GameAssets>,
    enemy_query: Query<&Enemy>,
) {
    if *state.get() != GameState::Playing {
        return;
    }
    session.elapsed += 1.0 / 60.0;
    session.last_enemy_spawn += 1.0 / 60.0;
    let desired_count = (session.wave as usize).min(8) + 2;
    if enemy_query.iter().count() < desired_count && session.last_enemy_spawn > 2.5 {
        let mut rng = rand::rng();
        spawn_enemy(
            &mut commands,
            &assets,
            rng.random_range(-300.0..=300.0),
            rng.random_range(-160.0..=160.0),
        );
        session.wave += 1;
        session.last_enemy_spawn = 0.0;
    }
}

fn keep_in_bounds(mut query: BoundedTransforms) {
    for mut transform in &mut query {
        transform.translation.x = transform.translation.x.clamp(-HALF_WIDTH, HALF_WIDTH);
        transform.translation.y = transform.translation.y.clamp(-HALF_HEIGHT, HALF_HEIGHT);
    }
}

fn commands_play_sound(commands: &mut Commands, handle: &Handle<AudioSource>) {
    commands.spawn((AudioPlayer(handle.clone()), PlaybackSettings::DESPAWN));
}

fn update_hud(
    state: Res<State<GameState>>,
    session: Res<GameSession>,
    mut score_query: Query<&mut Text, With<ScoreText>>,
    mut state_query: Query<&mut Text, (With<StateText>, Without<ScoreText>)>,
) {
    let score_text = format!(
        "Score: {}  HP: {}  Wave: {}",
        session.score, session.health, session.wave
    );
    if let Ok(mut text) = score_query.single_mut() {
        text.0 = score_text;
    }

    let status = match *state.get() {
        GameState::Menu => "Menu: Press Enter to start",
        GameState::Playing => "Playing: WASD to move, collect crystals, avoid enemies",
        GameState::GameOver => "Game over: Press Enter to restart, Esc to menu",
    };
    if let Ok(mut text) = state_query.single_mut() {
        text.0 = status.to_string();
    }
}

fn camera_follow(
    state: Res<State<GameState>>,
    q: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if *state.get() != GameState::Playing {
        return;
    }
    let Ok(player) = q.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };
    camera_transform.translation.x = player.translation.x * 0.18;
    camera_transform.translation.y = player.translation.y * 0.18;
}

fn ensure_generated_assets() {
    let root = Path::new("assets");
    if !root.exists() {
        fs::create_dir_all(root.join("generated")).unwrap();
    }
    let generated = root.join("generated");
    let _ = fs::create_dir_all(&generated);

    write_rounded_image(&generated.join("player.png"), [58, 248, 196, 255], 64);
    write_rounded_image(&generated.join("enemy.png"), [240, 92, 92, 255], 64);
    write_rounded_image(&generated.join("crystal.png"), [250, 200, 90, 255], 48);
    write_rounded_image(&generated.join("background.png"), [20, 25, 42, 255], 512);
    write_wav_tone(&generated.join("pickup.wav"), 880.0, 0.08);
    write_wav_tone(&generated.join("hit.wav"), 180.0, 0.14);
}

fn write_rounded_image(path: &Path, color: [u8; 4], size: u32) {
    let mut img = ImageBuffer::new(size, size);
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - size as f32 / 2.0;
            let dy = y as f32 - size as f32 / 2.0;
            let dist = (dx * dx + dy * dy).sqrt();
            let radius = size as f32 / 2.0 - 2.0;
            let alpha = if dist <= radius { 255 } else { 0 };
            let mut rgba = color;
            rgba[3] = alpha;
            img.put_pixel(x, y, Rgba(rgba));
        }
    }
    img.save(path).unwrap();
}

fn write_wav_tone(path: &Path, freq: f32, seconds: f32) {
    let sample_rate = 22050usize;
    let frame_count = (sample_rate as f32 * seconds) as usize;
    let mut bytes = Vec::new();
    let data_size = frame_count * 2;
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36u32 + data_size as u32).to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    bytes.extend_from_slice(b"fmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&(sample_rate as u32).to_le_bytes());
    bytes.extend_from_slice(&(sample_rate as u32 * 2).to_le_bytes());
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(&16u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&(data_size as u32).to_le_bytes());

    for i in 0..frame_count {
        let t = i as f32 / sample_rate as f32;
        let sample = (2.0 * std::f32::consts::PI * freq * t).sin();
        let amplitude = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
        bytes.extend_from_slice(&amplitude.to_le_bytes());
    }

    fs::write(path, bytes).unwrap();
}
