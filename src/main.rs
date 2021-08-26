use bevy::prelude::*;
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "dungeon rush".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
