use bevy::prelude::*;

#[derive(Component)]
struct Chapter07FixedUpdateMarker;

#[derive(Resource, Default)]
struct Chapter07FixedUpdateCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter07FixedUpdateCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter07FixedUpdateMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter07FixedUpdateCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter07FixedUpdateCounter>, query: Query<&Chapter07FixedUpdateMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter07FixedUpdateCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter07FixedUpdateMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter07FixedUpdateCounter>().value, 1);
    }
}
