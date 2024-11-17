use std::f64::consts::E;

// const K: f64 = 0.0000000284201;
const K: f64 = 0.0000000084201;
const INITIAL_PRICE: f64 = 0.000000028;

pub fn calculate_price(amount: f64, tokens_sold: f64) -> f64 {
    let current_price = INITIAL_PRICE * E.powf(K * tokens_sold as f64);
    let total_price = (current_price * (E.powf(K * amount as f64) - 1.0)) / K;
    
    // current_price *= E.powf(K * amount as f64);
    
    total_price
}

pub fn get_current_price(tokens_sold: f64) -> f64 {
    INITIAL_PRICE * E.powf(K * tokens_sold as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_calculation() {
        let test_amounts = vec![
            0.1, 0.2, 0.5, 2.0, 10.0, 100.0, 200.0, 1000.0, 10000.0, 20000.0, 210000.0, 
            1000000.0, 20000000.0, 30000000.0, 200000000.0,1.0, 2.0, 1000000.0, 300000000.0, 200000000.0
        ];
        
        let mut tokens_sold = 0.0;
        
        for &amount in test_amounts.iter() {
            let price = calculate_price(amount, tokens_sold);
            println!(
                "Lot: {} | Total cost: {:.12} | Cost per token: {:.12} | Current Price: {:.12}",
                amount,
                price,
                price / amount as f64,
                get_current_price(tokens_sold)
            );
            tokens_sold += amount;
        }
    }
}