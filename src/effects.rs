pub fn mana_regen(mana_return: i32) -> i32{
    return mana_return;
}

pub fn mastery_tick (base_healing: f32, mastery_percentage: i32, remaining_ticks: i16, total_number_of_ticks: i32) -> f32{
    let total_mastery_healing: f32 = base_healing * mastery_percentage as f32;
    let remaining_ticks = remaining_ticks;
    if remaining_ticks > 0 {
        return total_mastery_healing / total_number_of_ticks as f32;
    }
    else {
        return 0.0;
    }
}