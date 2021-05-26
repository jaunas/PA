use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();
    
    let mut boundaries = input
        .split_whitespace()
        .map(
            |s| s.parse::<u32>()
        );
    
    let left = boundaries.next().unwrap().unwrap();
    let right = boundaries.next().unwrap().unwrap();

    println!("{}", count_p(left, right));
}

fn count_p(left: u32, right: u32) -> u32 {
    let mut count = 0;

    for number in left..right {
        if is_p_num(number) {
            count += 1;
        }
    }

    count
}

fn is_p_num(num: u32) -> bool {
    for digit in get_digits(num) {
        if digit == 0 || num % digit != 0 {
            return false
        }
    }

    true
}

fn get_digits(num: u32) -> Vec<u32> {
    num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_digits_test() {
        let digits = get_digits(123);

        assert_eq!(digits.len(), 3);

        assert_eq!(digits[0], 1);
        assert_eq!(digits[1], 2);
        assert_eq!(digits[2], 3);
    }

    #[test]
    fn is_p_num_test() {
        assert_eq!(true, is_p_num(1));
        assert_eq!(true, is_p_num(7));
        assert_eq!(true, is_p_num(15));
        assert_eq!(true, is_p_num(48));

        assert_eq!(false, is_p_num(13));
        assert_eq!(false, is_p_num(47));
        assert_eq!(false, is_p_num(100));
    }
}