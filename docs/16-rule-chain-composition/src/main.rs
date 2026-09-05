use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct Attack {
    power: i32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct Armor {
    reduction: i32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct Health(i32);

#[derive(Component)]
struct Target(Entity);

#[derive(Component)]
struct Dead;

#[derive(Resource, Default)]
struct BattleLog(Vec<String>);

fn main() {
    let mut app = App::new();
    app.init_resource::<BattleLog>()
        .add_systems(
            Update,
            (
                begin_attack,
                apply_armor_reduction,
                apply_health_change,
                resolve_death,
                report_battle_state,
            )
                .chain(),
        );

    let player = app
        .world_mut()
        .spawn((Attack { power: 10 }, Target(Entity::PLACEHOLDER)))
        .id();
    let enemy = app
        .world_mut()
        .spawn((Health(12), Armor { reduction: 4 }, Target(player)))
        .id();
    app.world_mut().entity_mut(player).insert(Target(enemy));
    app.update();

    for line in &app.world().resource::<BattleLog>().0 {
        println!("{line}");
    }
}

fn begin_attack(mut attackers: Query<(&mut Attack, &Target)>) {
    for (mut attack, _) in &mut attackers {
        attack.power = attack.power.max(1);
    }
}

fn apply_armor_reduction(
    mut attackers: Query<(&Attack, &Target)>,
    defenders: Query<(&Health, &Armor)>,
    mut log: ResMut<BattleLog>,
) {
    for (attack, target) in &mut attackers {
        if let Ok((health, armor)) = defenders.get(target.0) {
            let damage = (attack.power - armor.reduction).max(0);
            log.0.push(format!(
                "raw attack {attack:?} hits target with hp {} and armor {} -> damage {}",
                health.0,
                armor.reduction,
                damage
            ));
        }
    }
}

fn apply_health_change(
    mut attackers: Query<(&Attack, &Target)>,
    mut defenders: Query<(&mut Health, &Armor)>,
) {
    for (attack, target) in &mut attackers {
        if let Ok((mut health, armor)) = defenders.get_mut(target.0) {
            let damage = (attack.power - armor.reduction).max(0);
            let taken = damage.min(health.0);
            health.0 -= taken;
        }
    }
}

fn resolve_death(
    mut commands: Commands,
    q: Query<(Entity, &Health)>,
    mut log: ResMut<BattleLog>,
) {
    for (entity, health) in &q {
        if health.0 <= 0 {
            commands.entity(entity).insert(Dead);
            log.0.push(format!("entity {:?} died", entity));
        }
    }
}

fn report_battle_state(q: Query<(Entity, &Health, Option<&Dead>)>, mut log: ResMut<BattleLog>) {
    for (entity, health, dead) in &q {
        let state = if dead.is_some() { "dead" } else { "alive" };
        log.0.push(format!("entity {:?} is {state} with hp {}", entity, health.0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armor_reduces_damage_before_health_changes() {
        let mut app = App::new();
        app.init_resource::<BattleLog>()
            .add_systems(
                Update,
                (
                    begin_attack,
                    apply_armor_reduction,
                    apply_health_change,
                    resolve_death,
                    report_battle_state,
                )
                    .chain(),
            );

        let attacker = app
            .world_mut()
            .spawn((Attack { power: 10 }, Target(Entity::PLACEHOLDER)))
            .id();
        let defender = app
            .world_mut()
            .spawn((Health(12), Armor { reduction: 4 }, Target(attacker)))
            .id();
        app.world_mut().entity_mut(attacker).insert(Target(defender));

        app.update();

        assert_eq!(app.world().get::<Health>(defender).unwrap().0, 6);
    }

    #[test]
    fn death_is_resolved_after_health_drops_to_zero() {
        let mut app = App::new();
        app.init_resource::<BattleLog>()
            .add_systems(
                Update,
                (
                    begin_attack,
                    apply_armor_reduction,
                    apply_health_change,
                    resolve_death,
                    report_battle_state,
                )
                    .chain(),
            );

        let attacker = app
            .world_mut()
            .spawn((Attack { power: 8 }, Target(Entity::PLACEHOLDER)))
            .id();
        let defender = app
            .world_mut()
            .spawn((Health(3), Armor { reduction: 1 }, Target(attacker)))
            .id();
        app.world_mut().entity_mut(attacker).insert(Target(defender));

        app.update();

        assert!(app.world().get::<Dead>(defender).is_some());
        assert_eq!(app.world().get::<Health>(defender).unwrap().0, 0);
    }
}
