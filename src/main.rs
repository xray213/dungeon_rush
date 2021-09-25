use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;

use res::ResourcePlugin;

use crate::res::{AtlasHandles, AudioHandles, FontHandles, TextureHandles};
use crate::ui::UiPlugin;

mod consts;
mod res;
mod ui;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "dungeon rush".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
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
         // mut assets: ResMut<Assets<Texture>>,
         // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // let title_handle: Handle<Texture> = asset_server.load("drawable/ui.png");


    // println!("{:?}",title);
}
