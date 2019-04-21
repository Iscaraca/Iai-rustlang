pub fn run() {
    let path = false;
    println!("{}", path);

    match path {
        true => println!("path is correct but in match"),
        false => println!("path sucks but in match"),
    }

    if path {
        println!("path is correct but in if/else");
    }
    else {
        println!("path sucks but in if/else");
    }

    
}