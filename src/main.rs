//main

use math::prelude::*;

fn main() {
    let x = Vector::new(1, 0, 3);
    let y = Vector::new(1, 2, 1);
    let z = Vector::new(2, 3, -1);

    let c = 1;
    let d = 4;
    let e = -2;

    let result = Vector::combine(c, x, d, y, e, z);
    dbg!(result);
}
