use std::{collections::HashMap, env};
use lazy_static::lazy_static;

const DEFAULT_OUT: &str = "a.dbmc";

fn main() {
    let mut args = env::args();
    args.next();

    let file_path = args.next().expect("You must provide a path to a dbasm file to compile");
    if !file_path.ends_with(".dbasm") {
        eprintln!("You must provide a .dbasm file, DroneBoi Assembly")
    }
    
    let output_path = args.next().unwrap_or(String::from(DEFAULT_OUT));

    println!("Hello, world!");
}

lazy_static! {
    /// A mapping between names and register enum
    static ref RegNameHash: HashMap<&'static str, Registers> = HashMap::from([
        ("rz", Registers::ZERO),
        ("r0", Registers::ZERO),
        ("ra", Registers::AluA),
        ("r1", Registers::AluA),
        ("rb", Registers::AluB),
        ("r2", Registers::AluB),
        ("ro", Registers::AluOut),
        ("r3", Registers::AluOut),
        ("rf", Registers::FLAGS),
        ("r4", Registers::FLAGS),
        ("rg1", Registers::GPR1),
        ("r5", Registers::GPR1),
        ("rg2", Registers::GPR2),
        ("r6", Registers::GPR2),
        ("rg3", Registers::GPR3),
        ("r7", Registers::GPR3),
    ]);
    static ref RegAddrHash: HashMap<Registers, u8> = HashMap::from([
        (Registers::ZERO, 0),
        (Registers::AluA, 1),
        (Registers::AluB, 2),
        (Registers::AluOut, 3),
        (Registers::FLAGS, 4),
        (Registers::GPR1, 5),
        (Registers::GPR2, 6),
        (Registers::GPR3, 7),
    ]);
}

/// Represents all registers on the CPU
#[derive(PartialEq, Hash, Eq)]
enum Registers {
    ZERO,
    AluA,
    AluB,
    AluOut,
    FLAGS,
    GPR1,
    GPR2,
    GPR3,
}
impl Registers {
    /// Parses out a register from a string representation
    fn parse_from_str(name: &str) -> Option<&Self> {
        RegNameHash.get(name)
    }
    /// Gets the address of the register
    fn to_addr(&self) -> &u8 {
        RegAddrHash.get(self).unwrap()
    }
}