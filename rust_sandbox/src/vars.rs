pub fn run() {
    let name = "Isaac";
    let mut age = 15;

    println!("My name is {} and I am {}.", name, age);

    age = 16;

    println!("My name is {} and I am now {}.", name, age);

    //Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign Multiple Variables
    let ( my_name, my_age ) = ( "Isaac", 16 );
    println!("My name is {} and I am now {}.", my_name, my_age);
}