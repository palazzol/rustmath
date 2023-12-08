
mod field;

use field::Field;

fn main() {
    let mut f1 = Field::<f64>::one();
    let f2 = Field::<f64>::new(2.0);

    f1 += &f2;
    println!("f1={:?}", f1);
    f1 -= &f2;
    println!("f1={:?}", f1);
    f1 *= &f2;
    println!("f1={:?}", f1);
    f1 /= &f2;
    println!("f1={:?}", f1);

    let f5 = &f1+&f2;
    let f6 = &f1*&f2;

    let f9 = &f1-&f2;
    let f10 = &f1/&f2;

    println!("f5={:?}", f5);
    println!("f6={:?}", f6);

    println!("f9={:?}", f9);
    println!("f10={:?}", f10);

/*
    f1 += f2;
    println!("f1={:?}", f1);
    f1 -= f2;
    println!("f1={:?}", f1);
    f1 *= f2;
    println!("f1={:?}", f1);
    f1 /= f2;
    println!("f1={:?}", f1);

    let f3 = f1+f2;
    let f4 = f1*f2;

    let f7 = f1-f2;
    let f8 = f1/f2;

    println!("f3={:?}", f3);
    println!("f4={:?}", f4);

    println!("f7={:?}", f7);
    println!("f8={:?}", f8);
*/

    let t = Field::<f64>::one() == Field::<f64>::one();
    let f = Field::<f64>::one() == Field::<f64>::zero();
    let nef = Field::<f64>::one() != Field::<f64>::one();
    let net = Field::<f64>::one() != Field::<f64>::zero();
    let gtf = Field::<f64>::one() > Field::<f64>::one();
    let gtt = Field::<f64>::one() > Field::<f64>::zero();
    let ltf1 = Field::<f64>::one() < Field::<f64>::one();
    let ltf2 = Field::<f64>::one() < Field::<f64>::zero();
    let get1 = Field::<f64>::one() >= Field::<f64>::one();
    let get2 = Field::<f64>::one() >= Field::<f64>::zero();
    let let1 = Field::<f64>::one() <= Field::<f64>::one();
    let leff = Field::<f64>::one() <= Field::<f64>::zero();

    println!("t={:?}", t);
    println!("f={:?}", f);
    println!("nef={:?}", nef);
    println!("net={:?}", net);
    println!("gtf={:?}", gtf);
    println!("gtt={:?}", gtt);
    println!("ltf1={:?}", ltf1);
    println!("ltf2={:?}", ltf2);
    println!("get1={:?}", get1);
    println!("get2={:?}", get2);
    println!("let1={:?}", let1);
    println!("leff={:?}", leff);
}

// Get Field working with generic types - d64(R) impl and d64,d64(C)  for now
// Get Vector working with Fields
// Get Matrix working with Fields
// How would sparse matrix work?
// Multivector as field? vector?
// Move on to classes of Functions and their Representations

