use std::io;

fn main () {
    println!("Enter substring:");
    let mut guess = String::new();
 
    io::stdin().read_line(&mut guess).expect("failed to readline");
 
    
    let chuoi = String::from("This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.");
    println!("{}",chuoi.matches(&guess.trim()).count());

    println!("{}", guess);
}