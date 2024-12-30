use std::fs::File;
use std::io::Write;

// Define a struct for Student
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() {
    // Create a vector of students
    let students = vec![
        Student {
            name: String::from("Oluchi Mordi"),
            matric_number: String::from("ACC10211111"),
            department: String::from("Accounting"),
            level: 300,
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO10110101"),
            department: String::from("Economics"),
            level: 200,
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_number: String::from("CSC10328882"),
            department: String::from("Computer"),
            level: 200,
        },
        Student {
            name: String::from("Adekunle Gold"),
            matric_number: String::from("EEE11020202"),
            department: String::from("Electrical"),
            level: 400,
        },
        Student {
            name: String::from("Blanca Edemoh"),
            matric_number: String::from("MEE10202001"),
            department: String::from("Mechanical"),
            level: 100,
        },
    ];

    // Display student details
    println!("PAU SMIS - Student Details\n");
    println!(
        "{:<20} {:<15} {:<15} {:<5}",
        "Student Name", "Matric Number", "Department", "Level"
    );
    for student in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        );
    }

    // Save student details to a file
    let mut file = File::create("students_data.txt").expect("Unable to create file");

    writeln!(
        file,
        "{:<20} {:<15} {:<15} {:<5}",
        "Student Name", "Matric Number", "Department", "Level"
    )
    .expect("Unable to write to file");

    for student in &students {
        writeln!(
            file,
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        )
        .expect("Unable to write to file");
    }

    println!("\nStudent details have been saved to 'students_data.txt'");
}
