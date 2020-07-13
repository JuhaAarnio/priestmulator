mod actions;

pub fn execute_action<'a>() -> Vec<actions::Action<'a>>{
    let priority_list = actions::generate_actions();
    return priority_list;
}