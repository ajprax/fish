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
    // TODO:
    //       add a resource with a kdtree or other spatial partitioning for more efficient querying
    //       add a visibility resource calculated from the kdtree
    //       split sac system once they can each access the visibility resource
    //       make more things proportionate to size (e.g. vision) this allows larger numbers in the same size tank without density problems
    //       be more deliberate with creating different kinds of fish
    //       filter clustering behavior based on fish of similar size/color
    //       give fish a turn rate based on size (big turns slow)
    //       make small fish afraid of large fish. remove the distinction between fish and sharks
    //       have a more progressive form of fleeing, avoid large fish like a wall and only flee if they get really close. give flight a duration instead of a distance
    //       make wall systems stronger. with high density of fish, they can force their way through the walls
    //       allow more complex wall configurations e.g. an inner and outer circle
    //       give the fish hunger. make them steer towards fish in proportion to that hunger and their size. remove (eat) fish that get too close
    //       fully implement TIME_RATE. the time rate affects the relative scale of speed vs angles. it may be correct to multiply or divide some of those values as well
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
