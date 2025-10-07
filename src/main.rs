use rand::prelude::*;
use roqoqo::{Circuit, operations::*};
use roqollage;
use roqoqo_qasm::*;

use std::fs::File;
use std::io::prelude::*;

fn euclid(a: u32, b: u32) -> u32 {
    let mut res = a % b;
    while res != 0 {
        println!("{res}");
        res = b % res;
    } 
    b
}

fn shor(number: u32) -> u32 {
    let mut rng = rand::rng();
    let base = rng.random::<u32>() % (number - 1) + 2;
    let gdc = euclid(number, base);

    println!("Number: {number}\nBase: {base}\nGDC: {gdc}\n");

    if gdc == 1 {
        return gdc;
    }

    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("bit_register".to_string(), 2, false);
    circuit.add_operation(Hadamard::new(0));

    let _ = roqollage::circuit_to_image(&circuit, None, roqollage::RenderPragmas::All, None, None)
        .expect("error")
        .save("circuit.png");

    let asm_vec = roqoqo_qasm::call_circuit(&circuit, "q", QasmVersion::V3point0(Qasm3Dialect::Roqoqo)).unwrap();

    let file = File::create("circuit.qasm");
    let mut openqasm: String = String::from("");
    for i in asm_vec {
        openqasm += &i;
        openqasm += "\n";
    }
    let _ = file 
        .expect("error")
        .write_all(openqasm.as_bytes());

    return 0;
}

fn main() {
    let number = 15; /* Taget number 3 * 5 = 15 */
    shor(number);
}
