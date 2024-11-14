fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 3;
        println!("The inner value of x is {x}");
    }
    println!("The outer value of x is {x}");
}
