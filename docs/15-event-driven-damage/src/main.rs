use bevy::prelude::*;

#[derive(Component)]
struct Chapter15EventDrivenDamageMarker;

#[derive(Resource, Default)]
struct Chapter15EventDrivenDamageCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter15EventDrivenDamageCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter15EventDrivenDamageMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter15EventDrivenDamageCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter15EventDrivenDamageCounter>, query: Query<&Chapter15EventDrivenDamageMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter15EventDrivenDamageCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter15EventDrivenDamageMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter15EventDrivenDamageCounter>().value, 1);
    }
}
