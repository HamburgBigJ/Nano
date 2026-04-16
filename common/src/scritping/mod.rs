use std::sync::{Mutex, OnceLock};
use mlua::Lua;

pub mod lua_script_core;


// AI NOTICE did not know hot to make a variable static and only init once
static GLOBAL_LUA: OnceLock<Mutex<Lua>> = OnceLock::new();

pub fn get_lua() -> &'static Mutex<Lua> {
    GLOBAL_LUA.get_or_init(|| Mutex::new(Lua::new()))
}