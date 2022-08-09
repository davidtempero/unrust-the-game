fn main() {
    let mut s = String::new();

    println!("type your string:");
    std::io::stdin()
        .read_line(&mut s)
        .expect("Failed to read string");

    string_size(&s);

}

fn string_size (name: &String) {
    println!("String has {} characters",name.len());
}

