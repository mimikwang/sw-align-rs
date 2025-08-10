use std::io;

mod aligner;
mod matrix;

use crate::aligner::Aligner;

const VALID_BASE: [char; 4] = ['A', 'G', 'C', 'T'];

/// TGTTACGG GGTTGACTA
fn main() -> Result<(), &'static str> {
    let input = read().map_err(|_| "io error")?;
    let seqs = parse(input)?;
    let mut aligner = Aligner::new(seqs.0, seqs.1);
    aligner.build()?;
    println!("{aligner}");
    Ok(())
}

fn read() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.trim().into();
    Ok(input)
}

fn parse(input: String) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    let seqs: Vec<&str> = input.split(" ").collect();
    if seqs.len() != 2 {
        return Err("input incorrect");
    }
    let seq0 = string_to_vec(seqs[0])?;
    let seq1 = string_to_vec(seqs[1])?;
    Ok((seq0, seq1))
}

fn string_to_vec(seq: &str) -> Result<Vec<u8>, &'static str> {
    seq.chars()
        .map(|c| {
            if VALID_BASE.contains(&c) {
                return Ok(c as u8);
            }
            Err("invalid base")
        })
        .collect::<Result<Vec<u8>, &'static str>>()
}
