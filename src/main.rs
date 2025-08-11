mod aligner;
mod errors;
mod input;
mod matrix;

use crate::aligner::Aligner;
use crate::errors::Result;
use crate::input::get_input;

fn main() -> Result<()> {
    let seqs = get_input()?;
    let mut aligner = Aligner::new(seqs.0, seqs.1);
    aligner.build()?;
    print!("{aligner}");
    Ok(())
}
