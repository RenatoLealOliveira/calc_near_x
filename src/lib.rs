pub mod calc_with_1 {

/// # Essa função faz uma soma e adiciona 1
/// 
/// # exemplo
/// 
/// ```rust
/// use calc_near_x::calc_with_1::sum_plus_one;
/// 
/// assert_eq!(42, sum_plus_one(41,0));
/// assert_eq!(4, sum_plus_one(1,2));
/// assert_eq!(1, sum_plus_one(0,0));
/// ```

pub fn sum_plus_one(x: u8, y: u8) -> u8 {
        x + y + 1
    }

/// # Essa função faz uma subtração e subtrai 1
/// 
/// - Se o primeiro parametro for meno ou igual ao segundo, vai retornar 0
/// 
/// # exemplo
/// 
/// ```rust
/// use calc_near_x;
/// 
/// assert_eq!(40, calc_near_x::calc_with_1::sub_less_one(41,0));
/// assert_eq!(0, calc_near_x::calc_with_1::sub_less_one(6,6));
/// assert_eq!(0, calc_near_x::calc_with_1::sub_less_one(5,50));
/// ```   

pub fn sub_less_one(x: u8, y: u8) -> u8 {
        if x <= y {
            return 0;
        }
        x - y -1
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum() {
        let result = calc_with_1::sum_plus_one(5, 6);
        let expected: u8 = 12;
        assert_eq!(result, expected);

    }

    #[test]
    fn test_sub_failed2() {
        let result = calc_with_1::sub_less_one(6, 6);
        let expected: u8 = 0;
        assert_eq!(result, expected);

    }

    #[test]
    fn test_sub_failed() {
        let result = calc_with_1::sub_less_one(5, 6);
        let expected: u8 = 0;
        assert_eq!(result, expected);

    }

    #[test]
    fn test_sub() {
        let result = calc_with_1::sub_less_one(6, 6);
        let expected: u8 = 0;
        assert_eq!(result, expected);

    }



}