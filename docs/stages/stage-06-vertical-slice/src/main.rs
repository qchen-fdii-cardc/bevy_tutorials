use bevy::prelude::*;

#[derive(Component)]
struct Stage06VerticalSliceEntity;

#[derive(Resource, Default)]
struct Stage06VerticalSliceState {
    tick: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Stage06VerticalSliceState>().add_systems(Update, advance_state);
    app.world_mut().spawn(Stage06VerticalSliceEntity);
    app.update();
    println!("tick={}", app.world().resource::<Stage06VerticalSliceState>().tick);
}

fn advance_state(mut state: ResMut<Stage06VerticalSliceState>, query: Query<&Stage06VerticalSliceEntity>) {
    state.tick += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_uses_entity_count() {
        let mut app = App::new();
        app.init_resource::<Stage06VerticalSliceState>().add_systems(Update, advance_state);
        app.world_mut().spawn(Stage06VerticalSliceEntity);
        app.update();
        assert_eq!(app.world().resource::<Stage06VerticalSliceState>().tick, 1);
    }
}
