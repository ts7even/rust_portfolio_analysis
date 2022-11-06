

fn sortino_ratio(returns: &[f64], target_return: f64) -> f64 {
    let mut downside_returns = Vec::new();
    for &r in returns {
        if r < target_return {
            downside_returns.push(r);
        }
    }
    let downside_std = standard_deviation(&downside_returns);
    (mean(returns) - target_return) / downside_std
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

pub fn main() {
    let returns = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    let target_return = 0.5;
    let sortino_ratio = sortino_ratio(&returns, target_return);
    println!("Sortino ratio: {}", sortino_ratio);
}