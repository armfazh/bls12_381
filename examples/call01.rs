use bls12_381::{pairing, G1Affine, G2Affine};

fn main() {
    println!("Example!");

    let g1 = G1Affine::generator();
    let g2 = G2Affine::generator();
    let g3 = pairing(&g1, &g2);

    print!("g1:\n{}\n", g1);
    print!("g2:\n{}\n", g2);
    print!("g3:\n{}\n", g3);
}
