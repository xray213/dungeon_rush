use std::fs;
use std::fs::File;
use std::path::Path;

use bevy::prelude::*;
use bevy::utils::HashMap;
use ron::de;
use serde::Deserialize;

use crate::status::GameStatue;

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
    pub font_handle: Handle<Font>,
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
        app.add_system_set(
            SystemSet::on_enter(GameStatue::Load).with_system(load_material.system()),
        );
    }
}

fn load_material(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_handles: ResMut<TextureHandles>,
    mut atlas_handles: ResMut<AtlasHandles>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    mut font_handles: ResMut<FontHandles>,
    mut audio_handles: ResMut<AudioHandles>,
    mut state: ResMut<State<GameStatue>>,
) {
    let ron_path = Path::new("./assets/drawable/");
    let paths = fs::read_dir(&ron_path).unwrap();
    let names = paths
        .filter_map(|entry| {
            entry.ok().and_then(|f| match f.path().extension() {
                Some(ext) if ext.eq("ron") => f.path().to_str().map(|s| String::from(s)),
                _ => None,
            })
        })
        .collect::<Vec<String>>();
    for name in names.iter() {
        let f = File::open(name).unwrap();
        let ron: AtlasInfo = de::from_reader(f).unwrap();
        let texture_handle = asset_server.load(format!("drawable/{}.png", ron.name).as_str());
        if ron.column == 1 {
            texture_handles.handles.insert(ron.name, texture_handle);
        } else {
            let atlas_handle = texture_atlas.add(TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(ron.width as f32, ron.height as f32),
                ron.column as usize,
                ron.row as usize,
            ));
            atlas_handles.handles.insert(ron.name, atlas_handle);
        }
    }
    //加载字体
    font_handles.font_handle = asset_server.load("font/m5x7.ttf");

    //加载声音
    let audio_path = "./assets/audio/";
    let audio_paths = fs::read_dir(audio_path).unwrap();
    let audio_names = audio_paths
        .filter_map(|entity| {
            entity
                .ok()
                .and_then(|f| f.path().to_str().map(|s| String::from(s)))
        })
        .collect::<Vec<String>>();
    for audio_name in audio_names.iter() {
        let p = Path::new(audio_name);
        if let Some(_ext) = p.extension() {
            let key = p.file_stem().unwrap().to_str().unwrap().to_string();
            let audio_handle = asset_server
                .load(format!("audio/{}", p.file_name().unwrap().to_str().unwrap()).as_str());
            audio_handles.handles.insert(key, audio_handle);
        }
    }
    let _ = state.replace(GameStatue::MainMenu);
}
