use std::f64::consts::E;

const K: f64 = 2.8e-9;

pub fn cal_price(x: i32, initial: &f64) -> f64 {
    let tot_price = (initial * (E.powf(K * x as f64) - 1.0)) / K;
    tot_price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_calculation() {
        let amts = vec![
        2.0, 10.0, 100.0, 200.0, 1000.0, 10000.0, 20000.0, 
        210000.0, 1000000.0, 20000000.0, 30000000.0, 2.0, 4.0, 
        100000000.0, 1.0, 200000000.0, 300000000.0
    ];
    let mut curr_price = 2.8e-8;

    for amt in amts {
        let price = cal_price(amt as i32, &curr_price);
        println!(
            "lot: {:.1} : Total cost: {:.12} | Cost per token: {:.12}", 
            amt, 
            price, 
            price / amt
        );
        curr_price = curr_price * E.powf(K * amt);
    }
    }
}