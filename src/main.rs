mod priority_list;
mod character;

fn main() {
    priority_list::execute_action();
    character::initialize_character("test", &mut "healer", &mut 100000, &mut 2500, &mut 1500, &mut 1500, &mut 6700);
    let runtime_input = 6;
    let runtime = runtime_input * 1000;
    let mut time = 0; 
    while time < runtime{
        time += 1; 
        println!("{}",time);
    }
    
}


