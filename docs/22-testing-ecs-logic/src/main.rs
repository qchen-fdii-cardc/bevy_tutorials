use bevy::prelude::*;

#[derive(Component)]
struct Chapter22TestingEcsLogicMarker;

#[derive(Resource, Default)]
struct Chapter22TestingEcsLogicCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter22TestingEcsLogicCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter22TestingEcsLogicMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter22TestingEcsLogicCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter22TestingEcsLogicCounter>, query: Query<&Chapter22TestingEcsLogicMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter22TestingEcsLogicCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter22TestingEcsLogicMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter22TestingEcsLogicCounter>().value, 1);
    }
}
