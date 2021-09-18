use bevy::prelude::*;
use bevy::render::texture::*;


const IMAGE_PATH: [&str; 3] = [
    "drawable\\0x72_DungeonTilesetII_v1_3",
    "drawable\\arrow",
    "drawable\\attack_up",
];

pub struct MaterialResource {
    // title_texture: Handle<ColorMaterial>,
    title_texture: Handle<TextureAtlas>,
}

impl FromWorld for MaterialResource {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let world = world.cell();
        let mut texture_atlas = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let title_handle = asset_server.load("drawable/title.png");

        let atlas = TextureAtlas::from_grid(title_handle, Vec2::new(276.0, 93.0), 29, 1);
        MaterialResource {
            title_texture: texture_atlas.add(atlas),
        }
    }
}

fn spawn_title(mut commands: Commands, materials: Res<MaterialResource>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: materials.title_texture.clone(),
            sprite: TextureAtlasSprite {
                flip_x: true,
                color: Color::rgb(1.0, 0.0, 0.0),
                ..Default::default()
            },
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.05, true));
}

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
        app.init_resource::<MaterialResource>()
            .add_startup_system(spawn_title.system())
            .add_system(animate_title.system());
    }
}
