mod actions;


pub fn create_action_list<'a>() -> Vec<actions::Action<'a>>{
    let priority_list = actions::generate_actions();
    return priority_list;
}

pub fn set_cooldown_status(mut action: actions::Action, status_to_be_set: bool) -> () {
    action.toggle_cooldown_status(status_to_be_set)
}