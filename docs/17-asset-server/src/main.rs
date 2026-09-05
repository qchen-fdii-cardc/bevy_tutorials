use bevy::{
    asset::{io::Reader, AssetLoader, AssetPlugin, AssetServer, LoadContext, LoadState},
    prelude::*,
    reflect::TypePath,
    tasks::{IoTaskPool, TaskPoolBuilder},
};

#[derive(Asset, TypePath, Debug, Clone, PartialEq, Eq)]
struct HeroAsset {
    label: String,
    power: u32,
}

#[derive(Default, TypePath)]
struct HeroAssetLoader;

#[derive(Debug)]
enum HeroAssetLoaderError {
    Io(std::io::Error),
    Parse(String),
}

impl From<std::io::Error> for HeroAssetLoaderError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl std::fmt::Display for HeroAssetLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "Could not read hero asset: {err}"),
            Self::Parse(err) => write!(f, "Could not parse hero asset: {err}"),
        }
    }
}

impl std::error::Error for HeroAssetLoaderError {}

impl AssetLoader for HeroAssetLoader {
    type Asset = HeroAsset;
    type Settings = ();
    type Error = HeroAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|err| HeroAssetLoaderError::Parse(err.to_string()))?;

        let mut label = "ember".to_string();
        let mut power = 5u32;

        for line in text.lines() {
            if let Some(value) = line.strip_prefix("name:") {
                label = value.trim().to_string();
            } else if let Some(value) = line.strip_prefix("power:") {
                power = value.trim().parse().unwrap_or(power);
            }
        }

        Ok(HeroAsset { label, power })
    }

    fn extensions(&self) -> &[&str] {
        &["hero"]
    }
}

#[derive(Resource)]
struct AssetReport {
    handle: Option<Handle<HeroAsset>>,
    state: LoadState,
}

impl Default for AssetReport {
    fn default() -> Self {
        Self {
            handle: None,
            state: LoadState::NotLoaded,
        }
    }
}

fn main() {
    IoTaskPool::get_or_init(|| TaskPoolBuilder::new().num_threads(1).build());

    let mut app = App::new();
    app.add_plugins(AssetPlugin::default());
    app.init_asset::<HeroAsset>();
    app.init_asset_loader::<HeroAssetLoader>();
    app.init_resource::<AssetReport>();
    app.add_systems(Startup, queue_asset);
    app.add_systems(Update, refresh_asset_state);

    for _ in 0..8 {
        app.update();
    }

    let report = app.world().resource::<AssetReport>();
    println!(
        "asset handle={} state={:?}",
        report.handle.is_some(),
        report.state
    );
}

fn queue_asset(mut report: ResMut<AssetReport>, asset_server: Res<AssetServer>) {
    if report.handle.is_none() {
        report.handle = Some(asset_server.load("hero.hero"));
    }
}

fn refresh_asset_state(asset_server: Res<AssetServer>, mut report: ResMut<AssetReport>) {
    if let Some(handle) = &report.handle {
        report.state = asset_server.load_state(handle.id());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_report_tracks_a_handle_and_load_state() {
        IoTaskPool::get_or_init(|| TaskPoolBuilder::new().num_threads(1).build());

        let mut app = App::new();
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<HeroAsset>();
        app.init_asset_loader::<HeroAssetLoader>();
        app.init_resource::<AssetReport>();
        app.add_systems(Startup, queue_asset);
        app.add_systems(Update, refresh_asset_state);

        for _ in 0..8 {
            app.update();
        }

        let report = app.world().resource::<AssetReport>();
        assert!(report.handle.is_some());
        assert!(matches!(
            report.state,
            LoadState::Loading | LoadState::Loaded | LoadState::Failed(_)
        ));
    }
}
