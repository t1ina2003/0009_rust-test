use std::io; // prelude

fn main() {
    println!("猜數字");

    println!("猜一個數字");
    
    let mut guess = String::new(); // 預設是immutable, String 預設 UTF8
    
    io::stdin().read_line(&mut guess) // &取地址, read_line 返回Result, OK or Err
    .expect("無法讀取行"); // OK expect取值顯示, Err取""顯示
    
    println!("你猜測的數字是: {}", guess);
}
