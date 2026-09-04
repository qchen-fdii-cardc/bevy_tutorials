use bevy::prelude::*;

#[derive(Component)]
struct Chapter20LocalizationConfigMarker;

#[derive(Resource, Default)]
struct Chapter20LocalizationConfigCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter20LocalizationConfigCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter20LocalizationConfigMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter20LocalizationConfigCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter20LocalizationConfigCounter>, query: Query<&Chapter20LocalizationConfigMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter20LocalizationConfigCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter20LocalizationConfigMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter20LocalizationConfigCounter>().value, 1);
    }
}
