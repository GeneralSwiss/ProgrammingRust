mod gcd;
use std::env;
use std::str::FromStr;

fn main() {
    let mut nums: Vec<u64> = Vec::new();
    for arg in env::args().skip(1) {
        nums.push(u64::from_str(&arg).expect("Unable to parse argument into a number."));
    }

    let mut answer = nums[0];
    for num in &nums[1..] {
        answer = gcd::gcd(answer, *num);
    }

    println!("The GCD between {:?} is {}", nums, answer);
}
