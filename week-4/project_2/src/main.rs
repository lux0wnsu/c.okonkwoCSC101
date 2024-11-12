use std::io; 

 

fn main() { 

    // Input experience status from the user 

    let mut input = String::new(); 

 

    println!("Is the employee experienced? (yes/no):"); 

    io::stdin().read_line(&mut input).expect("Failed to read line"); 

    let is_experienced = input.trim().eq_ignore_ascii_case("yes"); 

    input.clear(); 

 

    // Input age from the user 

    println!("Enter the age of the employee:"); 

    io::stdin().read_line(&mut input).expect("Failed to read line"); 

    let age: u32 = input.trim().parse().expect("Please enter a valid age"); 

 

    // Determine the incentive based on the conditions 

    let incentive = if is_experienced { 

        if age >= 40 { 

            1_560_000 // Incentive for experienced employee aged 40 or more 

        } else if age >= 30 { 

            1_480_000 // Incentive for experienced employee aged 30 to less than 40 

        } else if age < 28 { 

            1_300_000 // Incentive for experienced employee below 28 

        } else { 

            0 // Default case (should not happen as all age ranges are covered) 

        } 

    } else { 

        100_000 // Incentive for inexperienced employee 

    }; 

 

    println!("The annual incentive for the employee is: N{:.2}", incentive); 

} 