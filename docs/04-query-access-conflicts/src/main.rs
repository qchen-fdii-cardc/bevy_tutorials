use bevy::prelude::*;

#[derive(Component, Debug, PartialEq)]
struct Health(i32);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Resource, Default)]
struct CombatLog(Vec<String>);

fn main() {
    let mut app = App::new();
    app.init_resource::<CombatLog>().add_systems(
        Update,
        (heal_players, damage_enemies, report_health).chain(),
    );

    app.world_mut().spawn((Player, Health(8)));
    app.world_mut().spawn((Enemy, Health(5)));
    app.update();

    for entry in &app.world().resource::<CombatLog>().0 {
        println!("{entry}");
    }
}

fn heal_players(mut players: Query<&mut Health, With<Player>>) {
    for mut health in &mut players {
        health.0 += 2;
    }
}

fn damage_enemies(mut enemies: Query<&mut Health, With<Enemy>>) {
    for mut health in &mut enemies {
        health.0 -= 3;
    }
}

fn report_health(health: Query<&Health>, mut log: ResMut<CombatLog>) {
    log.0 = health
        .iter()
        .enumerate()
        .map(|(index, health)| format!("entity {index} has {} health", health.0))
        .collect();
}

#[cfg(test)]
type EnemyHealthReader<'w, 's> = Query<'w, 's, &'static Health, With<Enemy>>;

#[cfg(test)]
type PlayerHealthWriter<'w, 's> = Query<'w, 's, &'static mut Health, With<Player>>;

#[cfg(test)]
fn transfer_health(mut queries: ParamSet<(EnemyHealthReader, PlayerHealthWriter)>) {
    let enemy_count = queries.p0().iter().count() as i32;

    for mut health in &mut queries.p1() {
        health.0 += enemy_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filtered_writers_can_update_the_same_component_type() {
        let mut app = App::new();
        app.add_systems(Update, (heal_players, damage_enemies));

        let player = app.world_mut().spawn((Player, Health(8))).id();
        let enemy = app.world_mut().spawn((Enemy, Health(5))).id();

        app.update();

        assert_eq!(app.world().get::<Health>(player), Some(&Health(10)));
        assert_eq!(app.world().get::<Health>(enemy), Some(&Health(2)));
    }

    #[test]
    fn param_set_separates_a_read_phase_from_a_write_phase() {
        let mut app = App::new();
        app.add_systems(Update, transfer_health);

        let player = app.world_mut().spawn((Player, Health(3))).id();
        app.world_mut().spawn((Enemy, Health(5)));
        app.world_mut().spawn((Enemy, Health(7)));

        app.update();

        assert_eq!(app.world().get::<Health>(player), Some(&Health(5)));
    }
}
