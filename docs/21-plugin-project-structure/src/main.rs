use bevy::prelude::*;

#[derive(Component)]
struct Chapter21PluginProjectStructureMarker;

#[derive(Resource, Default)]
struct Chapter21PluginProjectStructureCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter21PluginProjectStructureCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter21PluginProjectStructureMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter21PluginProjectStructureCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter21PluginProjectStructureCounter>, query: Query<&Chapter21PluginProjectStructureMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter21PluginProjectStructureCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter21PluginProjectStructureMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter21PluginProjectStructureCounter>().value, 1);
    }
}
