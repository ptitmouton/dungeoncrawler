pub use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
        pos,
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(8),
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
    let is_chasing_player = match rng.roll_dice(1, 10) {
        1..=8 => true,
        _ => false,
    };
    let health = Health {
        max: hp,
        current: hp,
    };
    let render = Render {
        color: ColorPair::new(GREEN, RED),
        glyph,
    };
    if is_chasing_player {
        ecs.push((
            Enemy,
            pos,
            Name(name),
            health,
            ChasingPlayer,
            FieldOfView::new(6),
            render,
        ));
    } else {
        ecs.push((
            Enemy,
            pos,
            Name(name),
            health,
            MovingRandomly,
            FieldOfView::new(6),
            render,
        ));
    }
}

pub fn goblin() -> (i32, String, FontCharType) {
    (1, String::from("Goblin"), to_cp437('g'))
}

pub fn orc() -> (i32, String, FontCharType) {
    (2, String::from("Orc"), to_cp437('o'))
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name(String::from("Amulet of Yala")),
    ));
}

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('!'),
        },
        Name(String::from("Healing Potion")),
        ProvidesHealing { amount: 6 },
    ));
}

pub fn spawn_magic_mapper(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('{'),
        },
        Name(String::from("Dungeon Map")),
        ProvidesDungeonMap,
    ));
}

pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let roll = rng.roll_dice(1, 6);
    match roll {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_magic_mapper(ecs, pos),
        _ => spawn_monster(ecs, rng, pos),
    }
}
