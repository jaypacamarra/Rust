// Rust functions

fn foo() {
    println!("Attention. This is a new message.");
}

fn bar(arg: i32) {
    println!("you passed in the number {}", arg);
}

fn five() -> i32 {
    5  //<-- do not put a semicolon here or else it will become a statement
       //    doing so will make the function return unit or ()
       //    thus contradicting the i32 return type and throwing a 
       //    compile time error
}

fn main() {
    foo();
    bar(4);
    print_labeled_measurement(25, 'C');
    println!("value of five() is {}",five());

    //statements vs expressions
    //statements perform some action without returning a value
    //expression evaluate to a resultant value
    //examples of statements
    // - variable declarations
    // - function declarations
    //Illegal statements
    // - let x = (let y = 5);
    // The above is illegal because statements don't return a value
    // Languages like c allow this, int x = y = 6;
    // Rust does not allow this!!

    //expressions evaluate to a value
    //examples of expressions
    // - the value `6` in let y = 6 is an expression becaues it evaluates to 6
    // - calling a function is an expression
    // - calling a macro is an expression
    // - a new scope block created with curly brackes is an expression
    let y = {
        let x = 3;
        x + 1     //<---Note this DOES NOT have a semicolon.
    };

    println!("The value of y is: {}", y);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement: {value}{unit_label}");
}
