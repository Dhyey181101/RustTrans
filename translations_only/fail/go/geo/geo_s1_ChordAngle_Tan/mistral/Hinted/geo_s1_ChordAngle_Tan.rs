

use std::boxed::Box;

fn sum_(numbers: Box<Vec<i32>>) -> i32 {
 numbers.into_iter().sum()
}

fn main() {
 let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
 let sum1 = sum_(Box::new(numbers));
 println!("The sum is: {}", sum1);

 let sum2 = sum_(Box::new(vec![1, 2, 3, 4, 5]));
 println!("The sum is: {}", sum2);
}

