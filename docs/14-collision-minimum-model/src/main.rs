use bevy::prelude::*;

#[derive(Component)]
struct Chapter14CollisionMinimumModelMarker;

#[derive(Resource, Default)]
struct Chapter14CollisionMinimumModelCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter14CollisionMinimumModelCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter14CollisionMinimumModelMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter14CollisionMinimumModelCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter14CollisionMinimumModelCounter>, query: Query<&Chapter14CollisionMinimumModelMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter14CollisionMinimumModelCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter14CollisionMinimumModelMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter14CollisionMinimumModelCounter>().value, 1);
    }
}
