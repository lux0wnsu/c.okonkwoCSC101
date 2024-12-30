use std::fs::File;
use std::io::Write;

fn main() {
    let categories = vec![
        ("Lager", vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"]),
        ("Stout", vec!["Legend", "Turbo King", "Williams"]),
        ("Non-Alcoholic", vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"]),
    ];

    let mut file = File::create("drinks_catalog.txt").expect("Unable to create file");

    for (category, drinks) in categories {
        writeln!(file, "{}:", category).expect("Unable to write to file");
        for drink in drinks {
            writeln!(file, "  - {}", drink).expect("Unable to write to file");
        }
        writeln!(file).expect("Unable to write to file");
    }

    println!("Drinks catalog saved to 'drinks_catalog.txt'");
}
