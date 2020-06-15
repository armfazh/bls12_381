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
    // print!("acc:\n{}\n", &adder);
    let g3 = miller_loop(&mut adder);
    print!("g3:\n{}\n", g3);
    let want = MillerLoopResult(g3).final_exponentiation();
    print!("want:\n{}\n", want);

    let mut res = getFp12();
    print!("x:\n{}\n", res);
    let got = MillerLoopResult(res).final_exponentiation();
    print!("got:\n{}\n", got);
    print!("{}\n", got == want);
}

fn getFp12() -> Fp12 {
    let x = [
        "0x1a39b1a05747244a93c9571c5f3149517162ec7384acf48b569567d020a59770e348d686508581b274a491b897638cd",
        "0xac9a831c8e7abbc349b1f1e5999ce341e2295b5cc396c1315cb4edde306017ed379fa4c1f1861e3345567ca3fd25cf9",
        "0x145edf22ffb53dda916c0151b7888abe46d483be40cfd47abe01c6e68cdbda7a58d1da5ef05f8d44ebf5a373d99bb706",
        "0x3aa4bd42717f0b5e26e93fab47c55a8a42d0f1135a09770abc9958dcb217e5a068f44c0b1c2f7dc4c6363afc953fe32",
        "0x15860abe99008809e6a8f2869acd125d68bef388a31a8e0a4f24d964c4c13aa6d51740a44790fc69ca14c8bd781106dc",
        "0x1292da4e6df7dc359c46e5c9680ecb00bb2c5219c49c2a4666d115d6a9343be2c99928eefb0970be711ade14fe665b64",
        "0x1371fea9c2faa7ba7446c42194ed94571e3332eca609f576a38afc0943707830e3f46d9a372ed76fe70ab921a32ff7a0",
        "0x16049a1007cb7a4430d4a979e10169e3dad9ddd8c0a833b85aae8dbd0a5ea3dfa424a5472ab7c6855187ca285b4f2b04",
        "0x7a0db36acbc7ca4269addf0347d4110ffb2e56903ad11072faa5d9d2073109ffaf7e45ce208d4d6076def48aea24ded",
        "0x7504a99181d320bd01333e789ac209ed7e58fcd401577507b5390b6358a333191941c7bfeae77800240f35e52773fe",
        "0x104fa8416563dbc8f8692497766ecddf1d8663d81d4b6447094dfbb9718a4572e66368de6874444e58cce2b217c791c2",
        "0xdb6c4880c5df9a4c6548a8606d1e33597d448fd9dff09e2a294bd629bb917c809812ba31ccb54f1a98ae374f0f81ab",
    ];
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
