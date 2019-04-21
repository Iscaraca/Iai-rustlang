pub fn run() {
    //For loops
    for x in 0..10 {
        println!("{}", x);
    }

    println!("End of for loop");

    //While loops (not so good way)
    let mut x = 5;
    let mut done = false;

    while !done {
        x += 3;

        println!("{}", x);

        if x == 20 {
            done = true;
        }
    }

    println!("End of 1st while loop");

    //While loops (using keywords break and continue)
    let mut y = 5;

    loop {
        y += 3;

        println!("{}", y);

        if y == 20 {break;}
    }
    println!("End of 2nd while loop");
}