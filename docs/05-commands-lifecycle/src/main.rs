use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
struct DisplayName(String);

#[derive(Component, Debug, Clone, PartialEq, Eq)]
struct Spore {
    generation: u8,
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
struct Lifetime(u8);

#[derive(Resource, Default)]
struct LifecycleLog(Vec<String>);

fn main() {
    let mut app = build_app();

    app.update();
    app.update();
    app.update();

    for entry in &app.world().resource::<LifecycleLog>().0 {
        println!("{entry}");
    }
}

fn build_app() -> App {
    let mut app = App::new();
    app.init_resource::<LifecycleLog>()
        .add_systems(
            Update,
            (seed_initial_spore, age_spores, split_expired_spores),
        )
        .add_systems(PostUpdate, report_population);
    app
}

fn seed_initial_spore(
    spores: Query<(), With<Spore>>,
    mut commands: Commands,
    mut log: ResMut<LifecycleLog>,
) {
    if spores.is_empty() {
        commands.spawn((
            DisplayName("seed-g0".to_owned()),
            Spore { generation: 0 },
            Lifetime(2),
        ));
        log.0.push("queued seed-g0".to_owned());
    }
}

fn age_spores(
    mut spores: Query<(&DisplayName, &mut Lifetime), With<Spore>>,
    mut log: ResMut<LifecycleLog>,
) {
    for (name, mut lifetime) in &mut spores {
        if lifetime.0 > 0 {
            lifetime.0 -= 1;
            log.0.push(format!("aged {} to {}", name.0, lifetime.0));
        }
    }
}

fn split_expired_spores(
    spores: Query<(Entity, &DisplayName, &Spore, &Lifetime)>,
    mut commands: Commands,
    mut log: ResMut<LifecycleLog>,
) {
    for (entity, name, spore, lifetime) in &spores {
        if lifetime.0 == 0 {
            let next_generation = spore.generation + 1;
            commands.spawn((
                DisplayName(format!("{}-a", name.0)),
                Spore {
                    generation: next_generation,
                },
                Lifetime(1),
            ));
            commands.spawn((
                DisplayName(format!("{}-b", name.0)),
                Spore {
                    generation: next_generation,
                },
                Lifetime(1),
            ));
            commands.entity(entity).despawn();
            log.0.push(format!("queued split of {}", name.0));
        }
    }
}

fn report_population(
    spores: Query<(&DisplayName, &Spore, &Lifetime)>,
    mut log: ResMut<LifecycleLog>,
) {
    let mut snapshot: Vec<String> = spores
        .iter()
        .map(|(name, spore, lifetime)| {
            format!("{} [g{}, ttl={}]", name.0, spore.generation, lifetime.0)
        })
        .collect();
    snapshot.sort();

    let description = if snapshot.is_empty() {
        "empty".to_owned()
    } else {
        snapshot.join(", ")
    };
    log.0.push(format!("post-update population: {description}"));
}

#[cfg(test)]
fn population_snapshot(app: &mut App) -> Vec<String> {
    let mut spores = app.world_mut().query::<(&DisplayName, &Spore, &Lifetime)>();
    let mut snapshot: Vec<String> = spores
        .iter(app.world())
        .map(|(name, spore, lifetime)| {
            format!("{} [g{}, ttl={}]", name.0, spore.generation, lifetime.0)
        })
        .collect();
    snapshot.sort();
    snapshot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queued_spawns_become_visible_after_the_update_boundary() {
        let mut app = build_app();

        assert!(population_snapshot(&mut app).is_empty());

        app.update();

        assert_eq!(
            population_snapshot(&mut app),
            vec!["seed-g0 [g0, ttl=2]".to_owned()]
        );
        assert_eq!(
            app.world().resource::<LifecycleLog>().0,
            vec![
                "queued seed-g0".to_owned(),
                "post-update population: seed-g0 [g0, ttl=2]".to_owned()
            ]
        );
    }

    #[test]
    fn despawn_and_spawn_are_observed_after_deferred_commands_flush() {
        let mut app = build_app();
        let parent = app
            .world_mut()
            .spawn((
                DisplayName("parent-g0".to_owned()),
                Spore { generation: 0 },
                Lifetime(0),
            ))
            .id();

        app.update();

        assert_eq!(
            population_snapshot(&mut app),
            vec![
                "parent-g0-a [g1, ttl=1]".to_owned(),
                "parent-g0-b [g1, ttl=1]".to_owned(),
            ]
        );
        assert!(app.world().get::<Spore>(parent).is_none());
        assert_eq!(
            app.world().resource::<LifecycleLog>().0,
            vec![
                "queued split of parent-g0".to_owned(),
                "post-update population: parent-g0-a [g1, ttl=1], parent-g0-b [g1, ttl=1]"
                    .to_owned(),
            ]
        );
    }
}
