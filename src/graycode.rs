pub fn generate_gray_code(n_bits: u8) -> Vec<u8> {
    assert!(n_bits > 0);

    if n_bits == 1 {
        vec![0u8, 1]
    } else {
        let last_code = generate_gray_code(n_bits - 1);

        let mut last_code_reversed = last_code.clone();
        last_code_reversed.reverse();

        for i in last_code_reversed.iter_mut() {
            *i |= 1 << (n_bits - 1);
        }

        [last_code, last_code_reversed].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_codes() {
        let code = generate_gray_code(1);
        println!("{code:?}");

        let code = generate_gray_code(2);
        println!("{code:?}");

        let code = generate_gray_code(3);
        println!("{code:?}");
    }
}
