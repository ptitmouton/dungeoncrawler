pub use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Health {
            current: 10,
            max: 10,
        },
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };
    ecs.push((
        Enemy,
        pos,
        MovingRandomly,
        Name(name),
        Health {
            max: hp,
            current: hp,
        },
        Render {
            color: ColorPair::new(GREEN, RED),
            glyph,
        },
    ));
}

pub fn goblin() -> (i32, String, FontCharType) {
    (1, String::from("Goblin"), to_cp437('g'))
}

pub fn orc() -> (i32, String, FontCharType) {
    (2, String::from("Orc"), to_cp437('o'))
}
