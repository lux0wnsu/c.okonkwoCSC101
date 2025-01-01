use std::collections::HashMap;

fn main() {
    let mut position_map: HashMap<&str, &str> = HashMap::new();
    position_map.insert("APS 1-2", "Public Servant");
    position_map.insert("APS 1-5", "Administrator");
    position_map.insert("APS 5-8", "Senior Administrator");
    position_map.insert("E 11-8-10", "Office Manager");
    position_map.insert("E 12-10-13", "Director");
    position_map.insert("SES", "CEO");

    let staff_level = "APS 5-8";
    let years_of_experience = 6;

    let position = match staff_level {
        "APS 1-2" => "Intern",
        "APS 1-5" => "Research Assistant",
        "APS 5-8" if years_of_experience >= 5 && years_of_experience <= 8 => "Senior Administrator",
        "E 11-8-10" => "Office Manager",
        "E 12-10-13" => "Director",
        "SES" => "CEO",
        _ => "Unknown",
    };

    println!("The staff with {} years of experience and staff level {} holds the position of {}.", years_of_experience, staff_level, position);
}