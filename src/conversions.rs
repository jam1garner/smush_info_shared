use smash::lib::lua_const::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use smush_discord_shared::{Stage, Character};

fn character_pairs() -> [(i32, Character); 90] {
    use Character::*;
    [
        (*FIGHTER_KIND_BAYONETTA, Bayonetta),	
        (*FIGHTER_KIND_BRAVE, Brave),	
        (*FIGHTER_KIND_BUDDY, Buddy),	
        (*FIGHTER_KIND_CAPTAIN, Captain),	
        (*FIGHTER_KIND_CHROM, Chrom),	
        (*FIGHTER_KIND_CLOUD, Cloud),	
        (*FIGHTER_KIND_DAISY, Daisy),	
        (*FIGHTER_KIND_DEDEDE, Dedede),	
        (*FIGHTER_KIND_DIDDY, Diddy),	
        (*FIGHTER_KIND_DOLLY, Dolly),	
        (*FIGHTER_KIND_DONKEY, Donkey),	
        (*FIGHTER_KIND_DUCKHUNT, Duckhunt),	
        (*FIGHTER_KIND_FALCO, Falco),	
        (*FIGHTER_KIND_FOX, Fox),	
        (*FIGHTER_KIND_FUSHIGISOU, Fushigisou),	
        (*FIGHTER_KIND_GAMEWATCH, Gamewatch),	
        (*FIGHTER_KIND_GANON, Ganon),	
        (*FIGHTER_KIND_GAOGAEN, Gaogaen),	
        (*FIGHTER_KIND_GEKKOUGA, Gekkouga),	
        (*FIGHTER_KIND_IKE, Ike),	
        (*FIGHTER_KIND_INKLING, Inkling),	
        (*FIGHTER_KIND_JACK, Jack),	
        (*FIGHTER_KIND_KAMUI, Kamui),	
        (*FIGHTER_KIND_KEN, Ken),	
        (*FIGHTER_KIND_KIRBY, Kirby),	
        (*FIGHTER_KIND_KOOPA, Koopa),	
        (*FIGHTER_KIND_KOOPAG, Koopag),	
        (*FIGHTER_KIND_KOOPAJR, Koopajr),	
        (*FIGHTER_KIND_KROOL, Krool),	
        (*FIGHTER_KIND_LINK, Link),	
        (*FIGHTER_KIND_LITTLEMAC, Littlemac),	
        (*FIGHTER_KIND_LIZARDON, Lizardon),	
        (*FIGHTER_KIND_LUCARIO, Lucario),	
        (*FIGHTER_KIND_LUCAS, Lucas),	
        (*FIGHTER_KIND_LUCINA, Lucina),	
        (*FIGHTER_KIND_LUIGI, Luigi),	
        (*FIGHTER_KIND_MARIO, Mario),	
        (*FIGHTER_KIND_MARIOD, Mariod),	
        (*FIGHTER_KIND_MARTH, Marth),	
        (*FIGHTER_KIND_MASTER, Master),	
        (*FIGHTER_KIND_METAKNIGHT, Metaknight),	
        (*FIGHTER_KIND_MEWTWO, Mewtwo),	
        (*FIGHTER_KIND_MIIENEMYF, Miienemyf),	
        (*FIGHTER_KIND_MIIENEMYG, Miienemyg),	
        (*FIGHTER_KIND_MIIENEMYS, Miienemys),	
        (*FIGHTER_KIND_MIIFIGHTER, Miifighter),	
        (*FIGHTER_KIND_MIIGUNNER, Miigunner),	
        (*FIGHTER_KIND_MIISWORDSMAN, Miiswordsman),	
        (*FIGHTER_KIND_MURABITO, Murabito),	
        (*FIGHTER_KIND_NANA, Nana),	
        (*FIGHTER_KIND_NESS, Ness),	
        (*FIGHTER_KIND_PACKUN, Packun),	
        (*FIGHTER_KIND_PACMAN, Pacman),	
        (*FIGHTER_KIND_PALUTENA, Palutena),	
        (*FIGHTER_KIND_PEACH, Peach),	
        (*FIGHTER_KIND_PFUSHIGISOU, Pfushigisou),	
        (*FIGHTER_KIND_PICHU, Pichu),	
        (*FIGHTER_KIND_PIKACHU, Pikachu),	
        (*FIGHTER_KIND_PIKMIN, Pikmin),	
        (*FIGHTER_KIND_PIT, Pit),	
        (*FIGHTER_KIND_PITB, Pitb),	
        (*FIGHTER_KIND_PLIZARDON, Plizardon),	
        (*FIGHTER_KIND_POPO, Popo),	
        (*FIGHTER_KIND_PURIN, Purin),	
        (*FIGHTER_KIND_PZENIGAME, Pzenigame),	
        (*FIGHTER_KIND_REFLET, Reflet),	
        (*FIGHTER_KIND_RICHTER, Richter),	
        (*FIGHTER_KIND_RIDLEY, Ridley),	
        (*FIGHTER_KIND_ROBOT, Robot),	
        (*FIGHTER_KIND_ROCKMAN, Rockman),	
        (*FIGHTER_KIND_ROSETTA, Rosetta),	
        (*FIGHTER_KIND_ROY, Roy),	
        (*FIGHTER_KIND_RYU, Ryu),	
        (*FIGHTER_KIND_SAMUS, Samus),	
        (*FIGHTER_KIND_SAMUSD, Samusd),	
        (*FIGHTER_KIND_SHEIK, Sheik),	
        (*FIGHTER_KIND_SHIZUE, Shizue),	
        (*FIGHTER_KIND_SHULK, Shulk),	
        (*FIGHTER_KIND_SIMON, Simon),	
        (*FIGHTER_KIND_SNAKE, Snake),	
        (*FIGHTER_KIND_SONIC, Sonic),	
        (*FIGHTER_KIND_SZEROSUIT, Szerosuit),	
        (*FIGHTER_KIND_TOONLINK, Toonlink),	
        (*FIGHTER_KIND_WARIO, Wario),	
        (*FIGHTER_KIND_WIIFIT, Wiifit),	
        (*FIGHTER_KIND_WOLF, Wolf),	
        (*FIGHTER_KIND_YOSHI, Yoshi),	
        (*FIGHTER_KIND_YOUNGLINK, Younglink),	
        (*FIGHTER_KIND_ZELDA, Zelda),	
        (*FIGHTER_KIND_ZENIGAME, Zenigame),
    ]
}

lazy_static!{
    static ref KIND_TO_CHAR: HashMap<i32, Character> = {
        character_pairs().iter().cloned().collect()
    };
}

pub fn kind_to_char(kind: i32) -> Character {
    *KIND_TO_CHAR.get(&kind).unwrap_or(&Character::None)
}