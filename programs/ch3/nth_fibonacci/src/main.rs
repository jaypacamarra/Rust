fn get_nth_fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut first = 0;  // when n=2 then first is 0
        let mut second = 1; // when n=2 then second is 1
        let mut degree = 2; // at this point we are on the 2nd degree
        let mut fib = 0;    // holds the nth fibonacci 
        loop {
            if degree <= n {
                fib = first + second;
                degree = degree + 1;
                first = second;
                second = fib;
            }
            else {
                break fib 
            }
        } 
    }
}

fn main() {
    let degree_fib: [i32; 11] = [0,1,2,3,4,5,6,7,8,9,10];
    for n in degree_fib {
        println!("the {}th fibonacci is {}", n, get_nth_fibonacci(n));
    } 
}
