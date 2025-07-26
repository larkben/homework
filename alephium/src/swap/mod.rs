// swap Logic

// TODO
// .. perform some swaps / create instances
// .. crate unit tests
// .. perform math in notebook (newton's method for cubic iterations (newton iterations))
    // optimal number for imbalanced pools is 8 - 15 (more gas); and 3 - 6 for balanced

mod swap {

    enum SwapError {
        AmountIsZero,
        InsufficientLiquidity
    }

    struct Swap {
        token_one: String,
        token_two: String,
        reserve_one: u64,
        reserve_two: u64,
        fee: u64,
        fee_one: u64,
        fee_two: u64
    }

    impl Swap {

        // swap tokens with x * y = k
        fn swap(&mut self, amount_in: u64, token_in_is_one: bool) -> Result<u64, SwapError> {
            if amount_in == 0 {
                return Err(SwapError::AmountIsZero);
            }

            let (reserve_in, reserve_out) = if token_in_is_one {
                (self.reserve_one, self.reserve_two)
            } else {
                (self.reserve_two, self.reserve_one)
            };

            // Apply 0.03% fee: fee = 30 basis points = 30/10000
            let fee_amount = amount_in * self.fee / 10000;
            let amount_in_after_fee = amount_in - fee_amount;
            
            // Constant product formula: x * y = k
            // amount_out = (reserve_out * amount_in_after_fee) / (reserve_in + amount_in_after_fee)
            let amount_out = (reserve_out * amount_in_after_fee) / (reserve_in + amount_in_after_fee);
            
            if amount_out >= reserve_out {
                return Err(SwapError::InsufficientLiquidity);
            }

            // Update reserves and accrue fees
            if token_in_is_one {
                self.reserve_one += amount_in;
                self.reserve_two -= amount_out;
                self.fee_one += fee_amount;
            } else {
                self.reserve_two += amount_in;
                self.reserve_one -= amount_out;
                self.fee_two += fee_amount;
            }
            
            Ok(amount_out)
        }

        fn add_liquidity() {

        }

        fn remove_liquidty() {

        }
    }
}

mod stable_swap {

    enum StableSwapError {
        AmountIsZero,
        InsufficientLiquidity
    }
    
    struct StableSwap {
        token_one: String,
        token_two: String,
        reserve_one: u64,
        reserve_two: u64,
        fee: u64,
        fee_one: u64,
        fee_two: u64
    }

    impl StableSwap {

        // swap with stable swap formula x³y + y³x ≥ k
        fn swap(&mut self, amount_in: u64, token_in_is_one: bool) -> Result<u64, StableSwapError> {
            if amount_in == 0 {
                return Err(StableSwapError::AmountIsZero);
            }

            let (x, y) = if token_in_is_one {
                (self.reserve_one as f64, self.reserve_two as f64)
            } else {
                (self.reserve_two as f64, self.reserve_one as f64)
            };

            // Apply 0.03% fee
            let fee_amount = amount_in * self.fee / 10000;
            let dx = (amount_in - fee_amount) as f64;

            // Current invariant k = x³y + y³x
            let k = x.powi(3) * y + y.powi(3) * x;
            
            // New x after adding input
            let new_x = x + dx;
            
            // Solve for new_y: new_x³ * new_y + new_y³ * new_x = k
            // Factor out new_y: new_y * (new_x³ + new_y² * new_x) = k
            // This is a cubic equation: new_x * new_y³ + new_x³ * new_y - k = 0
            
            // Use Newton's method for cubic: f(y) = new_x * y³ + new_x³ * y - k
            // f'(y) = 3 * new_x * y² + new_x³
            let mut new_y = y; // Initial guess
            
            for _ in 0..10 { // Newton iterations
                let f = new_x * new_y.powi(3) + new_x.powi(3) * new_y - k;
                let f_prime = 3.0 * new_x * new_y.powi(2) + new_x.powi(3);
                
                if f_prime.abs() < 1e-12 {
                    break;
                }
                
                let new_y_next = new_y - f / f_prime;
                if (new_y_next - new_y).abs() < 1e-12 {
                    break;
                }
                new_y = new_y_next;
            }
            
            let amount_out = (y - new_y) as u64;
            
            if amount_out == 0 || new_y <= 0.0 {
                return Err(StableSwapError::InsufficientLiquidity);
            }

            // Update reserves and accrue fees
            if token_in_is_one {
                self.reserve_one += amount_in;
                self.reserve_two -= amount_out;
                self.fee_one += fee_amount;
            } else {
                self.reserve_two += amount_in;
                self.reserve_one -= amount_out;
                self.fee_two += fee_amount;
            }
            
            Ok(amount_out)
        }

        fn add_liquidity() {

        }

        fn remove_liquidty() {

        }
    }
}