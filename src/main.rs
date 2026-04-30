
use bevy::app::{App, Update};
use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::camera::Camera2d;
use bevy::DefaultPlugins;
use bevy::ecs::error::info;
use bevy::image::Image;
use bevy::input::keyboard::Key::Camera;
use bevy::log::info;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Res, ResMut, Resource, Startup, Transform};
use bevy::render::render_resource::encase::private::AlignmentValue;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::Sprite;
use crate::Elements::{Sand, Water};


const W: u32 = 300;
const H: u32 = 180;
const SCALE: f32 = 4.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SandWorld::new(W, H))
        .add_systems(Update, (
            render_world
            // need to add mouse input before render or pixel change
            ))
        .add_systems(Startup, setup)
        .run();


}

#[derive(Clone, PartialEq, Debug)]
enum Elements {
    Sand,
    Stone,
    Water,
    Empty,

}

impl Elements {
    pub fn color(&self) -> [f32; 4] {
        match self {
            Elements::Sand => [0.1, 0.2, 0.3, 1.0], // colors need to be ajusted
            Elements::Stone => [0.1, 0.2, 0.3, 1.0],
            Elements::Water => [0.1, 0.2, 0.3, 1.0],

            Elements::Empty => [0.0, 0.0, 0.0, 0.0],
        }
    }
}

#[derive(Resource)]
struct PixelTexture {
    handle: Handle<Image>
}

#[derive(Resource, Clone)]
struct SandWorld {
    pub height: u32,
    pub width: u32,
    pub cells: Vec<Elements>,
}

impl SandWorld {
    fn new(height: u32, width: u32) -> Self {
        Self {
            height,
            width,
            cells: vec![Elements::Empty; (height * width) as usize],
        }
    }

    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn set_cell(&mut self, x: u32, y: u32, element: Elements) {
        if x >= self.width || y >= self.height {
            return;
        }
        let i = self.index(x, y);
        self.cells[i] = element;
    }

    pub fn get_cell(&self, x: u32, y: u32) -> &Elements {
        if x >= self.width || y >= self.height {
            return &Elements::Empty;
        }
        &self.cells[self.index(x, y)]
    }
}


fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2d);

    let image = Image::new_fill(
        Extent3d {
            width: W,
            height: H,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[10, 10, 15, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    );

    let handle = images.add(image);

    commands.spawn((
        Sprite::from_image(handle.clone()),
        Transform::from_scale(Vec3::splat(SCALE)),
    ));

    let mut world = SandWorld::new(W, H);
    world.set_cell(130, 130, Elements::Sand);
    world.cells.iter().for_each(|cell| {
        info!("{:?}", cell);
    });

    commands.insert_resource(world);
    commands.insert_resource(PixelTexture { handle });



}


fn render_world(
    mut world: ResMut<SandWorld>,
    texture: Res<PixelTexture>,
    mut images: ResMut<Assets<Image>>,
) {
    info!("test");
    let Some(image) = images.get_mut(&texture.handle) else { return };
    let data = &mut image.data.as_mut().unwrap();

    for y in 0..world.height {
        for x in 0..world.width {
            let element = world.get_cell(x, y);
            let color = element.color();
            let i = ((y * world.width + x) * 4) as usize;

            if i + 3 < data.len() {
                data[i]     = (color[0] * 255.0) as u8;
                data[i + 1] = (color[1] * 255.0) as u8;
                data[i + 2] = (color[2] * 255.0) as u8;
                data[i + 3] = (color[3] * 255.0) as u8;
            }
        }
    }
}