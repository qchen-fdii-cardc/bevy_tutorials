use bevy::prelude::*;

#[derive(Component)]
struct Chapter13UiWorldSpaceMarker;

#[derive(Resource, Default)]
struct Chapter13UiWorldSpaceCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter13UiWorldSpaceCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter13UiWorldSpaceMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter13UiWorldSpaceCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter13UiWorldSpaceCounter>, query: Query<&Chapter13UiWorldSpaceMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter13UiWorldSpaceCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter13UiWorldSpaceMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter13UiWorldSpaceCounter>().value, 1);
    }
}
