use crate::elements::elements::{BehaviorScript, Element, ElementKind, ElementRegistry};
use bevy::app::{Plugin, Update};
use bevy::ecs::system::NonSendMarker;
use bevy::prelude::{Res, ResMut, Resource};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, simulation_system);
    }
}

#[derive(Resource)]
pub struct SandWorld {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<u8>,
    pub updated: Vec<bool>,
}

impl SandWorld {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            cells: vec![0; (width * height) as usize],
            updated: vec![false; (width * height) as usize],
        }
    }

    pub fn get_cell(&self, x: u32, y: u32) -> u8 {
        self.cell_index(x, y)
            .map(|idx| self.cells[idx])
            .unwrap_or(0)
    }

    pub fn set_cell(&mut self, x: u32, y: u32, id: u8) {
        if let Some(idx) = self.cell_index(x, y) {
            self.cells[idx] = id;
            self.updated[idx] = true;
        }
    }

    pub fn swap_cells(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        let Some(idx1) = self.cell_index(x1, y1) else {
            return;
        };
        let Some(idx2) = self.cell_index(x2, y2) else {
            return;
        };

        self.cells.swap(idx1, idx2);
        self.updated[idx1] = true;
        self.updated[idx2] = true;
    }

    pub fn is_empty(&self, x: u32, y: u32) -> bool {
        self.get_cell(x, y) == 0
    }

    fn cell_index(&self, x: u32, y: u32) -> Option<usize> {
        if x < self.width && y < self.height {
            Some((y * self.width + x) as usize)
        } else {
            None
        }
    }
}

pub fn simulation_system(
    mut world: ResMut<SandWorld>,
    registry: Res<ElementRegistry>,
    _main_thread: NonSendMarker,
) {
    world.updated.fill(false);

    for y in (0..world.height).rev() {
        for x in 0..world.width {
            let id = world.get_cell(x, y);
            if id == 0 || world.updated[(y * world.width + x) as usize] {
                continue;
            }

            let element = &registry.elements[id as usize];

            match element.kind {
                ElementKind::Powder => update_powder(x, y, &mut world),
                ElementKind::Liquid => update_liquid(x, y, &mut world),
                _ => {}
            }

            match &element.behavior {
                BehaviorScript::Rust(func) => {
                    func(x, y, &mut world);
                }
                BehaviorScript::JavaScript(func_name) => {
                    crate::script::js_executor::js_executor::execute_behavior(
                        func_name, x, y, &mut world, &registry,
                    );
                }
                BehaviorScript::None => {}
            }
        }
    }
}

fn update_powder(x: u32, y: u32, world: &mut SandWorld) {
    if y + 1 < world.height {
        if world.is_empty(x, y + 1) {
            world.swap_cells(x, y, x, y + 1);
        } else if x > 0 && world.is_empty(x - 1, y + 1) {
            world.swap_cells(x, y, x - 1, y + 1);
        } else if x + 1 < world.width && world.is_empty(x + 1, y + 1) {
            world.swap_cells(x, y, x + 1, y + 1);
        }
    }
}

fn update_liquid(x: u32, y: u32, world: &mut SandWorld) {
    if y + 1 < world.height && world.is_empty(x, y + 1) {
        world.swap_cells(x, y, x, y + 1);
    } else {
        let dir = if x % 2 == 0 { 1 } else { -1 };
        let target_x = x as i32 + dir;
        if target_x >= 0 && target_x < world.width as i32 && world.is_empty(target_x as u32, y) {
            world.swap_cells(x, y, target_x as u32, y);
        }
    }
}
