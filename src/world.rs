use crate::elements::elements::{BehaviorScript, ElementKind, ElementRegistry};
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
    updated_frame: Vec<u32>,
    frame: u32,
    dirty: Vec<bool>,
    dirty_cells: Vec<usize>,
}

impl SandWorld {
    pub fn new(width: u32, height: u32) -> Self {
        let len = (width * height) as usize;
        Self {
            width,
            height,
            cells: vec![0; len],
            updated_frame: vec![0; len],
            frame: 0,
            dirty: vec![false; len],
            dirty_cells: Vec::new(),
        }
    }

    pub fn get_cell(&self, x: u32, y: u32) -> u8 {
        self.cell_index(x, y)
            .map(|idx| self.cells[idx])
            .unwrap_or(0)
    }

    pub fn set_cell(&mut self, x: u32, y: u32, id: u8) {
        if let Some(idx) = self.cell_index(x, y) {
            self.mark_updated(idx);
            if self.cells[idx] != id {
                self.cells[idx] = id;
                self.mark_dirty(idx);
            }
        }
    }

    pub fn swap_cells(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        let Some(idx1) = self.cell_index(x1, y1) else {
            return;
        };
        let Some(idx2) = self.cell_index(x2, y2) else {
            return;
        };

        self.swap_indices(idx1, idx2);
    }

    pub fn is_empty(&self, x: u32, y: u32) -> bool {
        self.get_cell(x, y) == 0
    }

    pub fn dirty_cells(&self) -> &[usize] {
        &self.dirty_cells
    }

    pub fn clear_dirty(&mut self) {
        for idx in self.dirty_cells.drain(..) {
            self.dirty[idx] = false;
        }
    }

    fn begin_simulation_frame(&mut self) {
        self.frame = self.frame.wrapping_add(1);
        if self.frame == 0 {
            self.updated_frame.fill(0);
            self.frame = 1;
        }
    }

    fn is_updated(&self, idx: usize) -> bool {
        self.updated_frame[idx] == self.frame
    }

    fn mark_updated(&mut self, idx: usize) {
        self.updated_frame[idx] = self.frame;
    }

    fn mark_dirty(&mut self, idx: usize) {
        if !self.dirty[idx] {
            self.dirty[idx] = true;
            self.dirty_cells.push(idx);
        }
    }

    fn swap_indices(&mut self, idx1: usize, idx2: usize) {
        self.cells.swap(idx1, idx2);
        self.mark_updated(idx1);
        self.mark_updated(idx2);

        if idx1 != idx2 {
            self.mark_dirty(idx1);
            self.mark_dirty(idx2);
        }
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
    world.begin_simulation_frame();
    let width = world.width as usize;

    for y in (0..world.height).rev() {
        for x in 0..world.width {
            let idx = y as usize * width + x as usize;
            let id = world.cells[idx];
            if id == 0 || world.is_updated(idx) {
                continue;
            }

            let Some(element) = registry.elements.get(id as usize) else {
                continue;
            };

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
                    crate::script::js_executor::JsExecutor::execute_behavior(
                        func_name, x, y, &mut world, &registry,
                    );
                }
                BehaviorScript::None => {}
            }
        }
    }
}

fn update_powder(x: u32, y: u32, world: &mut SandWorld) {
    if y + 1 >= world.height {
        return;
    }

    let width = world.width as usize;
    let idx = y as usize * width + x as usize;
    let below = idx + width;

    if world.cells[below] == 0 {
        world.swap_indices(idx, below);
    } else if x > 0 && world.cells[below - 1] == 0 {
        world.swap_indices(idx, below - 1);
    } else if x + 1 < world.width && world.cells[below + 1] == 0 {
        world.swap_indices(idx, below + 1);
    }
}

fn update_liquid(x: u32, y: u32, world: &mut SandWorld) {
    let width = world.width as usize;
    let idx = y as usize * width + x as usize;

    if y + 1 < world.height {
        let below = idx + width;
        if world.cells[below] == 0 {
            world.swap_indices(idx, below);
            return;
        }
    }

    let dir = if x % 2 == 0 { 1 } else { -1 };
    let target_x = x as i32 + dir;
    if target_x >= 0 && target_x < world.width as i32 {
        let target_idx = y as usize * width + target_x as usize;
        if world.cells[target_idx] == 0 {
            world.swap_indices(idx, target_idx);
        }
    }
}
