use crate::elements::elements::ElementRegistry;
use crate::world::SandWorld;
use bevy::app::{App, Plugin, PreStartup, PreUpdate};
use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::camera::Camera2d;
use bevy::image::Image;
use bevy::log::info;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Query, Res, ResMut, Resource, Sprite, Transform, Window, With};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::PrimaryWindow;
use crate::SCALE;

pub struct WorldRenderPlugin;

impl Plugin for WorldRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, render_world);
        app.add_systems(PreStartup, setup);
    }
}

fn render_world(
    mut world: ResMut<SandWorld>,
    registry: Res<ElementRegistry>,
    mut texture: ResMut<PixelTexture>,
    mut images: ResMut<Assets<Image>>,
) {
    if !texture.needs_full_redraw && world.dirty_cells().is_empty() {
        return;
    }

    let Some(image) = images.get_mut(&texture.handle) else {
        return;
    };

    let Some(image_data) = &mut image.data else {
        return;
    };

    if texture.needs_full_redraw {
        for (idx, id) in world.cells.iter().copied().enumerate() {
            write_cell_pixel(image_data, idx, registry.get_color(id));
        }
        texture.needs_full_redraw = false;
    } else {
        for &idx in world.dirty_cells() {
            let id = world.cells[idx];
            write_cell_pixel(image_data, idx, registry.get_color(id));
        }
    }

    world.clear_dirty();
}

fn write_cell_pixel(image_data: &mut [u8], cell_idx: usize, color: [f32; 4]) {
    let i = cell_idx * 4;
    if i + 3 < image_data.len() {
        image_data[i] = (color[0] * 255.0) as u8;
        image_data[i + 1] = (color[1] * 255.0) as u8;
        image_data[i + 2] = (color[2] * 255.0) as u8;
        image_data[i + 3] = (color[3] * 255.0) as u8;
    }
}

#[derive(Resource)]
struct PixelTexture {
    handle: Handle<Image>,
    needs_full_redraw: bool,
}

pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    query: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);

    let window = query.single();
    let size = window.unwrap().size();

    let size_width = size.x as u32 / SCALE as u32;
    let size_height = size.y as u32 / SCALE as u32;

    info!("{}", size);

    let image = Image::new_fill(
        Extent3d {
            width: size_width,
            height: size_height,
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

    commands.insert_resource(SandWorld::new(size_width, size_height));
    commands.insert_resource(PixelTexture {
        handle,
        needs_full_redraw: true,
    });
}
