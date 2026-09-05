use bevy::prelude::*;

#[derive(Component)]
struct Chapter09StateRunConditionsMarker;

#[derive(Resource, Default)]
struct Chapter09StateRunConditionsCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter09StateRunConditionsCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter09StateRunConditionsMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter09StateRunConditionsCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter09StateRunConditionsCounter>, query: Query<&Chapter09StateRunConditionsMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter09StateRunConditionsCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter09StateRunConditionsMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter09StateRunConditionsCounter>().value, 1);
    }
}
