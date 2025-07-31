fn main() {
    println!("{}", is_even(20));
    //"{}" means dynamic variable
    println!("{}", fib(5));
}

fn is_even(num: i32) -> bool {
    //i32 is signed interger with 32 bits 
    if num % 2 == 0 {
        return true;
    }
    return false;
}

fn fib(num: i32) -> i32 {
    let mut x = 0;
    let mut y = 1;

    if num == 0 || num == 1 {
        return num;
    }

    for _ in 0..(num - 1) {
        let temp = y;
        y += x;
        x = temp;
    }

    return y;
}