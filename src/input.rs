use std::env;

use crate::Result;

const VALID_BASE: [char; 4] = ['A', 'G', 'C', 'T'];

pub fn get_input() -> Result<(Vec<u8>, Vec<u8>)> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("3 arguments expected");
    }

    let seq0 = string_to_vec(&args[1])?;
    let seq1 = string_to_vec(&args[2])?;

    Ok((seq0, seq1))
}

fn string_to_vec(seq: &str) -> Result<Vec<u8>> {
    seq.chars()
        .map(|c| {
            if VALID_BASE.contains(&c) {
                return Ok(c as u8);
            }
            Err("invalid base")
        })
        .collect::<Result<Vec<u8>>>()
}