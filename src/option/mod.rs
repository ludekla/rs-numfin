use crate::market::BinMarket;

pub enum VanillaType {
    Call,
    Put,
    DigitCall,
    DigitPut,
}

pub enum SpreadType {
    DoubleDigit,
    BearSpread,
    BullSpread,
}

/// Vanilla Payoff Functions.
fn call_payoff(strike: f64, underlying: f64) -> f64 {
    if underlying > strike {
        underlying - strike
    } else {
        0.0
    }
}

fn put_payoff(strike: f64, underlying: f64) -> f64 {
    if strike > underlying {
        strike - underlying
    } else {
        0.0
    }
}

fn dcall_payoff(strike: f64, underlying: f64) -> f64 {
    if underlying > strike {
        1.0
    } else {
        0.0
    }
}

fn dput_payoff(strike: f64, underlying: f64) -> f64 {
    if strike > underlying {
        1.0
    } else {
        0.0
    }
}

/// Spread Payoff Functions.
fn double_payoff(lower: f64, upper: f64, underlying: f64) -> f64 {
    if underlying < lower {
        0.0
    } else if underlying > upper {
        0.0
    } else {
        1.0
    }
}

fn bear_payoff(lower: f64, upper: f64, underlying: f64) -> f64 {
    if underlying < lower {
        upper - lower
    } else if underlying > upper {
        0.0
    } else {
        upper - underlying
    }
}

fn bull_payoff(lower: f64, upper: f64, underlying: f64) -> f64 {
    if underlying < lower {
        0.0
    } else if underlying > upper {
        upper - lower
    } else {
        underlying - lower
    }
}

pub trait Option {
    fn get_expiry(&self) -> usize;
    fn payoff(&self, underlying: f64) -> f64;
}

/// Plain Vanilla European Option: one strike price.
#[derive(Debug)]
pub struct VanillaOption {
    expiry: usize,
    strike: f64,
    poff: fn(f64, f64) -> f64,
}

/// Implements payoff function.
impl VanillaOption {
    pub fn new(tp: VanillaType, ex: usize, k: f64) -> Self {
        let poff = match tp {
            VanillaType::Call => call_payoff,
            VanillaType::Put => put_payoff,
            VanillaType::DigitCall => dcall_payoff,
            VanillaType::DigitPut => dput_payoff,
        };
        VanillaOption {
            expiry: ex,
            strike: k,
            poff,
        }
    }
}

impl Option for VanillaOption {
    fn get_expiry(&self) -> usize {
        self.expiry
    }
    fn payoff(&self, underlying: f64) -> f64 {
        (self.poff)(self.strike, underlying)
    }
}

/// European Spread Option: strike price range.
#[derive(Debug)]
pub struct SpreadOption {
    expiry: usize,
    lower: f64,
    upper: f64,
    poff: fn(f64, f64, f64) -> f64,
}

/// Implements payoff function.
impl SpreadOption {
    pub fn new(tp: SpreadType, ex: usize, lo: f64, hi: f64) -> Self {
        let poff = match tp {
            SpreadType::DoubleDigit => double_payoff,
            SpreadType::BearSpread => bear_payoff,
            SpreadType::BullSpread => bull_payoff,
        };
        SpreadOption {
            expiry: ex,
            lower: lo,
            upper: hi,
            poff,
        }
    }
}

impl Option for SpreadOption {
    fn get_expiry(&self) -> usize {
        self.expiry
    }
    fn payoff(&self, underlying: f64) -> f64 {
        (self.poff)(self.lower, self.upper, underlying)
    }
}

/// CRR pricing function
pub fn crr_pricer<O: Option>(bm: &BinMarket, opt: &O) -> f64 {
    let expiry = opt.get_expiry();
    let n = expiry + 1;
    let mut prices = Vec::with_capacity(n as usize);
    for i in 0..n {
        let price = bm.price(expiry, i);
        prices.push(opt.payoff(price));
    }
    let q = bm.martingal_prob();
    let up = q / (1.0 + bm.get_rate());
    let dn = (1.0 - q) / (1.0 + bm.get_rate());
    for t in (1..n).rev() {
        for i in 0..t {
            prices[i] = dn * prices[i] + up * prices[i + 1];
        }
    }
    prices[0]
}
