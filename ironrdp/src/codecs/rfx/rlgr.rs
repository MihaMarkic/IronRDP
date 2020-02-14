#[cfg(test)]
mod tests;

use std::{cmp::min, io};

use bitvec::prelude::{BitField, BitSlice, BitStore, Msb0};
use failure::Fail;

use super::EntropyAlgorithm;
use crate::{impl_from_error, utils::Bits};

const KP_MAX: u32 = 80;
const LS_GR: u32 = 3;
const UP_GR: u32 = 4;
const DN_GR: u32 = 6;
const UQ_GR: u32 = 3;
const DQ_GR: u32 = 3;

macro_rules! write_byte {
    ($output:ident, $value:ident) => {
        if !$output.is_empty() {
            $output[0] = $value;
            $output = &mut $output[1..];
        } else {
            break;
        }
    };
}

#[macro_export]
macro_rules! try_split_bits {
    ($bits:ident, $n:expr) => {
        if $bits.len() < $n {
            break;
        } else {
            $bits.split_to($n)
        }
    };
}

pub fn decode(
    mode: EntropyAlgorithm,
    tile: &[u8],
    mut output: &mut [i16],
) -> Result<(), RlgrError> {
    let mut k: u32 = 1;
    let mut kr: u32 = 1;
    let mut kp: u32 = k << LS_GR;
    let mut krp: u32 = kr << LS_GR;

    if tile.is_empty() {
        return Err(RlgrError::EmptyTile);
    }

    let mut bits = Bits::new(BitSlice::from_slice(tile));
    while !bits.is_empty() && !output.is_empty() {
        match CompressionMode::from(k) {
            CompressionMode::RunLength => {
                let number_of_zeros = count_number_of_leading_value(&mut bits, false);
                try_split_bits!(bits, 1);
                let run = count_run(number_of_zeros, &mut k, &mut kp)
                    + load_be::<u32>(try_split_bits!(bits, k as usize));

                let sign_bit = try_split_bits!(bits, 1).load_be::<u8>();

                let number_of_ones = count_number_of_leading_value(&mut bits, true);
                try_split_bits!(bits, 1);

                let code_remainder = load_be::<u32>(try_split_bits!(bits, kr as usize))
                    + ((number_of_ones as u32) << kr);

                update_parameters_according_to_number_of_ones(number_of_ones, &mut kr, &mut krp);
                kp = kp.saturating_sub(DN_GR);
                k = kp >> LS_GR;

                let magnitude = compute_rl_magnitude(sign_bit, code_remainder);

                let size = min(run as usize, output.len());
                fill(&mut output[..size], 0);
                output = &mut output[size..];
                write_byte!(output, magnitude);
            }
            CompressionMode::GolombRice => {
                let number_of_ones = count_number_of_leading_value(&mut bits, true);
                try_split_bits!(bits, 1);

                let code_remainder = load_be::<u32>(try_split_bits!(bits, kr as usize))
                    + ((number_of_ones as u32) << kr);

                update_parameters_according_to_number_of_ones(number_of_ones, &mut kr, &mut krp);

                match mode {
                    EntropyAlgorithm::Rlgr1 => {
                        let magnitude = compute_rlgr1_magnitude(code_remainder, &mut k, &mut kp);
                        write_byte!(output, magnitude);
                    }
                    EntropyAlgorithm::Rlgr3 => {
                        let n_index = compute_n_index(code_remainder);

                        let val1 = load_be::<u32>(try_split_bits!(bits, n_index));
                        let val2 = code_remainder - val1;
                        if val1 != 0 && val2 != 0 {
                            kp = kp.saturating_sub(2 * DQ_GR);
                            k = kp >> LS_GR;
                        } else if val1 == 0 && val2 == 0 {
                            kp = min(kp + 2 * UQ_GR, KP_MAX);
                            k = kp >> LS_GR;
                        }

                        let magnitude = compute_rlgr3_magnitude(val1);
                        write_byte!(output, magnitude);

                        let magnitude = compute_rlgr3_magnitude(val2);
                        write_byte!(output, magnitude);
                    }
                }
            }
        }
    }

    // fill remaining buffer with zeros
    fill(output, 0);

    Ok(())
}

fn fill(buffer: &mut [i16], value: i16) {
    for v in buffer {
        *v = value;
    }
}

fn load_be<U>(s: &BitSlice<Msb0, u8>) -> U
where
    U: BitStore,
{
    if s.is_empty() {
        U::from(0)
    } else {
        s.load_be::<U>()
    }
}

fn count_number_of_leading_value(bits: &mut Bits<'_>, value: bool) -> usize {
    let number_of_zeros = bits
        .iter()
        .take_while(|&&v| v == value)
        .map(|_| 1)
        .sum::<usize>();
    bits.split_to(number_of_zeros);

    number_of_zeros
}

fn count_run(number_of_zeros: usize, k: &mut u32, kp: &mut u32) -> u32 {
    (0..number_of_zeros)
        .map(|_| {
            let run = 1 << *k;
            *kp = min(*kp + UP_GR as u32, KP_MAX as u32);
            *k = (*kp >> LS_GR as u32) as u32;

            run
        })
        .sum()
}

fn compute_rl_magnitude(sign_bit: u8, code_remainder: u32) -> i16 {
    if sign_bit != 0 {
        -((code_remainder + 1) as i16)
    } else {
        (code_remainder + 1) as i16
    }
}

fn compute_rlgr1_magnitude(code_remainder: u32, k: &mut u32, kp: &mut u32) -> i16 {
    if code_remainder == 0 {
        *kp = min(*kp + UQ_GR as u32, KP_MAX as u32);
        *k = (*kp >> LS_GR as u32) as u32;

        0
    } else {
        *kp = kp.saturating_sub(DQ_GR as u32);
        *k = (*kp >> LS_GR as u32) as u32;

        if code_remainder % 2 != 0 {
            -(((code_remainder + 1) >> 1) as i16)
        } else {
            (code_remainder >> 1) as i16
        }
    }
}

fn compute_rlgr3_magnitude(val: u32) -> i16 {
    if val % 2 != 0 {
        -(((val + 1) >> 1) as i16)
    } else {
        (val >> 1) as i16
    }
}

fn compute_n_index(code_remainder: u32) -> usize {
    if code_remainder != 0 {
        let code_bytes = code_remainder.to_be_bytes();
        let code_bits = BitSlice::<Msb0, u8>::from_slice(code_bytes.as_ref());
        let number_of_zeros = code_bits
            .iter()
            .take_while(|&&v| !v)
            .map(|_| 1)
            .sum::<usize>();

        32 - number_of_zeros
    } else {
        0
    }
}

fn update_parameters_according_to_number_of_ones(
    number_of_ones: usize,
    kr: &mut u32,
    krp: &mut u32,
) {
    if number_of_ones == 0 {
        *krp = (*krp).saturating_sub(2);
        *kr = *krp >> LS_GR;
    } else if number_of_ones > 1 {
        *krp = min(*krp + number_of_ones as u32, KP_MAX);
        *kr = *krp >> LS_GR;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum CompressionMode {
    RunLength,
    GolombRice,
}

impl From<u32> for CompressionMode {
    fn from(m: u32) -> Self {
        if m != 0 {
            Self::RunLength
        } else {
            Self::GolombRice
        }
    }
}

#[derive(Debug, Fail)]
pub enum RlgrError {
    #[fail(display = "IO error: {}", _0)]
    IoError(#[fail(cause)] io::Error),
    #[fail(display = "The input tile is empty")]
    EmptyTile,
}

impl_from_error!(io::Error, RlgrError, RlgrError::IoError);
