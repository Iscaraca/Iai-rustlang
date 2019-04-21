pub fn run() {
    //let x = 5;
    let x = 6;
    //let x = 4;

    if x == 5 {
        println!("x is five");
    }

    else if x == 6 {
        println!("x is six");
    }

    else {
        println!("x is neither five nor six");
    }

    let y = if x == 5 { 10 } else { 15 };

    println!("{}", y)
}