use bevy::prelude::*;

#[derive(Component)]
struct Stage02TopdownControllerEntity;

#[derive(Resource, Default)]
struct Stage02TopdownControllerState {
    tick: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Stage02TopdownControllerState>().add_systems(Update, advance_state);
    app.world_mut().spawn(Stage02TopdownControllerEntity);
    app.update();
    println!("tick={}", app.world().resource::<Stage02TopdownControllerState>().tick);
}

fn advance_state(mut state: ResMut<Stage02TopdownControllerState>, query: Query<&Stage02TopdownControllerEntity>) {
    state.tick += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_uses_entity_count() {
        let mut app = App::new();
        app.init_resource::<Stage02TopdownControllerState>().add_systems(Update, advance_state);
        app.world_mut().spawn(Stage02TopdownControllerEntity);
        app.update();
        assert_eq!(app.world().resource::<Stage02TopdownControllerState>().tick, 1);
    }
}
