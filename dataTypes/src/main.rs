fn main() {
    let guess: i32 = "42".parse().expect("Not a number!");
    let mut tup: (i32, f64, u8) = (500, 6.4, 1);
        
    //modify the tup
    tup.0 = 0;
    let five_hundred = tup.0;

    //the type of the array and array size
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //initializing an array to the same number
    let a = [3; 5];

    use std::io;

     let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
 
    println!("Hello, world! {guess} {five_hundred}");
}
