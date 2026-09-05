use bevy::prelude::*;

#[derive(Component)]
struct Chapter06EventsChangeDetectionMarker;

#[derive(Resource, Default)]
struct Chapter06EventsChangeDetectionCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter06EventsChangeDetectionCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter06EventsChangeDetectionMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter06EventsChangeDetectionCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter06EventsChangeDetectionCounter>, query: Query<&Chapter06EventsChangeDetectionMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter06EventsChangeDetectionCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter06EventsChangeDetectionMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter06EventsChangeDetectionCounter>().value, 1);
    }
}
