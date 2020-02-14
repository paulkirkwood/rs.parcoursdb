pub fn stage_id(stage_number: i32, suffix: char, split_stages: bool) -> String {
    if split_stages == true {
        format!("{}{}", stage_number, suffix)
    }
    else {
        format!("{}", stage_number)
    }
}

pub fn increment_suffix(c: char) -> char {
    std::char::from_u32(c as u32 + 1).unwrap_or(c)
}
