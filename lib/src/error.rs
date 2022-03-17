pub fn exit<'a>(message: &'a str) {
    println!("Error: {message}");
    std::process::exit(1)
}