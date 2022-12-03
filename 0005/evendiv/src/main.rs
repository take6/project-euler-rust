use std::cmp;

use primes;

fn main() {
    const BASE_NUMBERS: [u64; 20] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20
    ];

    let mut counter: Vec<u64> = vec![0; BASE_NUMBERS.len()];

    BASE_NUMBERS.iter().for_each(|base| {
        let factors: Vec<u64> = primes::factors((*base).into());
        println!("base {} is factorized into {:?}", base, factors);
        counter = counter.iter()
            .enumerate()
            .map(|(counter_index, elem)| {
                cmp::max(
                    *elem,
                    factors.iter().filter(|x| **x == BASE_NUMBERS[counter_index])
                        .count()
                        .try_into().unwrap()
                )
            }).collect();
    });

    println!("count is {:?}", counter);

    let answer = counter.iter()
        .enumerate()
        .filter(|(_, x)| **x > 0)
        .fold(
            1u64,
            |acc, (i, x)| {
                let base: u64 = (i + 1).try_into().unwrap();
                acc * base.pow((*x).try_into().unwrap())
            }
        );

    // "evenly divisible" は商が偶数という意味かと思ったら
    // 違った。単に「割り切れる」という意味らしい。なので、
    // 以下のコードは不要
    //
    // let mut is_evenly_divisible: bool = false;
    // while !is_evenly_divisible {
    //     is_evenly_divisible = true;
    //     for base in BASE_NUMBERS {
    //         let quotient = answer / base;
    //         let modulo = answer % base;
    //         println!("{} / {} = {} ... {}", answer, base, quotient, modulo);
    //         if quotient % 2 != 0 {
    //             is_evenly_divisible = false;
    //             break;
    //         }
    //     }
    //     if !is_evenly_divisible {
    //         answer *= 2;
    //     }
    // }

    println!("answer = {}", answer);
}
