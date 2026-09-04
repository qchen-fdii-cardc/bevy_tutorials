use bevy::prelude::*;

#[derive(Component)]
struct Chapter18AnimationStateMarker;

#[derive(Resource, Default)]
struct Chapter18AnimationStateCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter18AnimationStateCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter18AnimationStateMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter18AnimationStateCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter18AnimationStateCounter>, query: Query<&Chapter18AnimationStateMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter18AnimationStateCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter18AnimationStateMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter18AnimationStateCounter>().value, 1);
    }
}
