fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    const MIN_TO_SEC: i32 = 60;
    println!(
        "To convert minute to second we will have to multiply with {}",
        MIN_TO_SEC
    );
    //Variable Shadowing
    let _y = 10;
    let _y = 10 + 1;
    {
        let _y = 10 + 2;
        println!("The value of y in inner scope is {}", _y);
    }
    println!("The value of y is {}", _y);
}
