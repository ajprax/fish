mod components;
mod constants;
mod systems;
mod utils;

use crate::constants::{PERF, USE_CIRLCE};
use bevy::prelude::*;
use iyes_perf_ui::PerfUiPlugin;

use crate::systems::*;
use crate::utils::*;

struct Perf;

impl Plugin for Perf {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(120.0))
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(Startup, perf_startup);
    }
}

fn main() {
    // TODO: give the shark hunger.
    //       make it steer towards fish in proportion to that hunger and their size
    //       remove (eat) fish that get too close
    let mut app = App::new();
    if PERF {
        app.add_plugins(Perf);
    }
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, (fish_startup, sharks_startup))
        .add_systems(
            Update,
            (
                start_fleeing,
                stop_fleeing,
                sac,
                fish_wander,
                sharks_wander,
                if USE_CIRLCE {
                    avoid_circle_walls
                } else {
                    avoid_square_walls
                },
                movement,
                translate,
                rotate,
            )
                .chain(),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run()
}
