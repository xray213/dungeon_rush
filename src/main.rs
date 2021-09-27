use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
// use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};

use crate::res::{AtlasHandles, AudioHandles, FontHandles, ResourcePlugin, TextureHandles};
use crate::status::GameStatue;
use crate::ui::UiPlugin;

mod res;
mod ui;
mod status;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "dungeon rush".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .add_state(GameStatue::Load)
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        // .add_plugin(AudioPlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(UiPlugin)
        .add_system(exit_on_esc_system.system())
        .init_resource::<TextureHandles>()
        .init_resource::<AtlasHandles>()
        .init_resource::<AudioHandles>()
        .init_resource::<FontHandles>()
        .run();
}

fn setup(mut commands: Commands,
         asset_server: Res<AssetServer>,
         audio: Res<Audio>,
         // mut assets: ResMut<Assets<Texture>>,
         // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    audio.play(asset_server.load("audio/audio.mp3"));
}
