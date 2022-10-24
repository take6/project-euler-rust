fn main() {
    let mut sum: i64 = 0;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            println!("{} is multiple of 3 or 5!", n);
            sum += n;
        }
    }
    println!("sum is {}", sum);
}