#[derive(Debug)]
pub struct Character <>{
    pub int: i32,
    pub mana: i32,
    pub haste_rating: i32,
    pub mastery_rating: i32,
    pub crit_rating: i32,
    pub versatility_rating: i32,
    pub leech_rating: i32,
    pub speed_rating: i32,
    pub max_mana: i32,
}

pub fn initialize_character<'a>(mana:&mut i32, haste:&mut i32, mastery:&mut i32, crit:&mut i32, versatility:&mut i32 ,int:&mut i32) -> Character<>{
    let test_character: Character = Character {mana: *mana, haste_rating: *haste, mastery_rating: *mastery, crit_rating: *crit, versatility_rating: *versatility, 
                                                 int: *int, leech_rating: 0, speed_rating: 0,  max_mana: *mana};
    return test_character
}