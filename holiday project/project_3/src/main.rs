use std::fs::File;
use std::io::Write;

// Define a struct to hold the combined data
struct CommissionerRecord {
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

fn main() {
    // Dataset 1: Commissioners
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];
    
    // Dataset 2: Ministries
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];
    
    // Dataset 3: Geopolitical Zones
    let geopolitical_zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];
    
    // Merge the datasets
    let mut records: Vec<CommissionerRecord> = Vec::new();
    for i in 0..commissioners.len() {
        records.push(CommissionerRecord {
            name: commissioners[i].to_string(),
            ministry: ministries[i].to_string(),
            geopolitical_zone: geopolitical_zones[i].to_string(),
        });
    }
    
    // Display the merged dataset
    println!("{:<5} {:<25} {:<20} {:<15}", "S/N", "Name", "Ministry", "Geopolitical Zone");
    for (i, record) in records.iter().enumerate() {
        println!(
            "{:<5} {:<25} {:<20} {:<15}",
            i + 1,
            record.name,
            record.ministry,
            record.geopolitical_zone
        );
    }
    
    // Save the merged dataset into a file
    let mut file = File::create("efcc_records.txt").expect("Unable to create file");
    writeln!(
        file,
        "{:<5} {:<25} {:<20} {:<15}",
        "S/N", "Name", "Ministry", "Geopolitical Zone"
    )
    .expect("Unable to write to file");
    
    for (i, record) in records.iter().enumerate() {
        writeln!(
            file,
            "{:<5} {:<25} {:<20} {:<15}",
            i + 1,
            record.name,
            record.ministry,
            record.geopolitical_zone
        )
        .expect("Unable to write to file");
    }
    
    println!("\nRecords successfully saved to 'efcc_records.txt'");
}
