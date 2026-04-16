use bevy::app::{App, Plugin, PostStartup, Startup};
use crate::assets::game_assets::{ResourcesRegistry, ScriptSystem};
use crate::CommonAssets;
use crate::scritping::get_lua;

pub struct LuaScriptCorePlugin;

impl Plugin for LuaScriptCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, init_lua);
    }
}

fn init_lua(
 script_system: ScriptSystem
) {
    script_system.init_lua();
}