fn main() {
    println!("Hello, world!");
    println!("{}",subtract_product_and_sum(234));
}

pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut n = n;
    let mut sum = 0;
    let mut prod = 1;
    while n > 0 {
        sum += n % 10;
        prod *= n % 10;
        n /= 10;
    }
    return prod - sum;
}



mod solutions {
    pub mod question_1281 {

    }

    pub mod question_191 {
        pub fn _hamming_weight_improved(n: u32) -> i32 {
            let mut count = 0;
            let mut num = n;

            while num > 0 {
                num &= num - 1;
                count += 1 ;
            }
            return count;
        }

        pub fn _hamming_weight(n: u32) -> i32 {
            let mut count: i32 = 0;
            let mut lense = 1;

            for i in 0..32 {
                if (n & lense) >> i == 1 {
                    count += 1;
                }
                lense = lense << 1;
            }
            return count;
        }
    }

    pub mod question_1491{
        pub fn _average(salary: Vec<i32>) -> f64 {
            let mut total = 0;
            let mut extremes = (salary[0], 0);
            for v in &salary {
                if *v < extremes.0 {
                    extremes.0 = *v;
                } else if *v > extremes.1 {
                    extremes.1 = *v
                }
                total += *v
            }
            let (min, max)  = extremes;
            let total = total - min - max ;
            let result  = (total) as f64 / (salary.len() - 2) as f64;
            return result;
        }
    }

    pub mod question_1523 {
        pub fn _count_odds_naive(low: i32, high: i32) -> i32 {
            let mut current = low;
            let mut count = 0;
            while current <= high {
                if current % 2 != 0 {
                    count += 1
                }
                current += 1
            }
            return count;
        }

        pub fn _count_odds(low: i32, high: i32) -> i32 {
            return match high % 2 == 0 && low % 2 == 0 {
                true => (high - low) / 2,
                false => (high - low) / 2 + 1,
            };
        }
    }
}

