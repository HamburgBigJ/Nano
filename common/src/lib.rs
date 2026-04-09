use bevy::app::App;
use bevy::prelude::Plugin;
use rust_embed::Embed;
use crate::assets::game_assets::GameAssetPlugin;

pub mod components;
pub mod assets;


#[derive(Embed)]
#[folder = "assets/"]
#[exclude = "*.txt"]
pub struct CommonAssets;


pub struct CommonGamePlugin;


impl Plugin for CommonGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameAssetPlugin::<CommonAssets>::default());
    }
}