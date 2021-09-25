use bevy::prelude::*;

use crate::res::{AtlasHandles, TextureHandles};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_ui.system());
    }
}

fn spawn_ui(mut commands: Commands,
            mut texture_handles: ResMut<TextureHandles>,
            mut atlas_handles: ResMut<AtlasHandles>,
            mut texture_atlas: ResMut<Assets<TextureAtlas>>,
            mut materials: ResMut<Assets<ColorMaterial>>,
            asset_server: Res<AssetServer>,
) {
    // let handle=asset_server.load("drawable/wizzard_m_hit_anim.png");
    let h2=texture_handles.handles.get_mut("wizzard_m_hit_anim").unwrap();
    commands.spawn_bundle(
        SpriteBundle {
            material: materials.add(h2.clone().into()),
            ..Default::default()
        }
    );
}