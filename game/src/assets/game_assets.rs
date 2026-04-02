use bevy::reflect::StructInfo;
use rust_embed::{Embed, EmbeddedFile};

#[derive(Embed)]
#[folder = "assets/"]
pub struct GameAssets;

// TODO: implement abstractions for asset grabbing


impl GameAssets {
    // TODO: using generics as struct export
    pub fn as_json(path: &str) {
        let asset = GameAssets::get(path).unwrap();

    }
}