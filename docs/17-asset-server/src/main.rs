use bevy::prelude::*;

#[derive(Component)]
struct Chapter17AssetServerMarker;

#[derive(Resource, Default)]
struct Chapter17AssetServerCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter17AssetServerCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter17AssetServerMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter17AssetServerCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter17AssetServerCounter>, query: Query<&Chapter17AssetServerMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter17AssetServerCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter17AssetServerMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter17AssetServerCounter>().value, 1);
    }
}
