use std::fs;
use std::fs::File;
use std::path::Path;

use bevy::prelude::*;
use bevy::render::texture::*;
use bevy::utils::HashMap;
use ron::de;
use serde::Deserialize;

#[derive(Default)]
pub struct AtlasHandles {
    handles: HashMap<String, Handle<TextureAtlas>>,
}

#[derive(Default)]
pub struct TextureHandles {
    handles: HashMap<String, Handle<Texture>>,
}

#[derive(Default)]
pub struct AudioHandles {
    // handles: HashMap<String, Handle<Audio>>,
}

#[derive(Default)]
pub struct FontHandles {
    handles: HashMap<String, Handle<Font>>,
}

#[derive(Deserialize)]
pub struct AtlasInfo {
    name: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    column: u32,
}

//
// fn spawn_title(mut commands: Commands, materials: Res<MaterialResource>) {
//     commands
//         .spawn_bundle(SpriteSheetBundle {
//             texture_atlas: materials.title_texture.clone(),
//             sprite: TextureAtlasSprite {
//                 flip_x: true,
//                 color: Color::rgb(1.0, 0.0, 0.0),
//                 ..Default::default()
//             },
//             transform: Transform::from_scale(Vec3::splat(1.0)),
//             ..Default::default()
//         })
//         .insert(Timer::from_seconds(0.05, true));
// }

fn animate_title(
    texture_atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (_entity, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.len() as u32;
        }
    }
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
        // println!("{}",format!("drawable/{}.png", ron.name));
        if ron.column == 1 {
            texture_handles.handles.insert(ron.name, texture_handle);
        } else {
            let atlas_handle = texture_atlas.add(TextureAtlas::from_grid(texture_handle, Vec2::new(ron.width as f32, ron.height as f32), ron.column as usize, 1));
            atlas_handles.handles.insert(ron.name, atlas_handle);
        }
        println!("{}", texture_handles.handles.len());
        println!("{}", atlas_handles.handles.len());
    }
}