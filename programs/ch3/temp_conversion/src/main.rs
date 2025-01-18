use std::io;


fn ask_user_for_temp() -> f32 {
    println!("enter a temperature in celsius");
    let mut celsius = String::new();
    
    io::stdin()
        .read_line(&mut celsius)
        .expect("could not read line");

    celsius.trim().parse().expect("could not read line")
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    // f = (celsius x 9/5) + 32 
    (celsius * 9.0/5.0) + 32.0
}

fn main() {
    let user_celsius: f32 = ask_user_for_temp();
    println!("{}C is equal to {}F", user_celsius, celsius_to_fahrenheit(user_celsius));
}
