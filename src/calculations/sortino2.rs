/*  
    Here I'm using the sotrino ratio but with 3rd party crates such as ndarray and ndarray-stats.
I'm also using Damodarsan's ERP method for forward looking expected returns (CAPM).
You can think of the last example as a ratio to present to your client to show the past performance of your portfolio, but this is not likley to
continue into the future. 

*/

//use ndarray::prelude::*;


use rand;
use rand::Rng;
fn sortino_ratio(returns: &[f64], risk_free_rate: f64, capm: f64) -> f64 {
    let mut downside_returns = Vec::new();
    for &r in returns {
        if r < risk_free_rate {
            downside_returns.push(r);
        }
    }
    let downside_std = standard_deviation(&downside_returns);
    (capm - risk_free_rate) / downside_std
   
}

fn mean(returns: &[f64]) -> f64 {
    returns.iter().sum::<f64>() / returns.len() as f64
}

fn standard_deviation(returns: &[f64]) -> f64 {
    let mean = mean(returns);
    let mut sum = 0.0;
    for &r in returns {
        sum += (r - mean).powi(2);
    }
    (sum / (returns.len() - 1) as f64).sqrt()
}

// Create 20 Random float point numbers for returns between -0.3 and 0.3
fn random_returns() -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut returns = Vec::new();
    for _ in 0..20 {
        returns.push(rng.gen_range(-0.3..0.3));
    }
    returns
}


pub fn main() {
    let returns = random_returns();
    let risk_free_rate = 0.041;
    let market_expected_return = 0.053;
    let equity_risk_premium = market_expected_return - risk_free_rate; 
    let beta = 1.27;
    let capm: f64 = risk_free_rate + (equity_risk_premium * beta);
    let sortino_ratio = sortino_ratio(&returns, risk_free_rate, capm);
    println!("Sortino ratio forward looking: {}", sortino_ratio);
    println!("Capm: {}", capm);
    println!("Returns Vector: {:?}", returns);
 }