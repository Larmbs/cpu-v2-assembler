//! Module for optimizing machine code

// These are problematic instructions and can usually be simplified
const MOVE: u16 = 3 << 12;

/// Optimizes the code you wrote to be more streamlined
pub fn optimize(code: Vec<u16>) -> Vec<u16> {
    let mut first_pass = Vec::with_capacity(code.len());
    // First pass removing any moves to same reg
    for line in code {
        // Checking if it is a move instruction and if the src and dest registers are the same
        if line & (0b1111 << 12) == MOVE && line & 0b1111 == (line & 0b1111_0000) >> 4 {
            continue;
        }
        first_pass.push(line)
    }

    first_pass
}
