fn main() {
    println!("Hello, world!");
    // 3-1 variable
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    //constant 
    const MAX_TEMP: u32 = 1000_000;
    println!("Constant MAX_TEMP is {}", MAX_TEMP);
    //Shadowing
    let y = 5;
    let y = y + 1;
    println!("The value of y is {}", y);
    
    // 3-2 type
    let _guess: u32 = "42".parse().expect("not a number");
    
    // 3-3 tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    println!("{}, {}, {}", x, y, z);
    let _a: [i32; 5] = [1,2,3,4,5]; // store in stack, Vecotr better.
    // rust examine array addr, index out of bounds
    // let months = ["a", "b", "c", "d"];
    // let index = [12, 13, 14, 15];
    // let month = months[index[0]];
    // println!("{}", month);
    another_function(5,6);

}
    //3-4 fn, parameter must have type
fn another_function(x: i32, y: i32){
    println!("the value of x is {}", x);
    println!("the value of y is {}", y);
    // special use 
    // z = {expect an expression}
    // let a = 1; semicolon ending is a "statement"
    // but a + 3 without semicolon is a "expression"
    // so it's ok for compile & run.
    let z = {
        let a = 1;
        a + 3
    };
    println!("the value of z is {}", z);

    println!("{}", five(1));
}
    // fn with return, "no name" for return.
fn five(x: i32) -> i32 {
    // return 3;// if we need return early, return 

    x + 5 // don't semicolon, it's expression not statement.
}
