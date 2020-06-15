use bls12_381::fp::Fp;
use bls12_381::fp12::Fp12;
use bls12_381::fp2::Fp2;
use bls12_381::fp6::Fp6;
use bls12_381::{
    doubling_step, miller_loop, pairing, Adder, G1Affine, G2Affine, G2Projective, MillerLoopDriver,
    MillerLoopResult,
};
use num_bigint::BigUint;
use std::convert::From;

fn main() {
    println!("Example!");

    let p = G1Affine::generator();
    let q = G2Affine::generator();

    print!("g1:\n{}\n", p);
    print!("g2:\n{}\n", q);

    let mut adder = Adder {
        cur: G2Projective::from(q),
        base: q,
        p,
    };
    print!("acc:\n{}\n", &adder);
    let g3 = miller_loop(&mut adder);
    print!("\n");
    print!("g3:\n{}\n", g3);
    let gt = MillerLoopResult(g3).final_exponentiation();
    print!("gt:\n{}\n", gt);

    let mut res = getFp12();

    print!("x:\n{}\n", res);
    // let gt = MillerLoopResult(res).final_exponentiation();
    // print!("gt:\n{}\n", gt);
}

fn getFp12() -> Fp12 {
    let x = [
"0x12842d15e29bbbf893fab2fc6a320730519a70d88525742e3259705d29b68986a518388481584ec3766d6e936928b60f",
"0x8a63b19120e2ff412e0599872da34637827e381091878506dd7f91d40561fb4bb734c7ce4c2ae8b39afa1e916e4f1e5",
"0x18910dd491e261ae4989e59d03b4c77181e0d9ae249349c916851328fea86ecf350ea3dfa0a1c51a0500cc37a44ce36b",
"0x363bdd78ab55056fa1b3a3917771b54d036e95459c0c88497c9c58ad098851ad7a90dd68418cb8f5d37d5c31b42349d",
"0xfa0fd173c7a34391785f339c2739bb9a8e7b146ecad40f1cbc9f799854b6626aa31da6d78024ae1bf2c7978362328b5",
"0xa29bf3eb30ca5c288e943459f335817e37e44ae0d0e4e5a36f9436d8102bcb651c74f39893c7cea2473e97c5a20cc14",
"0x194e60103dcc323bec34ba58a5de8f9a75ea7cdc36ca4d683c1069f01bab428e6538737340feccad817de9127b22aa6f",
"0x19b0fdbe74acd77f0a40f1f9c53724589a85f9e1c0bd49f1a7ef147f95d6fcd630d5ae6163a61d8fc9f044192f695c81",
"0x135b5b846da5dce1320c1ce6a0006bbdec34690a69ca2056b557dedef7684c824ae0a47a0de5f695d1d4f30ad84d6e30",
"0x65436eaf325ed58c160af275356b427da3cdb4023e5122f0886aed318da4c4d1d9d4e172130c90a38a1d0dc9d667b71",
"0x12311ef33007dd9a9524b33f9e432b4c34aa1c8d78893f6f737b19340ac626bd3e7e678684015f9b8fe9c8d3ebc92477",
"0xf112498a18b475e20aca6881323ba5463a1dcb98eed8f30b6f0d4b9aa6ad361e2443870ecbbcd04d935e49a69cb6aae",];
    Fp12 {
        c0: Fp6 {
            c0: Fp2 {
                c0: Fp::from_str(&x[0][2..]).unwrap(),
                c1: Fp::from_str(&x[1][2..]).unwrap(),
            },
            c1: Fp2 {
                c0: Fp::from_str(&x[2][2..]).unwrap(),
                c1: Fp::from_str(&x[3][2..]).unwrap(),
            },
            c2: Fp2 {
                c0: Fp::from_str(&x[4][2..]).unwrap(),
                c1: Fp::from_str(&x[5][2..]).unwrap(),
            },
        },
        c1: Fp6 {
            c0: Fp2 {
                c0: Fp::from_str(&x[6][2..]).unwrap(),
                c1: Fp::from_str(&x[7][2..]).unwrap(),
            },
            c1: Fp2 {
                c0: Fp::from_str(&x[8][2..]).unwrap(),
                c1: Fp::from_str(&x[9][2..]).unwrap(),
            },
            c2: Fp2 {
                c0: Fp::from_str(&x[10][2..]).unwrap(),
                c1: Fp::from_str(&x[11][2..]).unwrap(),
            },
        },
    }
}
