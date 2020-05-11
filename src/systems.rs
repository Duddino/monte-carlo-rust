use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::Transform,
    derive::SystemDesc,
    ecs::{
        Component, DenseVecStorage, Entities, HashMapStorage, Join, NullStorage, Read, ReadExpect,
        ReadStorage, System, SystemData, World, WorldExt, WriteStorage,
    },
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::UiText,
};
use rand::Rng;

#[derive(Default, SystemDesc)]
pub struct SpawnCirclesSystem;

impl<'s> System<'s> for SpawnCirclesSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, InsideCircle>,
        WriteStorage<'s, OutsideCircle>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, SpriteSheetComponent>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, crate::PiText>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut inside_circles,
            mut outside_circles,
            mut sprite_renderers,
            sprite_sheets,
            mut ui_text,
            pi_text,
        ): Self::SystemData,
    ) {
        for sprite_sheet in (&sprite_sheets).join() {
            let mut rng = rand::thread_rng();
            let x: f32 = rng.gen_range(-1.0, 1.0);
            let y: f32 = rng.gen_range(-1.0, 1.0);
            let mut transform = Transform::default();
            transform.set_translation_xyz(x * 100.0, y * 100.0, 0.0);
            let entity = entities.build_entity().with(transform, &mut transforms);
            let entity = if x * x + y * y <= 1.0 {
                entity.with(InsideCircle, &mut inside_circles).with(
                    SpriteRender {
                        sprite_sheet: sprite_sheet.sprite_sheet.clone(),
                        sprite_number: 0,
                    },
                    &mut sprite_renderers,
                )
            } else {
                entity.with(OutsideCircle, &mut outside_circles).with(
                    SpriteRender {
                        sprite_sheet: sprite_sheet.sprite_sheet.clone(),
                        sprite_number: 1,
                    },
                    &mut sprite_renderers,
                )
            };
            ui_text.get_mut(pi_text.text).unwrap().text = format!(
                "{:.4}",
                4.0 * inside_circles.count() as f32
                    / (outside_circles.count() + inside_circles.count()) as f32
            );
            entity.build();
        }
    }
}

#[derive(Default)]
pub struct InsideCircle;

impl Component for InsideCircle {
    type Storage = NullStorage<Self>;
}
#[derive(Default)]
pub struct OutsideCircle;

impl Component for OutsideCircle {
    type Storage = NullStorage<Self>;
}

pub struct SpriteSheetComponent {
    pub sprite_sheet: Handle<SpriteSheet>,
}

impl Component for SpriteSheetComponent {
    type Storage = HashMapStorage<Self>;
}
