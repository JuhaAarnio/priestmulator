use indicatif::ProgressBar;
use rand::Rng;
use std::time::Instant;
use log::{debug, info, warn};

use crate::priority_list::set_cooldown_status;

mod priority_list;
mod character;
pub mod effects;

fn main() {
      
    let runtime_input = 240;
    let iterations_input = 25000;
    let runtime = runtime_input * 1000;
    let avg_num_of_targets = 1;
    let now = Instant::now();

    let mut priority_list = priority_list::create_action_list();  
    let mut cycles = 0;
    let mut iterations = 0;
    let pb = ProgressBar::new(iterations_input * 5);
    let mut test_character = character::initialize_character(&mut 250000, &mut 3200, &mut 2870, &mut 3732, &mut 2032, 
        &mut 10773);
    let mut stat_percentages = stat_conversion(test_character.crit_rating, 
        test_character.mastery_rating, test_character.haste_rating, test_character.leech_rating, 
        test_character.speed_rating, test_character.versatility_rating);
    
    let max_mana = set_maximum_mana(test_character.mana as f32, true, true);
    let mut iteration_hps: Vec<f32> = Vec::new();
    env_logger::init();

    info!("mastery: {}", stat_percentages[1]);
    info!("crit: {}", stat_percentages[0]);
    info!("haste: {}", stat_percentages[2]);
    info!("versatility: {}", stat_percentages[3]);

    while iterations_input > iterations {

        test_character.mana = max_mana;
        let mut active_spell = &mut priority_list[0];
        let mut cast_time = active_spell.cast_time;
        let mut time = 0; 
        let mut total_healing: f32 = 0.0;
        let mut mana_regen_interval: i16 = 5000;
        let mut mastery_tick_interval: i16 = 3000;
        let mut mastery_healing: f32 = 0.0;
        let mut mastery_ticks: i16 = 0;
        
        let crit_multiplier: f32 = 2.0;
        let mana_cost = active_spell.mana_cost * test_character.max_mana as f32;  
        let mana_regen_interval_starting_value = 5000;
        
        while time < runtime {
            time += 1; 
            mana_regen_interval -= 1;
            mastery_tick_interval -= 1;

            if active_spell.is_on_cooldown == true {
                active_spell.current_cooldown += 1.0;
                if active_spell.current_cooldown >= active_spell.cooldown {
                    set_cooldown_status(&mut active_spell, false)
                } 
            }
            
            if test_character.mana >= mana_cost as i32 { 
                if cast_time > 0 {
                    cast_time -= 1;
                }
                if cast_time == 0 {
                    cast_time = active_spell.cast_time;
                    test_character.mana -= mana_cost as i32; 
                    let mut last_healing = active_spell.healing_coeff * test_character.int as f32 * (stat_percentages[3] / 100.0 + 1.0);
                    let mut rng = rand::thread_rng();
                    let crit_comparison_value: f32 = rng.gen();
                    if stat_percentages[0] > crit_comparison_value * 100.0 {
                        last_healing = last_healing * crit_multiplier;
                    }
                    mastery_healing += last_healing;
                    mastery_ticks = 3;
                    total_healing += (active_spell.healing_coeff * test_character.int as f32) * avg_num_of_targets as f32;
                    if active_spell.cooldown > 0.0 {
                        set_cooldown_status( &mut active_spell, true);                    
                    }
                    debug!("Casted {} healing for {}", active_spell.name, last_healing);
                }
            if test_character.mana < mana_cost as i32 {
                cast_time = active_spell.cast_time;
            }
            }

            if mana_regen_interval == 0 {
                test_character.mana += effects::mana_regen(1200);
                mana_regen_interval = mana_regen_interval_starting_value;
            }
            if mastery_tick_interval == 0 {
                if mastery_ticks > 0 {
                    total_healing += effects::mastery_tick(mastery_healing, stat_percentages[4] as i32, mastery_ticks, 3) as f32;
                    mastery_ticks -= 1;
                    mastery_healing -= effects::mastery_tick(mastery_healing, stat_percentages[4] as i32, mastery_ticks, 3) as f32;
                    debug!("Mastery tick: {}", mastery_healing);
                }
                
                if mastery_ticks <= 0  {
                    mastery_ticks = 0;
                }
                mastery_tick_interval = 3000;
            }
        }


        let healing_per_second = total_healing / (time / 1000) as f32;   
        iteration_hps.push(healing_per_second);
        
        iterations += 1;
        if iterations >= iterations_input && cycles < 4 as u64 {
            match cycles{
                0=> {println!("Simulating baseline")},
                1=> {test_character.crit_rating += 750;
                    stat_percentages = update_cycle_stat_changes(&test_character)},
                2=> {test_character.crit_rating -= 750;
                    test_character.haste_rating += 750;
                    stat_percentages = update_cycle_stat_changes(&test_character)},
                3=> {test_character.haste_rating -= 750;
                    test_character.mastery_rating += 750;
                    stat_percentages = update_cycle_stat_changes(&test_character)},
                _ => warn!("Non-implemented simulation cycle")
                

            }
            iterations = 0;
            cycles += 1;
            let elapsed = now.elapsed();
            let average_hps = average(&iteration_hps);
            println!("{}", iteration_hps.len());
            iteration_hps = Vec::new();
            println!("{}", average_hps);
            println!("{:.2?}", elapsed);
            println!("{}", cycles);
        }
        pb.inc(1);
    }

    fn stat_conversion(crit: i32, mastery: i32, haste: i32, _leech: i32, _speed: i32, versatility: i32) -> Vec<f32> {
        let mastery_conversion_rate: f32 = 160.0;
        let base_mastery = 10;
        let crit_conversion_rate = 180.0;
        let haste_conversion_rate = 170.0;
        let versatility_conversion_rate = 205.0;

        let mastery_percentage = mastery as f32 / mastery_conversion_rate;
        let crit_percentage = crit as f32 / crit_conversion_rate;
        let haste_percentage = haste as f32 / haste_conversion_rate;
        let versatility_percentage = versatility as f32 / versatility_conversion_rate;

        let mut conversion_rates: Vec<f32> = Vec::new();
        conversion_rates.push(crit_percentage);
        conversion_rates.push(mastery_percentage + base_mastery as f32);
        conversion_rates.push(haste_percentage);
        conversion_rates.push(versatility_percentage);
        conversion_rates.push(mastery_percentage);
        return conversion_rates;
    }

    fn average(values: &Vec<f32>) -> f32 {
        values.iter().sum::<f32>() as f32 / values.len() as f32
    }

    fn update_cycle_stat_changes(test_character: &character::Character) -> Vec<f32> {
        let stat_percentages = stat_conversion(test_character.crit_rating, 
            test_character.mastery_rating, test_character.haste_rating, test_character.leech_rating, 
            test_character.speed_rating, test_character.versatility_rating);
        info!("mastery: {}", stat_percentages[1]);
        info!("crit: {}", stat_percentages[0]);
        info!("haste: {}", stat_percentages[2]);
        info!("versatility: {}", stat_percentages[3]);    

        return stat_percentages;
    }

    fn set_maximum_mana(max_mana: f32, has_leg_enchant: bool, has_chest_enchant: bool) -> i32 {
        let mut base_mana = max_mana;
        if has_leg_enchant {
            base_mana += max_mana * 0.03
        }
        if has_chest_enchant {
           base_mana += max_mana * 0.05
        } 
        return base_mana as i32;
    }
}


