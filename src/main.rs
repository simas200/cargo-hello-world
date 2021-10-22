fn max(x: i32, y: i32) -> i32{
    if x > y { 
        return x
    } else {
        return y
    }
}
fn main() {
    let x = 1;
    let y = 5;
    println!("Value of x = {}", x);
    println!("Max value = {}", max(x, y));
    println!("Hello, world!");
}