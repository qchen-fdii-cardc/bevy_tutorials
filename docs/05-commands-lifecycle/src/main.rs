use bevy::prelude::*;

#[derive(Component)]
struct Chapter05CommandsLifecycleMarker;

#[derive(Resource, Default)]
struct Chapter05CommandsLifecycleCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter05CommandsLifecycleCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter05CommandsLifecycleMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter05CommandsLifecycleCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter05CommandsLifecycleCounter>, query: Query<&Chapter05CommandsLifecycleMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter05CommandsLifecycleCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter05CommandsLifecycleMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter05CommandsLifecycleCounter>().value, 1);
    }
}
