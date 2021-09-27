use bevy::prelude::*;

use crate::res::{AtlasHandles, TextureHandles};
use crate::status::GameStatue;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameStatue::MainMenu).with_system(spawn_ui.system())
        );
    }
}

fn spawn_ui(mut commands: Commands,
            mut texture_handles: ResMut<TextureHandles>,
            mut atlas_handles: ResMut<AtlasHandles>,
            mut texture_atlas: ResMut<Assets<TextureAtlas>>,
            mut materials: ResMut<Assets<ColorMaterial>>,
            asset_server: Res<AssetServer>,
) {
    let handle = texture_handles.handles.get("wizzard_m_hit_anim").unwrap();
    commands.spawn_bundle(
        SpriteBundle {
            material: materials.add(handle.clone().into()),
            ..Default::default()
        }
    );
}