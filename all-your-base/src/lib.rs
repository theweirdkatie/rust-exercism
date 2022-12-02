#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], _from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let mut base_10_num = 0;
    let mut result = vec![];

    if _from_base <= 1 {
        return Err(Error::InvalidInputBase);
    } else if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    } else {
        // convert to base 10
        for (n_place, val) in number.iter().rev().enumerate().rev() {
            if (0.._from_base).contains(val) {
                base_10_num += val * _from_base.pow(n_place.try_into().unwrap());
            } else {
                return Err(Error::InvalidDigit(*val));
            }
        }

        // convert to desired base
        loop {
            let remain = base_10_num % to_base;
            result.insert(0, remain);
            base_10_num /= to_base;
            if base_10_num == 0 {
                break;
            } else {
            };
        }
        Ok(result)
    }
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
