use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{Transform, TransformBundle},
    ecs::{Entity, World, WorldExt},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        Camera, ImageFormat, RenderingBundle, SpriteSheet, SpriteSheetFormat, Texture,
    },
    ui::{Anchor, RenderUi, TtfFormat, UiBundle, UiText, UiTransform},
    utils::application_root_dir,
};

mod systems;
pub const ARENA_HEIGHT: f32 = 200.0;
pub const ARENA_WIDTH: f32 = 200.0;

pub struct PiText {
    pub text: Entity,
}
struct MyState;
impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_camera(world);
        initialize_pi(world);
        let sprite_sheet = load_sprite_sheet(world);
        world
            .create_entity()
            .with(systems::SpriteSheetComponent { sprite_sheet })
            .build();
    }
}
fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/circles.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/circles.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_pi(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "fonts/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let text_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        0.,
        0.,
        1.,
        200.,
        50.,
    );

    let text = world
        .create_entity()
        .with(text_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    world.insert(PiText { text });
}
fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let input_bundle = InputBundle::<StringBindings>::new();
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(
            systems::SpawnCirclesSystem::default(),
            "spawn_circle_system",
            &[],
        );

    let mut game = Application::new(assets_dir, MyState, game_data)?;
    game.run();

    Ok(())
}
