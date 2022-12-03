use std::cmp;

use primes;

fn main() {
    const BASE_NUMBERS: [u64; 20] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20
    ];
    // const BASE_NUMBERS: [u64; 10] = [
    //     1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    // ];

    let mut counter: [u64; BASE_NUMBERS.len()] = [0; BASE_NUMBERS.len()];

    for base in BASE_NUMBERS {
        let factors: Vec<u64> = primes::factors(base.into());
        println!("base {} is factorized into {:?}", base, factors);
        for elem in BASE_NUMBERS {
            let counter_index: usize = (elem - 1).try_into().unwrap();
            let new_count: u64 = cmp::max(
                counter[counter_index],
                factors.iter().filter(|x| **x == elem).count()
                    .try_into().unwrap()
            );
            counter[counter_index] = new_count;
        }
    }

    println!("count is {:?}", counter);

    let mut answer: u64 = 1u64;
    for (index, count) in counter.iter().enumerate() {
        let base: u64 = (index + 1).try_into().unwrap();
        answer *= base.pow((*count).try_into().unwrap());
        // println!("answer {}", answer);
    }

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
