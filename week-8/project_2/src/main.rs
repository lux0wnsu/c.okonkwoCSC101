use std::collections::HashMap;

fn main() {
    let mut developer_experience: HashMap<&str, u32> = HashMap::new();
    developer_experience.insert("Developer A", 5);
    developer_experience.insert("Developer B", 8);
    developer_experience.insert("Developer C", 3);
    developer_experience.insert("Developer D", 10);
    developer_experience.insert("Developer E", 7);

    let mut max_experience = 0;
    let mut max_experience_developer = "";

    for (developer, experience) in developer_experience.iter() {
        if experience > &max_experience {
            max_experience = *experience;
            max_experience_developer = *developer;
        }
    }

    println!("The developer with the highest years of programming experience is {} with {} years.", max_experience_developer, max_experience);
}