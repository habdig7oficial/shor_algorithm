use roqoqo::{Circuit, operations::*};
use roqollage;
use roqoqo_qasm::*;

use std::fs::File;
use std::io::prelude::*;

use roqoqo::prelude::EvaluatingBackend;
use roqoqo_quest::Backend;


fn main() {
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("$r0".to_string(), 2, true);
    circuit += Hadamard::new(0);
    circuit += MeasureQubit::new(0, "$r0".to_string(), 0);

    let _ = roqollage::circuit_to_image(&circuit, None, roqollage::RenderPragmas::All, None, None)
        .expect("error")
        .save("circuit.png");

    let asm_vec = roqoqo_qasm::call_circuit(&circuit, "q", QasmVersion::V3point0(Qasm3Dialect::Roqoqo)).unwrap();

    let file = File::create("circuit.qasm");
    let mut openqasm: String = String::from("");
    for i in asm_vec {
        openqasm += &i;
        openqasm += "\n";
        println!("{i}")
    }
    let _ = file 
        .expect("error")
        .write_all(openqasm.as_bytes());

    let simulate = Backend::new(1, None);
    
    let res = simulate.run_circuit(&circuit).unwrap();

    println!("{:?}", res);

}
