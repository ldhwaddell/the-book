fn main() {
    return_loop();
    loop_labels();
    while_loop();
    loop_collection();
    for_loop();
    reverse_for_loop();

    let n = 45;
    let number = nth_fib(n);
    println!("{n}th fib is {number}");

    let c = 25.0;
    let f = c_to_f(c);

    println!("{c} celcius is {f} farenheit");
}

fn return_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!")
}

fn loop_collection() {
    // Slow, compiler adds runtime code to perform conditional check on if index in bounds on every loop.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);

        index += 1
    }
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is {element}")
    }
}

fn reverse_for_loop() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!");
}

fn nth_fib(n: u32) -> u32 {
    return n - 1 + n - 2;
}

fn c_to_f(c: f32) -> f32 {
    return (9.0 / 5.0) * c + 32.0;
}
