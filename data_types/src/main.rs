#![allow(unused)]

fn main() {
    //Scaler type represents a single value like int floating-point number, boolean, char
    //in means signed number with n bits meaning it can store -2^(n-1) to 2^(n-1)-1 numbers
    let c = 100;
    let name = "Shouvik";
    //in the same way for unassigned number the range is 0 to 2^(n)-1
    //all floating point numbers are signed
    let x: f64 = 2.5;
    println!(
        "The value of x is {} and it is 64 bit floating point number",
        x
    );
    let bool = true;
    let bool2 = false;
    let f: bool = false;
    let f: bool = true;
    let c: char = 'A';

    //Compound types means grouping multiple variables in one type.
    //Rust has two premitive compound types -> tuple and array
    //tuples are fixed sized.
    let tup: (i32, &str, char) = (500, "Shouvik", 'Z');
    //now to retrive a single value out of a tuple we can do this ->
    let (x, y, z) = tup;
    println!("The value of y is {}", y);
    // we can also use period(.) followed by index of the value we want to retrive
    let five_hundred = tup.0;
    println!("The value is {}", five_hundred);
    //tuple without any value is called unit;
    let tup2: () = ();
    //ARRAY TYPE;
    //Every element of an array should have the same type
    //Arrays also have fixed length
    let arr = [1, 2, 3, 4, 5];
    //arrays allocates data in stack not in heaps
    let arr2: [&str; 5] = ["Shouvik1", "Shouvik2", "Shouvik3", "Shouvik4", "Shouvik5"];
    let arr3 = [3; 5]; //this means the arr3 contains 5 3's
                       //accrssing array element
    let first = arr3[2];
    println!("{}", first);
}
