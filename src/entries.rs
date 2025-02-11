//! Predefined entry types offered by this library.

use bevy::prelude::*;
use crate::prelude::*;

/// Prelude of predefined entry types.
pub mod prelude {
    pub use super::{PerfUiDefaultEntries, PerfUiAllEntries};

    pub use super::diagnostics::{
        PerfUiEntryFPS,
        PerfUiEntryFrameTime,
        PerfUiEntryFPSWorst,
        PerfUiEntryFrameTimeWorst,
        PerfUiEntryFrameCount,
        PerfUiEntryEntityCount,
    };

    #[cfg(feature = "sysinfo")]
    pub use super::diagnostics::{
        PerfUiEntryCpuUsage,
        PerfUiEntryMemUsage,
    };

    pub use super::time::{
        PerfUiEntryClock,
        PerfUiEntryRunningTime,
        PerfUiEntryFixedTimeStep,
        PerfUiEntryFixedOverstep,
    };
    pub use super::window::{
        PerfUiEntryWindowResolution,
        PerfUiEntryWindowScaleFactor,
        PerfUiEntryWindowMode,
        PerfUiEntryWindowPresentMode,
        PerfUiEntryCursorPosition,
    };
}

pub mod diagnostics;
pub mod time;
pub mod window;

pub(crate) fn predefined_entries_plugin(app: &mut App) {
    app.add_perf_ui_simple_entry::<PerfUiEntryFPS>();
    app.add_perf_ui_simple_entry::<PerfUiEntryFrameTime>();
    app.add_perf_ui_simple_entry::<PerfUiEntryFPSWorst>();
    app.add_perf_ui_simple_entry::<PerfUiEntryFrameTimeWorst>();
    app.add_perf_ui_simple_entry::<PerfUiEntryFrameCount>();
    app.add_perf_ui_simple_entry::<PerfUiEntryEntityCount>();

    #[cfg(feature = "sysinfo")]
    app.add_perf_ui_simple_entry::<PerfUiEntryCpuUsage>();
    #[cfg(feature = "sysinfo")]
    app.add_perf_ui_simple_entry::<PerfUiEntryMemUsage>();

    app.add_perf_ui_simple_entry::<PerfUiEntryClock>();
    app.add_perf_ui_simple_entry::<PerfUiEntryRunningTime>();
    app.add_perf_ui_simple_entry::<PerfUiEntryFixedTimeStep>();
    app.add_perf_ui_simple_entry::<PerfUiEntryFixedOverstep>();

    app.add_perf_ui_simple_entry::<PerfUiEntryWindowResolution>();
    app.add_perf_ui_simple_entry::<PerfUiEntryWindowScaleFactor>();
    app.add_perf_ui_simple_entry::<PerfUiEntryWindowMode>();
    app.add_perf_ui_simple_entry::<PerfUiEntryWindowPresentMode>();
    app.add_perf_ui_simple_entry::<PerfUiEntryCursorPosition>();
}

/// Bundle for a Perf UI with all entry types provided by `iyes_perf_ui`.
///
/// This gives you a simple one-liner to spawn a comprehensive Perf UI!
///
/// ```rust
/// commands.spawn(PerfUiCompleteBundle::default());
/// ```
///
/// If you want to create a Perf UI with specific entries of your choice,
/// just spawn an entity with your desired entries, instead of using
/// this bundle.
///
/// ```rust
/// commands.spawn((
///     PerfUiEntryFPS::default(),
///     PerfUiEntryClock::default(),
///     // ...
/// ));
/// ```
///
/// If you'd like to customize the formatting and presentation, also add
/// the [`PerfUiRoot`] component. If omitted, it will be added automatically
/// with default settings, as it is required by the various entries.
#[allow(missing_docs)]
#[derive(Bundle, Default)]
pub struct PerfUiAllEntries {
    pub fps: PerfUiEntryFPS,
    pub fps_worst: PerfUiEntryFPSWorst,
    pub frametime: PerfUiEntryFrameTime,
    pub frametime_worst: PerfUiEntryFrameTimeWorst,
    pub frame_count: PerfUiEntryFrameCount,
    pub entity_count: PerfUiEntryEntityCount,
    #[cfg(feature = "sysinfo")]
    pub cpu_usage: PerfUiEntryCpuUsage,
    #[cfg(feature = "sysinfo")]
    pub mem_usage: PerfUiEntryMemUsage,
    pub fixed_timestep: PerfUiEntryFixedTimeStep,
    pub fixed_overstep: PerfUiEntryFixedOverstep,
    pub time_running: PerfUiEntryRunningTime,
    pub time_clock: PerfUiEntryClock,
    pub cursor_position: PerfUiEntryCursorPosition,
    pub window_resolution: PerfUiEntryWindowResolution,
    pub window_scale_factor: PerfUiEntryWindowScaleFactor,
    pub window_mode: PerfUiEntryWindowMode,
    pub window_present_mode: PerfUiEntryWindowPresentMode,
}

/// Bundle for a Perf UI with some of the entry types provided by `iyes_perf_ui`.
///
/// This gives you a simple one-liner to spawn a Perf UI!
///
/// It will contain an opinionated curated selection of what I consider to
/// be the most useful of the entries provided by this crate.
///
/// Also see [`PerfUiAllEntries`].
///
/// ```rust
/// commands.spawn(PerfUiBundle::default());
/// ```
///
/// If you want to create a Perf UI with specific entries of your choice,
/// just spawn an entity with your desired entries, instead of using
/// this bundle.
///
/// ```rust
/// commands.spawn((
///     PerfUiEntryFPS::default(),
///     PerfUiEntryClock::default(),
///     // ...
/// ));
/// ```
///
/// If you'd like to customize the formatting and presentation, also add
/// the [`PerfUiRoot`] component. If omitted, it will be added automatically
/// with default settings, as it is required by the various entries.
#[allow(missing_docs)]
#[derive(Bundle, Default)]
pub struct PerfUiDefaultEntries {
    pub fps: PerfUiEntryFPS,
    pub fps_worst: PerfUiEntryFPSWorst,
    pub frametime: PerfUiEntryFrameTime,
    pub frametime_worst: PerfUiEntryFrameTimeWorst,
    pub entity_count: PerfUiEntryEntityCount,
    pub cursor_position: PerfUiEntryCursorPosition,
    pub window_resolution: PerfUiEntryWindowResolution,
}

/// All entries related to framerate.
///
/// ```rust
/// commands.spawn((
///     PerfUiFramerateEntries::default(),
///     // ...
/// ));
/// ```
#[allow(missing_docs)]
#[derive(Bundle, Default)]
pub struct PerfUiFramerateEntries {
    pub fps: PerfUiEntryFPS,
    pub fps_worst: PerfUiEntryFPSWorst,
    pub frametime: PerfUiEntryFrameTime,
    pub frametime_worst: PerfUiEntryFrameTimeWorst,
}

/// All entries related to system diagnostics.
///
/// ```rust
/// commands.spawn((
///     PerfUiSystemEntries::default(),
///     // ...
/// ));
/// ```
#[cfg(feature = "sysinfo")]
#[allow(missing_docs)]
#[derive(Bundle, Default)]
pub struct PerfUiSystemEntries {
    pub cpu_usage: PerfUiEntryCpuUsage,
    pub mem_usage: PerfUiEntryMemUsage,
}

/// All entries related to fixed timestep.
///
/// ```rust
/// commands.spawn((
///     PerfUiFixedTimeEntries::default(),
///     // ...
/// ));
/// ```
#[allow(missing_docs)]
#[derive(Bundle, Default)]
pub struct PerfUiFixedTimeEntries {
    pub fixed_timestep: PerfUiEntryFixedTimeStep,
    pub fixed_overstep: PerfUiEntryFixedOverstep,
}

/// All entries related to windowing.
///
/// ```rust
/// commands.spawn((
///     PerfUiWindowEntries::default(),
///     // ...
/// ));
/// ```
#[allow(missing_docs)]
#[derive(Bundle, Default)]
pub struct PerfUiWindowEntries {
    pub cursor_position: PerfUiEntryCursorPosition,
    pub window_resolution: PerfUiEntryWindowResolution,
    pub window_scale_factor: PerfUiEntryWindowScaleFactor,
    pub window_mode: PerfUiEntryWindowMode,
    pub window_present_mode: PerfUiEntryWindowPresentMode,
}
