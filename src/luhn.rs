pub fn luhn(cc_number: &str) -> bool {
    let string_number = cc_number.split_whitespace().collect::<String>();
    let mut sum = 0;
    
    if string_number.len() < 2 {
        return false;
    }
    
    for (i, char) in string_number.chars().rev().enumerate() {
        match char.to_digit(10) {
            Some(digit) => {
                sum += if i % 2 == 1 {
                    let dd = digit * 2;
                    dd / 10 + dd % 10
                } else {
                    digit
                }
            }
            None => return false
        }
    }

    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number1() {
    assert!(luhn("4263 9826 4026 9299"));
}
#[test]
fn test_valid_cc_number2() {
    assert!(luhn("4539 3195 0343 6467"));
}
#[test]
fn test_valid_cc_number3() {
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {
    let r = luhn("4263 9826 4026 9299");
    println!("{r}");
}
