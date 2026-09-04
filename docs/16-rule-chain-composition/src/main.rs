use bevy::prelude::*;

#[derive(Component)]
struct Chapter16RuleChainCompositionMarker;

#[derive(Resource, Default)]
struct Chapter16RuleChainCompositionCounter {
    value: u32,
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Chapter16RuleChainCompositionCounter>().add_systems(Update, update_counter);
    app.world_mut().spawn(Chapter16RuleChainCompositionMarker);
    app.update();
    println!("counter={}", app.world().resource::<Chapter16RuleChainCompositionCounter>().value);
}

fn update_counter(mut counter: ResMut<Chapter16RuleChainCompositionCounter>, query: Query<&Chapter16RuleChainCompositionMarker>) {
    counter.value += query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tracks_markers() {
        let mut app = App::new();
        app.init_resource::<Chapter16RuleChainCompositionCounter>().add_systems(Update, update_counter);
        app.world_mut().spawn(Chapter16RuleChainCompositionMarker);
        app.update();
        assert_eq!(app.world().resource::<Chapter16RuleChainCompositionCounter>().value, 1);
    }
}
