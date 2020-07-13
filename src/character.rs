#[derive(Debug)]
pub struct Character <'a>{
    name: &'a str,
    role: &'a str,
    strenght: i32,
    pub int: i32,
    sta: i32,
    agi: i32,
    pub mana: i32,
    haste_rating: i32,
    mastery_rating: i32,
    crit_rating: i32,
    leech_rating: i32,
    speed_rating: i32,
    dodge_percent: f32,
    parry_percent: f32,
    spell_power: i32,
    attack_power: i32,
    pub max_mana: i32,
    total_hp: i32,
}

pub fn initialize_character<'a>(name:&'a str, role:&'a str, mana:&mut i32, haste:&mut i32, mastery:&mut i32, crit:&mut i32, int:&mut i32) -> Character<'a>{
    let test_character: Character = Character {name: name, role: role, mana: *mana, haste_rating: *haste, 
                                                                    mastery_rating: *mastery, crit_rating: *crit, dodge_percent: 0.0, 
                                                                    agi: 0, int: *int, attack_power:0, leech_rating: 0, parry_percent: 0.0, 
                                                                    speed_rating: 0, spell_power: 0, sta: 0, strenght: 0, total_hp: 0, max_mana: *mana};
    return test_character
}