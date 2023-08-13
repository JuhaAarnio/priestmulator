#[derive(Debug)]
pub struct Action <'a>{
    pub cast_time: u32,
    pub mana_cost: f32,
    pub healing_coeff: f32,
    pub name: &'a str,
    pub cooldown: f32,
    pub is_on_cooldown: bool
}

impl Action<'_> {
    pub fn toggle_cooldown_status(&mut self, status: bool) {
        self.is_on_cooldown = status
    }
}

pub fn generate_actions<'a>() -> Vec<Action<'a>> {
    let mut actions = Vec::new(); 
    let prayer_of_healing: Action = Action {cast_time: 2500, mana_cost: 0.045, healing_coeff: 0.83125, 
        name: "Prayer of Healing", cooldown: 0.0, is_on_cooldown: false};
    let holy_word_serenity: Action = Action {cast_time: 0, mana_cost: 0.025, healing_coeff: 6.65,
        name: "Holy word: Serenity", cooldown: 60.0, is_on_cooldown: false };
    let flash_heal: Action = Action {cast_time: 1500, mana_cost: 0.03, healing_coeff: 2.03, 
        name: "Flash Heal", cooldown: 0.0, is_on_cooldown: false};
    actions.push(holy_word_serenity);
    actions.push(flash_heal);
    actions.push(prayer_of_healing);
    
    return actions;
}
