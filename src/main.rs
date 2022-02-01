mod priority_list;
mod character;
pub mod effects;

fn main() {
    let priority_list = priority_list::execute_action();
    let mut test_character = character::initialize_character("test", &mut "healer", &mut 100000, &mut 2500, &mut 1500, &mut 1500, &mut 1500, &mut 6700);
    let runtime_input = 600;
    let runtime = runtime_input * 1000;
    let mana_cost = priority_list[0].mana_cost * test_character.max_mana as f32;
    let avg_num_of_targets = 4;
    let mana_regen_interval_starting_value = 5000;
    let mut cast_time = priority_list[0].cast_time;
    let mut time = 0; 
    let mut total_healing: f32 = 0.0;
    let mut mana_regen_interval: i16 = 5000;
    
    while time < runtime {
        time += 1; 
        mana_regen_interval -= 1;
        stat_conversion(test_character.crit_rating, test_character.mastery_rating, test_character.haste_rating, test_character.leech_rating, 
                        test_character.speed_rating, test_character.versatility_rating);
        if test_character.mana as f32 > mana_cost {
            cast_time -= 1;
            if cast_time == 0 {
                println!("{:?}", priority_list[0]);
                println!("{:?}", test_character.mana);
                cast_time = priority_list[0].cast_time;
                test_character.mana -= mana_cost as i32; 
                total_healing += (priority_list[0].healing_coeff * test_character.int as f32) * avg_num_of_targets as f32;
            if test_character.mana < mana_cost as i32 {
                println!("out of mana");
                cast_time = priority_list[0].cast_time;
            }
        } 
        }
        if mana_regen_interval == 0 {
            test_character.mana += effects::mana_regen(1200);
            mana_regen_interval = mana_regen_interval_starting_value;
        }
    }

    let healing_per_second = total_healing / (time / 1000) as f32;   
    println!("{:?}", test_character);
    println!("{}", total_healing.to_string());
    println!("{}", healing_per_second);
}

fn stat_conversion(crit: i32, mastery: i32, haste: i32, leech: i32, speed: i32, versatility: i32) -> Vec<f32> {
    let mastery_conversion_rate = 0.0;
    let crit_conversion_rate = 72.0;
    let haste_conversion_rate = 68.0;
    let versatility_conversion_rate = 85.0;

    let mastery_percentage = mastery_conversion_rate;
    let crit_percentage = crit as f32 / crit_conversion_rate;
    let haste_percentage = haste as f32 / haste_conversion_rate;
    let versatility_percentage = versatility as f32 / versatility_conversion_rate;
    
    let mut conversion_rates: Vec<f32> = Vec::new();
    conversion_rates.push(crit_percentage);
    conversion_rates.push(mastery_percentage);
    conversion_rates.push(haste_percentage);
    conversion_rates.push(versatility_percentage);
    conversion_rates.push(mastery_percentage);
    return conversion_rates;
}


