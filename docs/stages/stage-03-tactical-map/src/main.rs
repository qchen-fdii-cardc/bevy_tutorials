use bevy::prelude::*;

#[derive(Component)]
struct Stage03TacticalMapEntity;

#[derive(Resource, Default)]
struct Stage03TacticalMapState {
    tick: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Stage03TacticalMapState>().add_systems(Update, advance_state);
    app.world_mut().spawn(Stage03TacticalMapEntity);
    app.update();
    println!("tick={}", app.world().resource::<Stage03TacticalMapState>().tick);
}

fn advance_state(mut state: ResMut<Stage03TacticalMapState>, query: Query<&Stage03TacticalMapEntity>) {
    state.tick += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_uses_entity_count() {
        let mut app = App::new();
        app.init_resource::<Stage03TacticalMapState>().add_systems(Update, advance_state);
        app.world_mut().spawn(Stage03TacticalMapEntity);
        app.update();
        assert_eq!(app.world().resource::<Stage03TacticalMapState>().tick, 1);
    }
}
