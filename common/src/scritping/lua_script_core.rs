use bevy::app::{App, Plugin, PostStartup, PreStartup, PreUpdate, Startup};
use bevy::log::info;
use bevy::prelude::{IntoScheduleConfigs, Res};
use serde_json::ser::State::First;
use crate::assets::game_assets::{ResourcesRegistry, ScriptSystem};
use crate::CommonAssets;
use crate::scritping::get_lua;

pub struct LuaScriptCorePlugin;

impl Plugin for LuaScriptCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (register_lua_functions, init_lua, startup_lua).chain());
    }
}

fn init_lua(
 script_system: ScriptSystem
) {
    script_system.init_lua();
}


fn startup_lua(
    script_system: ScriptSystem,
) {
    script_system.run_functions("onStart");
}



fn register_lua_functions() {
    let lua = get_lua().lock().unwrap();

    let info_function = lua.create_function(|_, a: String| {
        info!("[Lua] {}", a);
        Ok(a)
    }).expect("failed to register function");

    lua.globals().set("info", info_function);



}