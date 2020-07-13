mod priority_list;
mod character;

fn main() {
    let priority_list = priority_list::execute_action();
    let mut test_character = character::initialize_character("test", &mut "healer", &mut 100000, &mut 2500, &mut 1500, &mut 1500, &mut 6700);
    let runtime_input = 600;
    let runtime = runtime_input * 1000;
    let mana_cost = priority_list[0].mana_cost * test_character.max_mana as f32;
    let avg_num_of_targets = 3;
    let mut cast_time = priority_list[0].cast_time;
    let mut time = 0; 
    let mut total_healing: f32 = 0.0;
    
    while time < runtime{
        time += 1; 
        cast_time -= 1;
        if cast_time == 0 && test_character.mana as f32 > mana_cost {
            println!("{:?}", priority_list[0]);
            cast_time = priority_list[0].cast_time;
            test_character.mana -= mana_cost as i32; 
            total_healing += (priority_list[0].healing_coeff * test_character.int as f32) * avg_num_of_targets as f32;
            if test_character.mana < mana_cost as i32 {
                println!("out of mana");
                break;
            }
        }
    }   
    println!("{:?}", test_character);
    println!("{}", total_healing.to_string())
}


