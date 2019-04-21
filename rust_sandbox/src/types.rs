pub fn run() {
    /*
    bool
    char
    i8, i16, i32, i64, isize, u8, u16, u32, u64, usize
    f32, f64
    array
    slice
    str
    tuple
    */

    //bool
    let t = true;
    let f = false;

    //char
    let a = 'a';
    let b = 'b';
    let keyboard = '⌨';

    //int
    let x = 5;
    let life = 42;
    let jenny = 8675309;

    //float
    let pi = 3.14;
    let e = 2.718;

    println!("{:?}", (t, f, a, b, keyboard, x, life, jenny, pi, e));

    //array
    //let name: [type; size] = [elem1, elem2, elem3, elem4];
    let mut array: [i32; 5] = [0, 1, 2, 3, 4];

    //Reassign value
    array[0] = 20;

    println!("The first element of the array is {}", array[0]);

    let mut counter = 0;
    for x in array.iter(){
        println!("The element at index {} is {}", counter, x);
        counter += 1;
    }

    //Get array length
    println!("Array length: {}", array.len());
}