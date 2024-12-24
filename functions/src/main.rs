fn main() {
    println!("Hello, world!");

    //expression, expressions don't end with a semi colon
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let five_result = five();
    let plus_one_result = plus_one(2);

    println!("value of plus one result result {}", plus_one_result);

    println!("value of five result is: {}", five_result); 

    another_function(2);
    print_labeled_measurement(5, 'h');
}
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//functions that return
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
     x + 1
}