#[derive(Debug)]
struct Action <'a>{
    cast_time: u32,
    mana_cost: f32,
    healing_coeff: f32,
    name: &'a str,
}

pub fn generate_actions() {
    let prayer_of_healing: Action = Action {cast_time: 2500, mana_cost: 0.045, healing_coeff: 0.625, name: "Prayer of Healing"};
    let flash_heal: Action = Action {cast_time: 1500, mana_cost: 0.03, healing_coeff: 1.35, name: "Flash Heal"};
    println!("{:?}", flash_heal);
    println!("{:?}", prayer_of_healing);
}
