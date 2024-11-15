// Ownership and borrowing
fn main() {
    let mut x = 42;
    {
        let y = &mut x;
        *y = *y + 1;
        println!("y={y}");
    }
    println!("x={x}");
}
