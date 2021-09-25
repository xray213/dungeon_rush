use std::fs;
use std::fs::File;
use std::path::Path;

use bevy::prelude::*;
use bevy::utils::HashMap;
use ron::de;
use serde::Deserialize;

#[derive(Default)]
pub struct AtlasHandles {
    handles: HashMap<String, Handle<TextureAtlas>>,
}

#[derive(Default)]
pub struct TextureHandles {
    pub handles: HashMap<String, Handle<Texture>>,
}

#[derive(Default)]
pub struct AudioHandles {
    pub handles: HashMap<String, Handle<AudioSource>>,
}

#[derive(Default)]
pub struct FontHandles {
    pub handles: HashMap<String, Handle<Font>>,
}

#[derive(Deserialize)]
pub struct AtlasInfo {
    name: String,
    width: u32,
    height: u32,
    column: u32,
    row: u32,
}

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(load_material.system());
        // .add_system(animate_title.system());
    }
}

fn load_material(mut commands: Commands,
                 asset_server: Res<AssetServer>,
                 mut texture_handles: ResMut<TextureHandles>,
                 mut atlas_handles: ResMut<AtlasHandles>,
                 mut texture_atlas: ResMut<Assets<TextureAtlas>>) {
    let ron_path = Path::new("./assets/drawable/");
    let paths = fs::read_dir(&ron_path).unwrap();
    let names = paths.filter_map(|entry| {
        entry.ok().and_then(|f|
            match f.path().extension() {
                Some(ext) if ext.eq("ron") => f.path().to_str().map(|s| String::from(s)),
                _ => None
            }
        )
    }).collect::<Vec<String>>();
    for name in names.iter() {
        let f = File::open(name).unwrap();
        let ron: AtlasInfo = de::from_reader(f).unwrap();
        let texture_handle = asset_server.load(format!("drawable/{}.png", ron.name).as_str());
        if ron.column == 1 {
            texture_handles.handles.insert(ron.name, texture_handle);
        } else {
            let atlas_handle = texture_atlas.add(TextureAtlas::from_grid(texture_handle, Vec2::new(ron.width as f32, ron.height as f32), ron.column as usize, ron.row as usize));
            atlas_handles.handles.insert(ron.name, atlas_handle);
        }
    }
}