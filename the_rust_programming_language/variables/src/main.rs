fn main() {
    // variables mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // variables using same name
    let spaces = " ";
    let spaces = spaces.len();
    println!("The length of the space variable is: {}",spaces);

    // integers
    let guess: u32 = "42".parse().expect("Not a string");
    println!("The age of the oldest person I know is: {}", guess);

    // floats
    let a = 2.0;
    let b: f32 = 2.5;
    println!("a equals {} and b equal {}",a,b);

    // tuples
    let c: (i32, f32, u8) = (500, 2.5, 1);
    let five_hundred = c.0;
    let two_point_five = c.1;
    let one = c.2;

    // println!("My tuples values is: {}", c); doesn't work
    println!("My tuples first value is {}, the second is {} and the third is {}", five_hundred, two_point_five, one);

    // arrays
    let arr = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    for i in arr.iter() {
        println!("The current month is: {}", i);
    }

    another_function(String::from("Hello"));

    let number = return_a_number();
    if number > 5 {
        println!("This number is greater than 5");
    } else {
        println!("This number is smaller than 5");
    }

    let mut z = 1;
    z = z + 1;
    println!("Z is equal to: {}", z);
}

fn another_function(s: String) {
    println!("Just another function called with parameter: {}", s);

    let x = 5;
    let y = {
        x + 1
    };

    println!("The variables y is: {}", y);
}

fn return_a_number() -> u32 {
    45
}
