struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // Define the laptops
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    // Calculate total cost for 3 of each brand
    let total_cost = hp.total_cost(3)
        + ibm.total_cost(3)
        + toshiba.total_cost(3)
        + dell.total_cost(3);

    println!("The total cost for 3 laptops from each brand is: ₦{}", total_cost);
}
