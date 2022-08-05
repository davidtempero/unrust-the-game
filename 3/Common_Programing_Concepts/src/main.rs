use std::io;

fn main() {

    let mut temp = String::new();
    println!("Insert input:");
    io::stdin().read_line(&mut temp).expect("Failed to get input");

    let uint_var: u32;
    uint_var = temp.trim().parse().expect("Failed to convert to unsigned integer");

    println!("Var is {}",uint_var);

    const TRIES: u8 = 8;

    'first_for: for number in 1..TRIES {
        println!("This is my {} try",number);
    }
    /*
    Multiple
    comment
    lines
    */
}
