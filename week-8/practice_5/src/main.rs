fn main() {
    // Create an empty vector "City"
    let mut city : Vec<String> = Vec::new();

    // Print the City Vector
    println!("The City vector has element {}", city.len());

    // Push new elements into the vector
    let mut input1 = String::new();
    println!("How many Cities do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num: i32 = input1.trim().parse().expect("Invalid input");
    for _ in 0..city_num {
        let mut input2 = String::new();
        println!("Enter city {}",  city.len() + 1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_city:String = input2.trim().parse().expect("Invalid input");
        city.push(new_city);
    }

    // Print the updated City Vector
    println!("Your preferred cities are: \n");
    let mut count=1;

    //loop to iterate elements in vector 
    for i in city
    {

    // Iterate through 1 to the length of the vector
        println!("{} {}", count, i);
        count+=1
    }

}