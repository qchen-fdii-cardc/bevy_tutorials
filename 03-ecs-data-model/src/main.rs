use bevy::prelude::*;

#[derive(Component)]
struct Name(&'static str);

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Resource, Default)]
struct SimulationStep(u32);

fn main() {
    let mut app = App::new();
    app.init_resource::<SimulationStep>()
        .add_systems(Update, move_entities);

    app.world_mut().spawn((
        Name("player"),
        Player,
        Position(Vec2::ZERO),
        Velocity(Vec2::new(3.0, 0.0)),
    ));
    app.world_mut().spawn((
        Name("enemy"),
        Enemy,
        Position(Vec2::new(10.0, 5.0)),
        Velocity(Vec2::new(-1.0, 2.0)),
    ));

    app.update();

    let mut positions = app.world_mut().query::<(&Name, &Position)>();
    for (name, position) in positions.iter(app.world()) {
        println!("{} is at {}", name.0, position.0);
    }
}

fn move_entities(mut steps: ResMut<SimulationStep>, mut movers: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut movers {
        position.0 += velocity.0;
    }
    steps.0 += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_rule_only_changes_entities_with_position_and_velocity() {
        let mut app = App::new();
        app.init_resource::<SimulationStep>()
            .add_systems(Update, move_entities);

        let moving_entity = app
            .world_mut()
            .spawn((
                Position(Vec2::new(2.0, 3.0)),
                Velocity(Vec2::new(4.0, -1.0)),
            ))
            .id();
        let stationary_entity = app.world_mut().spawn(Position(Vec2::new(7.0, 8.0))).id();

        app.update();

        assert_eq!(app.world().resource::<SimulationStep>().0, 1);
        assert_eq!(
            app.world().get::<Position>(moving_entity).unwrap().0,
            Vec2::new(6.0, 2.0)
        );
        assert_eq!(
            app.world().get::<Position>(stationary_entity).unwrap().0,
            Vec2::new(7.0, 8.0)
        );
    }
}
