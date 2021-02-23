use std::io::stdin;
use std::time::SystemTime;

fn fib(limit: u8) -> u8 {
    let defaults: [u8; 2] = [0, 1];
    if defaults.contains(&limit) {
        return limit;
    }

    fib(limit - 1) + fib(limit - 2)
}

fn main() {
    let start = SystemTime::now();
    let mut buffer = String::new();
    println!("Write Fibonacci limit where 0 <= limit <= 255:");
    stdin().read_line(&mut buffer).expect("Error reading");
    let time_for_chosen_number = SystemTime::now();
    buffer.remove(buffer.len() - 1);
    let limit = buffer.parse::<u8>().unwrap_or_else(|_| 0);
    let result = fib(limit);
    let end = SystemTime::now();
    println!(
        "Time this fib algorithm took: {}",
        end.duration_since(start).unwrap().as_nanos()
            - time_for_chosen_number
                .duration_since(start)
                .unwrap()
                .as_nanos()
    );
    println!("Fib result at index {} is {}", limit, result);
}
