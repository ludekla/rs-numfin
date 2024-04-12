/// Binomial Model for a single underlying.
#[derive(Debug)]
pub struct BinMarket {
    spot: f64,
    dtick: f64,
    utick: f64,
    rate: f64,
}

/// Implements 2 methods that make use of the market data.
/// martingal_prob computes the risk-free probability for an uptick,
/// and price computes the price of the underlying given a time
/// horizon a d the number of upticks.
impl BinMarket {
    // Associated factory.
    pub fn new(s: f64, d: f64, u: f64, r: f64) -> Self {
        BinMarket {
            spot: s,
            dtick: d,
            utick: u,
            rate: r,
        }
    }
    pub fn get_rate(&self) -> f64 {
        self.rate
    }
    pub fn martingal_prob(&self) -> f64 {
        (self.rate - self.dtick) / (self.utick - self.dtick)
    }
    pub fn price(&self, horizon: usize, uts: usize) -> f64 {
        let ups = uts as f64 * (1.0 + self.utick).ln();
        let dps = (horizon - uts) as f64 * (1.0 + self.dtick).ln();
        self.spot * (ups + dps).exp()
    }
}
