mod app;
#[cfg(feature = "assets")]
mod assets;
mod erased_gd;
mod scene;
mod scene_tree;
mod utils;

pub use bevy;
pub use godot;

pub mod prelude {
    pub use super::erased_gd::{ErasedGd, ErasedGdResource};
    pub use super::scene::GodotScene;
    pub use super::scene_tree::SceneTreeRef;
    pub use super::utils::{
        AsPhysicsSystem, AsVisualSystem, GodotPhysicsFrame, GodotVisualFrame, SystemDeltaTimer,
    };
    #[allow(deprecated)]
    pub use bevy::ecs::prelude::{
        apply_state_transition, apply_system_buffers, dbg, error, ignore, info, system_adapter,
        unwrap, warn, Added, AnyOf, Bundle, ChangeTrackers, Changed, Commands, Component,
        Condition, Deferred, DetectChanges, DetectChangesMut, Entity, Event, EventReader,
        EventWriter, Events, FromWorld, In, IntoPipeSystem, IntoSystem, IntoSystemConfig,
        IntoSystemConfigs, IntoSystemSet, IntoSystemSetConfig, IntoSystemSetConfigs, Local, Mut,
        NextState, NonSend, NonSendMut, OnEnter, OnExit, OnUpdate, Or, ParallelCommands, ParamSet,
        Query, QueryState, Ref, RemovedComponents, Res, ResMut, Resource as BevyResource, Schedule,
        Schedules, State, States, System, SystemParamFunction, SystemSet, With, Without, World,
    };
    pub use bevy::ecs::schedule::common_conditions::*;
    pub use bevy::input::prelude::{
        Axis, Gamepad, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType, Gamepads,
        Input as BevyInput, KeyCode, MouseButton, ScanCode, TouchInput, Touches,
    };
    pub use bevy::{
        app::prelude::*, core::prelude::*, hierarchy::prelude::*, log::prelude::*,
        math::prelude::*, reflect::prelude::*, time::prelude::*, transform::prelude::*,
        utils::prelude::*, window::prelude::*, DefaultPlugins, MinimalPlugins,
    };
    pub use bevy_godot4_proc_macros::bevy_app;
    pub use godot::prelude::*;
}

pub use app::{BevyApp, APP_BUILDER_FN};
