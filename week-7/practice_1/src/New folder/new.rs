fn main() {
    let x  = 10;
    let y = {
        let x = 3;
        x + 2
    };
    println!("Y: {}", y);
}