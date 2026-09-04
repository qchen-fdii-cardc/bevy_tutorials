use bevy::prelude::*;

#[derive(Component)]
struct Stage01SporeSwarmEntity;

#[derive(Resource, Default)]
struct Stage01SporeSwarmState {
    tick: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Stage01SporeSwarmState>().add_systems(Update, advance_state);
    app.world_mut().spawn(Stage01SporeSwarmEntity);
    app.update();
    println!("tick={}", app.world().resource::<Stage01SporeSwarmState>().tick);
}

fn advance_state(mut state: ResMut<Stage01SporeSwarmState>, query: Query<&Stage01SporeSwarmEntity>) {
    state.tick += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_uses_entity_count() {
        let mut app = App::new();
        app.init_resource::<Stage01SporeSwarmState>().add_systems(Update, advance_state);
        app.world_mut().spawn(Stage01SporeSwarmEntity);
        app.update();
        assert_eq!(app.world().resource::<Stage01SporeSwarmState>().tick, 1);
    }
}
