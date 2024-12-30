use std::io;

fn main() {
    loop {
        println!("Select a calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Surface Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please enter a number between 1 and 6.");
                continue;
            }
        };

        match choice {
            1 => calculate_trapezium(),
            2 => calculate_rhombus(),
            3 => calculate_parallelogram(),
            4 => calculate_cube(),
            5 => calculate_cylinder(),
            6 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please select again."),
        }
    }
}

fn calculate_trapezium() {
    let height = get_input("Enter the height of the trapezium:");
    let base1 = get_input("Enter the first base of the trapezium:");
    let base2 = get_input("Enter the second base of the trapezium:");
    let area = (height * (base1 + base2)) / 2.0;
    println!("Area of Trapezium: {:.2}", area);
}

fn calculate_rhombus() {
    let diagonal1 = get_input("Enter the first diagonal of the rhombus:");
    let diagonal2 = get_input("Enter the second diagonal of the rhombus:");
    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus: {:.2}", area);
}

fn calculate_parallelogram() {
    let base = get_input("Enter the base of the parallelogram:");
    let altitude = get_input("Enter the altitude of the parallelogram:");
    let area = base * altitude;
    println!("Area of Parallelogram: {:.2}", area);
}

fn calculate_cube() {
    let side = get_input("Enter the length of a side of the cube:");
    let area = 6.0 * side * side;
    println!("Surface Area of Cube: {:.2}", area);
}

fn calculate_cylinder() {
    let radius = get_input("Enter the radius of the cylinder:");
    let height = get_input("Enter the height of the cylinder:");
    let volume = std::f64::consts::PI * radius * radius * height;
    println!("Volume of Cylinder: {:.2}", volume);
}

fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}
