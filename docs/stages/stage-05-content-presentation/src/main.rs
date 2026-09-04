use bevy::prelude::*;

#[derive(Component)]
struct Stage05ContentPresentationEntity;

#[derive(Resource, Default)]
struct Stage05ContentPresentationState {
    tick: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Stage05ContentPresentationState>().add_systems(Update, advance_state);
    app.world_mut().spawn(Stage05ContentPresentationEntity);
    app.update();
    println!("tick={}", app.world().resource::<Stage05ContentPresentationState>().tick);
}

fn advance_state(mut state: ResMut<Stage05ContentPresentationState>, query: Query<&Stage05ContentPresentationEntity>) {
    state.tick += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_uses_entity_count() {
        let mut app = App::new();
        app.init_resource::<Stage05ContentPresentationState>().add_systems(Update, advance_state);
        app.world_mut().spawn(Stage05ContentPresentationEntity);
        app.update();
        assert_eq!(app.world().resource::<Stage05ContentPresentationState>().tick, 1);
    }
}
