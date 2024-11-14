// Functions
fn main() {
    println!("Hello, functions!");

    another_function(5);

    labeled_result(5, 'm');

    let nums: [u32; 5] = [1, 3, 5, 11, 5];

    let first_even = nums.iter().find(|&n| n % 2 == 0);
    match first_even {
        Some(num) => println!("Found even number {num}"),
        None => println!("No even number found."),
    }
}

fn another_function(x: i32) {
    println!("The value of my parameters is {x}");
}

fn labeled_result(value: i32, unit: char) {
    println!("The measurement is: {value}{unit}")
}
