// created with ai ( claude )

use crate::elements::elements::ElementRegistry;
use crate::world::SandWorld;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{
    AssetApp, AssetLoader, AssetServer, Assets, Handle, LoadContext, LoadState, embedded_asset,
    io::Reader, load_embedded_asset,
};
use bevy::ecs::system::NonSendMarker;
use bevy::prelude::{error, info, Asset, Res, ResMut, Resource};
use bevy::reflect::TypePath;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};

#[cfg(not(target_arch = "wasm32"))]
const EXTERNAL_MOD_DIR: &str = "mods";

pub struct ModLoaderPlugin;

impl Plugin for ModLoaderPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../../assets/scripts/fire_behavior.js");
        embedded_asset!(app, "../../assets/mods/user_mod.js");

        app.init_asset::<JsAsset>()
            .register_asset_loader(JsAssetLoader)
            .init_resource::<JsLoadQueue>()
            .init_resource::<ModsLoaded>()
            .add_systems(Startup, queue_embedded_js)
            .add_systems(Update, load_queued_js);
    }
}

#[derive(Asset, TypePath, Debug)]
pub struct JsAsset {
    pub source: String,
}

#[derive(Default, TypePath)]
struct JsAssetLoader;

impl AssetLoader for JsAssetLoader {
    type Asset = JsAsset;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let source = String::from_utf8(bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.utf8_error()))?;
        Ok(JsAsset { source })
    }

    fn extensions(&self) -> &[&str] {
        &["js"]
    }
}

#[derive(Resource, Default)]
struct JsLoadQueue {
    embedded: Vec<QueuedEmbeddedJs>,
    loaded: bool,
}

#[derive(Resource, Default)]
pub struct ModsLoaded {
    pub loaded: bool,
}

struct QueuedEmbeddedJs {
    label: &'static str,
    kind: JsKind,
    handle: Handle<JsAsset>,
}

#[derive(Clone, Copy)]
enum JsKind {
    Script,
    Mod,
}

fn queue_embedded_js(mut queue: ResMut<JsLoadQueue>, asset_server: Res<AssetServer>) {
    queue.embedded = vec![
        QueuedEmbeddedJs {
            label: "assets/scripts/fire_behavior.js",
            kind: JsKind::Script,
            handle: load_embedded_asset!(&*asset_server, "../../assets/scripts/fire_behavior.js"),
        },
        QueuedEmbeddedJs {
            label: "assets/mods/user_mod.js",
            kind: JsKind::Mod,
            handle: load_embedded_asset!(&*asset_server, "../../assets/mods/user_mod.js"),
        },
    ];
}

fn load_queued_js(
    mut queue: ResMut<JsLoadQueue>,
    mut mods_loaded: ResMut<ModsLoaded>,
    loaded_js: Res<Assets<JsAsset>>,
    asset_server: Res<AssetServer>,
    mut registry: ResMut<ElementRegistry>,
    mut world: ResMut<SandWorld>,
    _main_thread: NonSendMarker,
) {
    if queue.loaded {
        return;
    }

    for item in &queue.embedded {
        if matches!(
            asset_server.get_load_state(&item.handle),
            Some(LoadState::Failed(_))
        ) {
            error!("Failed to load embedded JS asset: {}", item.label);
            queue.loaded = true;
            mods_loaded.loaded = true;
            return;
        }

        if loaded_js.get(&item.handle).is_none() {
            return;
        }
    }

    for item in &queue.embedded {
        let Some(script) = loaded_js.get(&item.handle) else {
            return;
        };

        if let Err(e) = execute_js_source(
            &mut registry,
            &mut world,
            item.kind,
            item.label,
            &script.source,
        ) {
            error!("Error loading embedded JS asset '{}': {}", item.label, e);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Err(e) = load_external_mods(&mut registry, &mut world, Path::new(EXTERNAL_MOD_DIR)) {
            error!("Error loading external mods: {}", e);
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        error!("External mods are not available on wasm; using embedded JS only");
    }

    queue.loaded = true;
    mods_loaded.loaded = true;
}

#[cfg(not(target_arch = "wasm32"))]
fn load_external_mods(
    registry: &mut ElementRegistry,
    world: &mut SandWorld,
    mod_dir: &Path,
) -> Result<(), String> {
    let entries = match fs::read_dir(mod_dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            info!(
                "External mod directory not found, skipping: {}",
                mod_dir.display()
            );
            return Ok(());
        }
        Err(e) => return Err(format!("Failed to read '{}': {}", mod_dir.display(), e)),
    };

    let mut paths = Vec::new();
    for entry in entries {
        let path = entry
            .map_err(|e| format!("Failed to read mod directory entry: {}", e))?
            .path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("js") {
            paths.push(path);
        }
    }

    paths.sort();

    for path in paths {
        load_external_mod(registry, world, &path)?;
    }

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn load_external_mod(
    registry: &mut ElementRegistry,
    world: &mut SandWorld,
    path: &PathBuf,
) -> Result<(), String> {
    let source = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read external mod '{}': {}", path.display(), e))?;
    execute_js_source(
        registry,
        world,
        JsKind::Mod,
        &format!("external mod {}", path.display()),
        &source,
    )
}

fn execute_js_source(
    registry: &mut ElementRegistry,
    world: &mut SandWorld,
    kind: JsKind,
    label: &str,
    source: &str,
) -> Result<(), String> {
    crate::script::js_executor::JsExecutor::register_function_with_world(
        source,
        registry,
        Some(world),
    )?;

    match kind {
        JsKind::Script => info!("Loaded embedded script: {}", label),
        JsKind::Mod => info!("Loaded mod: {}", label),
    }

    Ok(())
}
