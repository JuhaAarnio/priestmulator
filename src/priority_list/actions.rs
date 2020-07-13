#[derive(Debug)]
pub struct Action <'a>{
    pub cast_time: u32,
    pub mana_cost: f32,
    pub healing_coeff: f32,
    pub name: &'a str,
}

pub fn generate_actions<'a>() -> Vec<Action<'a>> {
    let mut actions = Vec::new(); 
    let prayer_of_healing: Action = Action {cast_time: 2500, mana_cost: 0.045, healing_coeff: 0.625, name: "Prayer of Healing"};
    let flash_heal: Action = Action {cast_time: 1500, mana_cost: 0.03, healing_coeff: 1.35, name: "Flash Heal"};
    actions.push(prayer_of_healing);
    actions.push(flash_heal);
    return actions;
}
