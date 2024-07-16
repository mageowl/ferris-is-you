use macroquad::{
    color::Color,
    math::vec2,
    texture::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D},
};

use crate::frame::{
    object::{Object, ObjectClass},
    property::Property,
    Frame,
};

pub struct Assets {
    obj_ferris: Texture2D,
    obj_wall: Texture2D,
    obj_flag: Texture2D,
    obj_tile: Texture2D,

    verb_is: Texture2D,

    prop_win: Texture2D,
    prop_you: Texture2D,
    prop_stop: Texture2D,
    prop_push: Texture2D,

    noun_ferris: Texture2D,
    noun_wall: Texture2D,
    noun_flag: Texture2D,
    noun_tile: Texture2D,
}

impl Assets {
    pub async fn load() -> Self {
        Self {
            obj_ferris: load_texture("assets/sprites/obj/ferris.png").await.unwrap(),
            obj_wall: load_texture("assets/sprites/obj/wall.png").await.unwrap(),
            obj_flag: load_texture("assets/sprites/obj/tile.png").await.unwrap(),
            obj_tile: load_texture("assets/sprites/obj/tile.png").await.unwrap(),

            verb_is: load_texture("assets/sprites/text/is_on.png").await.unwrap(),

            prop_win: load_texture("assets/sprites/text/prop/win_on.png")
                .await
                .unwrap(),
            prop_you: load_texture("assets/sprites/text/prop/you_on.png")
                .await
                .unwrap(),
            prop_stop: load_texture("assets/sprites/text/prop/stop_on.png")
                .await
                .unwrap(),
            prop_push: load_texture("assets/sprites/text/prop/push_on.png")
                .await
                .unwrap(),

            noun_ferris: load_texture("assets/sprites/text/noun/ferris_on.png")
                .await
                .unwrap(),
            noun_wall: load_texture("assets/sprites/text/noun/wall_on.png")
                .await
                .unwrap(),
            noun_flag: load_texture("assets/sprites/text/noun/flag_on.png")
                .await
                .unwrap(),
            noun_tile: load_texture("assets/sprites/text/noun/tile_on.png")
                .await
                .unwrap(),
        }
    }

    fn get_sprite(&self, object: &Object) -> Option<&Texture2D> {
        match object.class {
            ObjectClass::Generic(id) => match id {
                Object::FERRIS => Some(&self.obj_ferris),
                Object::WALL => Some(&self.obj_wall),
                Object::FLAG => Some(&self.obj_flag),
                Object::TILE => Some(&self.obj_tile),

                _ => None,
            },
            ObjectClass::TextIs => Some(&self.verb_is),
            ObjectClass::TextNoun(id) => match id {
                Object::FERRIS => Some(&self.noun_ferris),
                Object::WALL => Some(&self.noun_wall),
                Object::FLAG => Some(&self.noun_flag),
                Object::TILE => Some(&self.noun_tile),

                _ => None,
            },
            ObjectClass::TextProperty(id) => match id {
                Property::YOU => Some(&self.prop_you),
                Property::WIN => Some(&self.prop_win),
                Property::STOP => Some(&self.prop_stop),
                Property::PUSH => Some(&self.prop_push),

                _ => None,
            },
        }
    }
}

impl Frame {
    pub fn draw(&self, assets: &Assets) {
        for tile in &self.grid {
            if tile.is_empty() {
                continue;
            }

            let pos = tile[0].pos;

            for object in tile {
                draw_texture_ex(
                    assets.get_sprite(&object).unwrap(),
                    (pos.x * 24) as f32,
                    (pos.y * 24) as f32,
                    Color::from_hex(0xffffff),
                    DrawTextureParams {
                        dest_size: Some(vec2(24.0, 24.0)),
                        ..Default::default()
                    },
                )
            }
        }
    }
}