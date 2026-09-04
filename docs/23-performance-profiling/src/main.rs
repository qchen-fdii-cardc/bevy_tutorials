use bevy::prelude::*;

#[derive(Component)]
struct Chapter23PerformanceProfilingMarker;

#[derive(Resource, Default)]
struct Chapter23PerformanceProfilingCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter23PerformanceProfilingCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter23PerformanceProfilingMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter23PerformanceProfilingCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter23PerformanceProfilingCounter>, query: Query<&Chapter23PerformanceProfilingMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter23PerformanceProfilingCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter23PerformanceProfilingMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter23PerformanceProfilingCounter>().value, 1);
    }
}
