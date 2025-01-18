// Control flow

fn main() {
    let num: i32 = 5;
    if num <= 5 {
        println!("true");
    }

    //illegal
    //num must be a bool type
    //if num {
    //    println!("num is non zero");
    //}
    
    //since if is an expression we can evaluate it and 
    //assign it to a variable
    let condition: bool = false;
    let var = if condition { 5 } else { 8 };
    println!("var is {}", var);

    //in an if expression you cannot have mismatched types
    //the code below will fail to compile
    //let fea = if condition { 9 } else { "six" };
    
    //looping
    //you can exit with ctrl+c
    // loop {
    //     println!{"again"};
    // }

    //loops are expressions too and thus can be evaluated 
    //and return a value
    //break <value to return here>
    let result = {
        let mut counter = 0;
        loop {
            counter += 1;
            if counter == 10 {
                break counter * 2; 
            }
        }
    };

    println!("counter result is {}", result);

    
    //loops can have labels !!
    //useful if you need to stop an outer loop from an inner loop
    //loop labels must start with a single quote
    let mut oindex = 0;
    let mut iindex = 0;
    'outer_loop: loop {
        oindex += 1;
        println!("outer loop index {}", oindex); 
        loop {
            iindex += 1;
            if iindex == 4 {
                println!("inner index reached four so stop outer loop");
                break 'outer_loop
            }
        }
    }
    println!("oindex = {}  index = {}", oindex, iindex);

    //while loop
    //example iterating through an array
    //error prone and slow
    let myarray: [i32; 5] = [ 10, 20, 30, 40, 50 ]; 
    let mut index = 0;
    while index < 5 {
        println!("myarray[{}] = {}", index, myarray[index]);    
        index = index + 1;
    }

    //for loop
    //easier way to iterate though an array
    for element in myarray {
        println!("element is {}", element);
    }

    //for loop with Rev
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("lift off");
}
