#[cfg(feature = "dev")]
use bevy::dev_tools::states::log_transitions;
use bevy::prelude::*;
use loading::LoadingScreenPlugin;

mod loading;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LoadingScreenPlugin,));
        app.init_state::<Screen>();
        app.enable_state_scoped_entities::<Screen>();
        #[cfg(feature = "dev")]
        app.add_systems(Startup, log_transitions::<Screen>);
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Screen {
    #[default]
    Loading,
    Game,
}
