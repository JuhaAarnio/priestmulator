mod priority_list;

fn main() {
    let runtime_input = 600;
    let runtime = runtime_input * 1000;
    let mut time = 0; 
    while time < runtime{
        time += 1; 
        println!("{}",time);
    }
    priority_list::execute_action();
}
