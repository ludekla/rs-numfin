use std::env;

mod market;
mod option;
use market::BinMarket;
use option::{crr_pricer, SpreadOption, SpreadType, VanillaOption, VanillaType};

fn main() {
    println!("Hello, Binomial Market CRR Pricer!");

    // Fetch command-line arguments.
    let mut args: Vec<f64> = Vec::with_capacity(3);
    for arg in env::args() {
        if let Ok(val) = arg.parse::<f64>() {
            args.push(val);
        }
    }

    let spot = 100.0;
    let (dtick, utick) = (-0.01, 0.01);
    let rate = 0.005;

    let bm = BinMarket::new(spot, dtick, utick, rate);
    println!("{:?}", bm);

    let expiry = args[0] as usize;
    let lo = args[1];
    let hi = args[2];

    let call = VanillaOption::new(VanillaType::Call, expiry, lo);
    println!("Call {:9.5}", crr_pricer(&bm, &call));

    let put = VanillaOption::new(VanillaType::Put, expiry, hi);
    println!("Put  {:9.5}", crr_pricer(&bm, &put));

    let dcall = VanillaOption::new(VanillaType::DigitCall, expiry, lo);
    println!("Digital Call {:9.5}", crr_pricer(&bm, &dcall));

    let dput = VanillaOption::new(VanillaType::DigitPut, expiry, hi);
    println!("Digital Put  {:9.5}", crr_pricer(&bm, &dput));

    let dopt = SpreadOption::new(SpreadType::DoubleDigit, expiry, lo, hi);
    println!("Double Digital Option: {:9.5}", crr_pricer(&bm, &dopt));

    let bear = SpreadOption::new(SpreadType::BearSpread, expiry, lo, hi);
    println!("Bear Spread Option:    {:9.5}", crr_pricer(&bm, &bear));

    let bull = SpreadOption::new(SpreadType::BullSpread, expiry, lo, hi);
    println!("Bull Spread Option:    {:9.5}", crr_pricer(&bm, &bull));
}
