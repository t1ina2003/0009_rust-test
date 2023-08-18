fn main() {
    println!("Hello, world!");
    // 3-1 variable
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    //constant 
    const MAX_TEMP: u32 = 1000_000;
    //Shadowing
    let y = 5;
    let y = y + 1;
    println!("The value of y is {}", y);

}
