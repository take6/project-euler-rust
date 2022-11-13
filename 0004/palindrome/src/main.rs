fn to_right_way(n: &u64) -> u64 {
    let mut s: String = String::from("");
    for c in n.to_string().chars().rev() {
        s.push(c);
    }
    let r: u64 = s.parse().unwrap();
    r
}

fn is_palindromic(left_way: &u64, right_way: &u64) -> bool {
    left_way == right_way
}

fn main() {
    let mut max_palindromic: u64 = 0;
    for n0 in (100..999).rev() {
        for n1 in (100..999).rev() {

            let number: u64 = n0 * n1;
            let right_way = to_right_way(&number);
            let result = is_palindromic(&number, &right_way);

            if result && number > max_palindromic {
                println!("number {}, right_way {}, result {}", number, right_way, result);
                max_palindromic = number;
            }
        }
    }

    println!("max palindromic number is {}", max_palindromic);
}
