use rand::prelude::*;

fn euclid(a: i32, b: i32) -> i32 {
    let mut res = a % b;
    while res != 0 {
        println!("{res}");
        res = b % res;
    } 
    b
}

fn main() {
    let n = 15 /* Taget number 3 * 5 = 15 */

}
