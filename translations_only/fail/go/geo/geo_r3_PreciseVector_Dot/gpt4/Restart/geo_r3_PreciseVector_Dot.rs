
use std::str::FromStr;

fn main() {
    let a = from_str("123456789012345678901234567890").unwrap();
    let b = from_str("987654321098765432109876543210").unwrap();
    let sum = add(&a, &b);
    let product = mul(&a, &b);
    println!("Sum: {:?}", sum);
    println!("Product: {:?}", product);
}

fn add(a: &BigUint, b: &BigUint) -> BigUint {
    let mut result = vec![0; a.data.len().max(b.data.len()) + 1];
    let mut carry = 0;

    for i in 0..result.len() {
        let a_val = *a.data.get(i).unwrap_or(&0);
        let b_val = *b.data.get(i).unwrap_or(&0);
        let sum = a_val + b_val + carry;
        result[i] = sum % 10;
        carry = sum / 10;
    }

    if carry > 0 {
        result.push(carry);
    }

    result.reverse();
    BigUint::new(result)
}

fn mul(a: &BigUint, b: &BigUint) -> BigUint {
    let mut result = vec![0; a.data.len() + b.data.len()];
    
    for (i, &a_val) in a.data.iter().enumerate() {
        for (j, &b_val) in b.data.iter().enumerate() {
            let sum = a_val * b_val + result[i + j];
            result[i + j] = sum % 10;
            result[i + j + 1] += sum / 10;
        }
    }

    while result.len() > 1 && result.last() == Some(&0) {
        result.pop();
    }

    result.reverse();
    BigUint::new(result)
}

fn from_str(s: &str) -> Result<BigUint, ()> {
    let data = s.bytes().map(|b| b - b'0').collect();
    Ok(BigUint::new(data))
}

#[derive(Debug, Clone)]
struct BigUint {
    data: Vec<u8>,
}

impl BigUint {
    fn new(data: Vec<u8>) -> Self {
        BigUint { data }
    }
}
