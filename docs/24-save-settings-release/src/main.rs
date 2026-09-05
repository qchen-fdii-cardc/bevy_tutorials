use bevy::prelude::*;

#[derive(Component)]
struct Chapter24SaveSettingsReleaseMarker;

#[derive(Resource, Default)]
struct Chapter24SaveSettingsReleaseCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter24SaveSettingsReleaseCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter24SaveSettingsReleaseMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter24SaveSettingsReleaseCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter24SaveSettingsReleaseCounter>, query: Query<&Chapter24SaveSettingsReleaseMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter24SaveSettingsReleaseCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter24SaveSettingsReleaseMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter24SaveSettingsReleaseCounter>().value, 1);
    }
}
