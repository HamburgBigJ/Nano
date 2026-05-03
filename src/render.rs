use crate::elements::elements::ElementRegistry;
use crate::world::SandWorld;
use bevy::app::{App, Plugin, PreStartup, PreUpdate, Update};
use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::camera::Camera2d;
use bevy::image::Image;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Res, ResMut, Resource, Sprite, Startup, Transform};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub struct WorldRenderPlugin;

impl Plugin for WorldRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, render_world);
        app.add_systems(PreStartup, setup);
    }
}

fn render_world(
    world: Res<SandWorld>,
    registry: Res<ElementRegistry>,
    texture: Res<PixelTexture>,
    mut images: ResMut<Assets<Image>>,
) {
    let Some(image) = images.get_mut(&texture.handle) else {
        return;
    };

    for y in 0..world.height {
        for x in 0..world.width {
            let id = world.get_cell(x, y);
            let color = registry.get_color(id);
            let i = ((y * world.width + x) * 4) as usize;

            if let Some(image_data) = &mut image.data {
                image_data[i] = (color[0] * 255.0) as u8;
                image_data[i + 1] = (color[1] * 255.0) as u8;
                image_data[i + 2] = (color[2] * 255.0) as u8;
                image_data[i + 3] = (color[3] * 255.0) as u8;
            }
        }
    }
}

#[derive(Resource)]
struct PixelTexture {
    handle: Handle<Image>,
}

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);

    let image = Image::new_fill(
        Extent3d {
            width: crate::W,
            height: crate::H,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[10, 10, 15, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let handle = images.add(image);

    commands.spawn((
        Sprite::from_image(handle.clone()),
        Transform::from_scale(Vec3::splat(crate::SCALE)),
    ));

    commands.insert_resource(SandWorld::new(crate::W, crate::H));
    commands.insert_resource(PixelTexture { handle });
}
