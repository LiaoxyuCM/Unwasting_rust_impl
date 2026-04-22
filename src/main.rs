pub mod unwasting {
    pub fn unwaste_compile(s: &str) -> String {
        let mut chars = Vec::new();
        let mut cells = [0u32; 10];
        let mut result = String::new();
        let mut last_most_close_index = 0usize;

        for ch in s.chars() {
            chars.push(ch as u32);
        }

        for &num in &chars {
            let mut most_close = u32::MAX;
            let mut most_close_index = 0;

            for (i, &cell_num) in cells.iter().enumerate() {
                let diff = cell_num.abs_diff(num);
                if diff < most_close {
                    most_close = diff;
                    most_close_index = i;
                }
            }

            if last_most_close_index != most_close_index {
                result.push_str(&most_close_index.to_string());
            }
            last_most_close_index = most_close_index;

            let cell_val = &mut cells[most_close_index];

            if *cell_val > num {
                let diff = *cell_val - num;
                let binary = format!("{:b}", diff);
                result.push_str(&"[".repeat(binary.len() - 1));

                let mut bits: Vec<char> = binary.chars().collect();
                let last_bit = bits.pop().unwrap();

                for bit in bits {
                    if bit == '1' {
                        result.push_str("-]");
                    } else {
                        result.push_str("]");
                    }
                }

                if last_bit == '1' {
                    result.push('-');
                }
            } else if *cell_val < num {
                let diff = num - *cell_val;
                let binary = format!("{:b}", diff);
                result.push_str(&"[".repeat(binary.len() - 1));

                let mut bits: Vec<char> = binary.chars().collect();
                let last_bit = bits.pop().unwrap();

                for bit in bits {
                    if bit == '1' {
                        result.push_str("+]");
                    } else {
                        result.push_str("]");
                    }
                }

                if last_bit == '1' {
                    result.push('+');
                }
            }

            *cell_val = num;
            result.push('.');
        }

        result
    }
}

use unwasting::unwaste_compile;
use std::io;
fn main() {
    let mut str_input = String::new();
    loop {
        match io::stdin().read_line(&mut str_input){
            Ok(_) => {
                println!("{}", unwaste_compile(&str_input.trim_end_matches(&['\r', '\n'] as &[_])));
            },
            Err(_) => break
        }
    }
    ()
}