mod market;
mod option;

use market::BinMarket;
use option::{crr_pricer, SpreadOption, SpreadType, VanillaOption, VanillaType};

fn main() {
    println!("Hello, Binomial Market!");

    let bm = BinMarket::new(100.0, -0.01, 0.01, 0.005);
    println!("{:?}", bm);

    let lo = 100.0;
    let hi = 200.0;
    let expiry: usize = 100;

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
