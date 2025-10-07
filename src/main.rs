use rand::prelude::*;
use roqoqo::{Circuit, operations:*}

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

    return 0;
}

fn main() {
    let number = 15; /* Taget number 3 * 5 = 15 */
    shor(number);
}
