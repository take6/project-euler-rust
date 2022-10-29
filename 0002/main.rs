use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let user_limit: Option<&String> = args.get(1);
    let limit: i64 = match user_limit {
        Some(x) => x.as_str().parse().unwrap(),
        // Default is 1000000
        None => 1000000,
    };

    println!("limit is {}", limit);

    let mut first: i64 = 1;
    let mut second: i64 = 1;
    let mut sum: i64 = 0;
    while second < limit {
        let tmp = second;
        second += first;
        first = tmp;
        if second % 2 == 0 {
            sum += second;
        }
        println!("second {}, sum {}", second, sum);
    }

    println!("first is {}, second is {}, sum is {}", first, second, sum);
}