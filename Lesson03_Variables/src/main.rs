use std::{io, usize};

/* ======== Integer Types ==========
*   Length      Signed      Unsigned
*   8-bit       i8          u8
*   16-bit      i16         u16
*   32-bit      i32         u32
*   64-bit      i64         u64
*   128-bit     i128        u128
*   arch        isize       usize
*
*   A note on overflow:
*       - When compiling rust in debug mode, the
*         program will panic at runtime on an
*         integer overflow.
*       - When compiling rust in release mode, the
*         program will not panic at runtime and
*         will instead overflow using two's
*         complement wrapping. For exampl, a u8
*         variable with a value of 255 will over flow
*         to 0 and 1 if we add +1 and +2 respectively
*         to the variable.
*       - You can explicitly handle wrapping with
*         the standard library methods for primitive
*         numeric types
*/
fn main() {
    //integers
    let a: i8 = -45;
    let b: isize = 123;

    //floats
    let x = 2.0; //f64
    let y: f32 = 3.0; //f32

    //numeric operations
    let sum = 5 + 10;
    let diff = 67.9 - 12.56;
    let quotient = 67.9 / 12.56;
    let truncated = -5 / 3; //results in -1
    let remainder = 43 % 3;

    //bools!
    let cond: bool = true;
    let f: bool = false;

    //char
    //chars in rust are 4bytes!
    //More on storing UTF-8 encoded text with strings
    let c = 'd';
    let mychar: char = 'Z';
    let heart_eyed_cat = '😻';

    //tuple - general way of grouping multiple values into one type
    //tuples have fixed length. They are single compound elements
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //you can use pattern matching to destructure a tuple
    let (v1, v2, v3) = tup;
    println!("{} {} {}", v1, v2, v3); // 500 6.4 1
                                      //you access a tuple element individually too.
                                      //Rust uses 0 indexing.
    println!("{} {} {}", tup.0, tup.1, tup.2); //500 6.4 1
                                               //You can create empty tuples called, units
                                               //expressions return unit if they don't return any other value
    let empty_tup = ();

    //arrays - useful if you want data on the stack
    let arr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    // you can initialize an array of 10 elements like this
    // where each element to 0, you can use different values
    // too other than 0
    let arr3 = [0; 10];

    // invalid array element access example
    let mut index: String = String::new();
    let arr4: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr4[index];
    println!("element at index {} is {}", index, element);
}
