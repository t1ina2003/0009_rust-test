use std::io; // prelude
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜數字");

    // #### Gen secret num
    let secret_number = rand::thread_rng().gen_range(1..101); // i32, u32, i64
    println!("神秘數字{}", secret_number);

    // #### Read user input
    let mut guess = String::new(); // 預設是immutable, String 預設 UTF8
    println!("猜一個數字");
    io::stdin().read_line(&mut guess) // &取地址, read_line 返回Result, OK or Err
    .expect("無法讀取行"); // OK expect取值顯示, Err取""顯示
    println!("你猜測的數字是: {}", guess);

    // #### Match input and secret
    // shadow new variable
    let guess:u32 = guess.trim().parse().expect("Please type a number");
    // match, arm
    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You Win!"), //arm
        Ordering::Greater => println!("Too Big!"), //arm
        Ordering::Less => println!("Too Small!"), //arm
    }
}
