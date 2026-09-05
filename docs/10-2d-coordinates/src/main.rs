use bevy::prelude::*;

#[derive(Component)]
struct Chapter102dCoordinatesMarker;

#[derive(Resource, Default)]
struct Chapter102dCoordinatesCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter102dCoordinatesCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter102dCoordinatesMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter102dCoordinatesCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter102dCoordinatesCounter>, query: Query<&Chapter102dCoordinatesMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter102dCoordinatesCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter102dCoordinatesMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter102dCoordinatesCounter>().value, 1);
    }
}
