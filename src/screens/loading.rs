use bevy::prelude::*;

use crate::tiles::TileAssets;

use super::Screen;

pub(super) struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);
        app.add_systems(
            Update,
            continue_to_title_screen.run_if(in_state(Screen::Loading).and(all_assets_loaded)),
        );
    }
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((StateScoped(Screen::Loading), Text::new("Loading...")));
}

fn continue_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Game);
}

fn all_assets_loaded(tile_assets: Option<Res<TileAssets>>) -> bool {
    tile_assets.is_some()
}
