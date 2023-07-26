fn main() {
    let x: i32 = 53;
    if x < 50 {
        println!("less than fifty");
    } else {
        println!("greater than fifty");
    }

    //we can also store condition in let but in this case whatever we are string in the let will
    //have to be of same type
    let condition: &str = if x < 50 {
        "Less than fifty"
    } else {
        "Greater than fifty"
    };
    println!("The real truth is, he is {}", condition);
}
