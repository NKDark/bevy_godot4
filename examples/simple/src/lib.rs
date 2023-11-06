use bevy::ecs::prelude::*;
use bevy_godot4::prelude::*;
use godot::engine::{resource_loader::CacheMode, ResourceLoader, Sprite2D};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Playing,
}

struct BevyExtensionLibrary;

unsafe impl ExtensionLibrary for BevyExtensionLibrary {
    fn on_level_init(init_level: InitLevel) {
        match init_level {
            InitLevel::Core => {}
            InitLevel::Servers => {
                bevy_godot4::godot::private::class_macros::auto_register_classes(init_level);
                let mut app_builder_func = bevy_godot4::APP_BUILDER_FN.lock().unwrap();
                if app_builder_func.is_none() {
                    *app_builder_func = Some(Box::new(build_app));
                }
            }
            InitLevel::Scene => {}
            InitLevel::Editor => {}
        }
    }
}

#[no_mangle]
unsafe extern "C" fn gdext_rust_init(
    interface_or_get_proc_address: ::godot::sys::InitCompat,
    library: ::godot::sys::GDExtensionClassLibraryPtr,
    init: *mut ::godot::sys::GDExtensionInitialization,
) -> ::godot::sys::GDExtensionBool {
    ::godot::init::__gdext_load_library::<BevyExtensionLibrary>(
        interface_or_get_proc_address,
        library,
        init,
    )
}

fn __static_type_check() {
    let _unused: ::godot::sys::GDExtensionInitializationFunction = Some(gdext_rust_init);
}

fn build_app(app: &mut App) {
    app.add_state::<GameState>()
        .init_resource::<MyAssets>()
        .add_system(spawn_sprite.in_schedule(OnEnter(GameState::Playing)))
        .add_system(
            move_sprite
                .as_physics_system()
                .run_if(in_state(GameState::Playing)),
        );
}

// #[bevy_app]
// fn build_app(app: &mut App) {
//     app.add_state::<GameState>()
//         .init_resource::<MyAssets>()
//         .add_system(spawn_sprite.in_schedule(OnEnter(GameState::Playing)))
//         .add_system(
//             move_sprite
//                 .as_physics_system()
//                 .run_if(in_state(GameState::Playing)),
//         );
// }

#[derive(Resource, Debug)]
pub struct MyAssets {
    pub sprite: ErasedGdResource,
}

impl Default for MyAssets {
    fn default() -> Self {
        let mut resource_loader = ResourceLoader::singleton();
        let sprite = ErasedGdResource::new(
            resource_loader
                .load_ex("sprite.tscn".into())
                .cache_mode(CacheMode::CACHE_MODE_REUSE)
                .done()
                .unwrap(),
        );

        Self { sprite }
    }
}

fn spawn_sprite(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn(
        GodotScene::from_resource(assets.sprite.clone())
            .with_translation2d(Vector2 { x: 200.0, y: 200.0 }),
    );
}

fn move_sprite(mut sprite: Query<&mut ErasedGd>, mut delta: SystemDeltaTimer) {
    if let Ok(mut sprite) = sprite.get_single_mut() {
        let mut sprite = sprite.get::<Sprite2D>();
        let delta = delta.delta_seconds() * 20.0;
        let position = sprite.get_position();

        sprite.set_position(Vector2 {
            x: position.x + delta,
            y: position.y + delta,
        });
    }
}
