use bevy::prelude::*;

#[derive(Component)]
struct Chapter19AudioUiFeedbackMarker;

#[derive(Resource, Default)]
struct Chapter19AudioUiFeedbackCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter19AudioUiFeedbackCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter19AudioUiFeedbackMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter19AudioUiFeedbackCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter19AudioUiFeedbackCounter>, query: Query<&Chapter19AudioUiFeedbackMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter19AudioUiFeedbackCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter19AudioUiFeedbackMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter19AudioUiFeedbackCounter>().value, 1);
    }
}
