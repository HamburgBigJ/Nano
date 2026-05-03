use crate::world::SandWorld;
use bevy::app::{App, Startup};
use bevy::ecs::system::NonSendMarker;
use bevy::prelude::{Plugin, ResMut, Resource};

pub struct ElementsPlugin;

impl Plugin for ElementsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ElementRegistry::default());
        app.add_plugins(crate::script::mod_loader::ModLoaderPlugin);
        app.add_systems(Startup, register_defaults);
    }
}

fn register_defaults(
    mut elements: ResMut<ElementRegistry>,
    mut world: ResMut<SandWorld>,
    _main_thread: NonSendMarker,
) {
    crate::script::js_executor::js_executor::initialize();

    let empty = Element {
        name: "Empty".to_string(),
        color: [0.0, 0.0, 0.0, 0.0],
        kind: ElementKind::Static,
        behavior: BehaviorScript::None,
    };

    let sand = Element {
        name: "Sand".to_string(),
        color: [1.0, 0.8, 0.2, 1.0],
        kind: ElementKind::Powder,
        behavior: BehaviorScript::None,
    };

    elements.register(empty);
    let sand_id = elements.register(sand);

    let fire = Element {
        name: "Fire".to_string(),
        color: [1.0, 0.3, 0.0, 1.0],
        kind: ElementKind::Gas,
        behavior: BehaviorScript::JavaScript("fireBehavior".to_string()),
    };
    let fire_id = elements.register(fire);

    for x in 140..160 {
        for y in 40..60 {
            world.set_cell(x, y, sand_id);
        }
    }

    for x in 145..155 {
        for y in 30..35 {
            world.set_cell(x, y, fire_id);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    Static,
    Powder,
    Liquid,
    Gas,
}

#[derive(Clone)]
pub enum BehaviorScript {
    None,
    Rust(fn(x: u32, y: u32, world: &mut SandWorld)),
    JavaScript(String),
}

#[derive(Clone)]
pub struct Element {
    pub name: String,
    pub color: [f32; 4],
    pub kind: ElementKind,
    pub behavior: BehaviorScript,
}

#[derive(Resource, Default)]
pub struct ElementRegistry {
    pub elements: Vec<Element>,
}

impl ElementRegistry {
    pub fn register(&mut self, element: Element) -> u8 {
        let id = self.elements.len() as u8;
        self.elements.push(element);
        id
    }

    pub fn get_color(&self, id: u8) -> [f32; 4] {
        self.elements
            .get(id as usize)
            .map(|e| e.color)
            .unwrap_or([0.0, 0.0, 0.0, 0.0])
    }
}
