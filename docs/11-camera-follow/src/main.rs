use bevy::prelude::*;

#[derive(Component)]
struct Chapter11CameraFollowMarker;

#[derive(Resource, Default)]
struct Chapter11CameraFollowCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter11CameraFollowCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter11CameraFollowMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter11CameraFollowCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter11CameraFollowCounter>, query: Query<&Chapter11CameraFollowMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter11CameraFollowCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter11CameraFollowMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter11CameraFollowCounter>().value, 1);
    }
}
