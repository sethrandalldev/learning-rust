use std::io;

fn main() {
    println!("Get nth fibonacci.");

    let mut input = String::new();

    loop {
        println!("Please input fibonacci value you want to compute");

        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

       let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("fib({input}) = {}", get_fib(input));
        break;
    }

    fn get_fib(n: u32) -> u32 {
        if n < 2 { return n; }
        return get_fib(n-1) + get_fib(n-2);
    }


}