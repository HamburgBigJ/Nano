use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_egui::EguiPlugin;
use rust_embed::RustEmbed;
use crate::assets::game_assets::GameAssetPlugin;
use crate::scritping::lua_script_core::LuaScriptCorePlugin;

pub mod components;
pub mod assets;
mod scritping;

#[derive(RustEmbed)]
#[folder = "assets/"]
#[exclude = "*.txt"]
pub struct CommonAssets;


pub struct CommonGamePlugin;


impl Plugin for CommonGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameAssetPlugin::<CommonAssets>::default());
        app.add_plugins(LuaScriptCorePlugin);
    }
}