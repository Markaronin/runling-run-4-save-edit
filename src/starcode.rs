use num::{BigInt, FromPrimitive};

const ALPHABET: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!$%/()=?,.-;:_^#+* @{[]}|~`";
const ENCRYPTION_KEY: &str = "WalkerKey";

fn alphabet_index(c: char) -> usize {
    ALPHABET
        .char_indices()
        .find_map(|(i, a)| if a == c { Some(i) } else { None })
        .unwrap()
}

fn shift_backward(c: char, d: char) -> char {
    let c_index = alphabet_index(c);
    let d_index = alphabet_index(d);
    let result_index = ((c_index + ALPHABET.len()) - d_index) % ALPHABET.len();
    ALPHABET.chars().nth(result_index).unwrap()
}
fn shift_forward(c: char, d: char) -> char {
    let c_index = alphabet_index(c);
    let d_index = alphabet_index(d);
    let result_index = ((c_index + ALPHABET.len()) + d_index) % ALPHABET.len();
    ALPHABET.chars().nth(result_index).unwrap()
}

pub fn decrypt(s: String) -> String {
    let mut result: String = String::new();
    for (i, c) in s.char_indices() {
        let key_index = i % ENCRYPTION_KEY.len();
        result.push(shift_backward(
            c,
            ENCRYPTION_KEY.chars().nth(key_index).unwrap(),
        ))
    }
    result
}
pub fn encrypt(s: String) -> String {
    let mut result: String = String::new();
    for (i, c) in s.char_indices() {
        let key_index = i % ENCRYPTION_KEY.len();
        result.push(shift_forward(
            c,
            ENCRYPTION_KEY.chars().nth(key_index).unwrap(),
        ))
    }
    result
}

pub fn compress(mut s: BigInt) -> String {
    let base = BigInt::from_usize(ALPHABET.len()).unwrap();
    let mut remainder: BigInt;
    let mut result = String::new();

    while s != BigInt::ZERO {
        remainder = &s % &base;
        result.push(
            ALPHABET
                .chars()
                .nth(remainder.to_string().parse().unwrap())
                .unwrap(),
        );
        s = s / &base;
    }

    result.chars().rev().collect()
}
pub fn uncompress(s: String) -> BigInt {
    let base = BigInt::from_usize(ALPHABET.len()).unwrap();
    let mut power = BigInt::from_usize(1).unwrap();
    let mut result = BigInt::ZERO;

    for c in s.chars().rev() {
        let index = alphabet_index(c);
        let digit_value = &power * BigInt::from_usize(index).unwrap();
        result = result + digit_value;
        power = &power * &base;
    }

    result
}

pub fn get_int(s: &mut BigInt, max_val: usize) -> usize {
    let max_val = BigInt::from_usize(max_val + 1).unwrap();
    let val = &*s % &max_val;
    *s = &*s / max_val;

    val.to_string().parse().unwrap()
}
pub fn store_int(s: &mut BigInt, val: usize, max_val: usize) {
    assert!(val <= max_val);

    let max_val = BigInt::from_usize(max_val + 1).unwrap();
    let val = BigInt::from_usize(val).unwrap();

    *s = (&*s * max_val) + val;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let encrypted_runling = "/Uni.I^uVUgUsr::M3I~IeI0".to_string();

        let decrypted_runling = decrypt(encrypted_runling.clone());

        assert_eq!(encrypt(decrypted_runling.clone()), encrypted_runling);

        let decompressed_runling = uncompress(decrypted_runling.clone());

        assert_eq!(compress(decompressed_runling.clone()), decrypted_runling);
    }
}
