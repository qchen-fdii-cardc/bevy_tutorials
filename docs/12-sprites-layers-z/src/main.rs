use bevy::prelude::*;

#[derive(Component)]
struct Chapter12SpritesLayersZMarker;

#[derive(Resource, Default)]
struct Chapter12SpritesLayersZCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter12SpritesLayersZCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter12SpritesLayersZMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter12SpritesLayersZCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter12SpritesLayersZCounter>, query: Query<&Chapter12SpritesLayersZMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter12SpritesLayersZCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter12SpritesLayersZMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter12SpritesLayersZCounter>().value, 1);
    }
}
