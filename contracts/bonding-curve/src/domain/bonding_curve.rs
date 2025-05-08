use cosmwasm_std::Uint128;

pub struct BondingCurve {
    tier_prices: Vec<Uint128>,
    tokens_per_tier: u128,
    current_supply: Uint128,
    reserve_native_amount: Uint128,
    completed: bool,
    maximum_supply: Uint128,
}

pub struct BoughtEvent {
    pub tokens_bought: Uint128,
    pub rest_native_amount: Uint128,
}

impl BoughtEvent {
    pub fn new(tokens_bought: Uint128, rest_native_amount: Uint128) -> Self {
        BoughtEvent {
            tokens_bought,
            rest_native_amount,
        }
    }
}

pub struct SoldEvent {
    pub reserve_token_bought: Uint128,
    pub rest_tokens_amount: Uint128,
}

impl SoldEvent {
    pub fn new(reserve_token_bought: Uint128, rest_tokens_amount: Uint128) -> Self {
        SoldEvent {
            reserve_token_bought,
            rest_tokens_amount,
        }
    }
}

impl BondingCurve {
    pub fn new(
        tier_prices: Vec<Uint128>,
        tokens_per_tier: u128,
        maximum_supply: Uint128,
        current_supply: Uint128,
        reserve_native_amount: Uint128,
    ) -> Self {
        BondingCurve {
            tier_prices: tier_prices,
            tokens_per_tier: tokens_per_tier,
            current_supply: current_supply,
            reserve_native_amount: reserve_native_amount,
            completed: current_supply >= maximum_supply,
            maximum_supply,
        }
    }

    pub fn exp_bonding_curve(current_supply: Uint128, reserve_native_amount: Uint128) -> Self {
        let tier_prices = vec![
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(4u128),
            Uint128::from(4u128),
            Uint128::from(4u128),
            Uint128::from(4u128),
            Uint128::from(4u128),
            Uint128::from(4u128),
            Uint128::from(5u128),
            Uint128::from(5u128),
            Uint128::from(5u128),
            Uint128::from(5u128),
            Uint128::from(6u128),
            Uint128::from(6u128),
            Uint128::from(6u128),
            Uint128::from(6u128),
            Uint128::from(7u128),
            Uint128::from(7u128),
            Uint128::from(7u128),
            Uint128::from(8u128),
            Uint128::from(8u128),
            Uint128::from(8u128),
            Uint128::from(9u128),
            Uint128::from(9u128),
            Uint128::from(10u128),
            Uint128::from(10u128),
            Uint128::from(11u128),
            Uint128::from(11u128),
            Uint128::from(12u128),
            Uint128::from(12u128),
            Uint128::from(13u128),
            Uint128::from(13u128),
            Uint128::from(14u128),
            Uint128::from(15u128),
            Uint128::from(16u128),
            Uint128::from(16u128),
            Uint128::from(17u128),
            Uint128::from(18u128),
            Uint128::from(19u128),
            Uint128::from(20u128),
            Uint128::from(21u128),
            Uint128::from(22u128),
            Uint128::from(23u128),
            Uint128::from(24u128),
            Uint128::from(25u128),
            Uint128::from(27u128),
        ];
        let tokens_per_tier = 100_000_000_000u128;
        let maximum_supply = Uint128::from(12_000_000_000_000u128);
        BondingCurve::new(
            tier_prices,
            tokens_per_tier,
            maximum_supply,
            current_supply,
            reserve_native_amount,
        )
    }

    fn calculate_buy_tokens(&self, reserve_amount: Uint128) -> Result<(Uint128, Uint128), String> {
        if self.completed {
            return Err("Bonding curve is completed".to_string());
        }
        let mut remaining_reserve = reserve_amount;
        let mut total_tokens = 0u128;

        // Déterminer le palier actuel
        let current_tier = (self.current_supply.u128() / self.tokens_per_tier) as usize;
        let tokens_in_current_tier = self.current_supply.u128() % self.tokens_per_tier;

        // Parcours des paliers à partir du palier actuel
        for (i, &price) in self.tier_prices.iter().enumerate().skip(current_tier) {
            let available_tokens_in_tier = if i == current_tier {
                self.tokens_per_tier - tokens_in_current_tier // Tokens restants dans le palier actuel
            } else {
                self.tokens_per_tier // Tokens complets dans les paliers suivants
            };

            let tier_cost = price * Uint128::from(available_tokens_in_tier);

            if remaining_reserve >= tier_cost {
                // Peut acheter tous les tokens de ce palier
                total_tokens += available_tokens_in_tier;
                remaining_reserve -= tier_cost;
            } else {
                // Peut acheter partiellement dans ce palier
                let tokens_in_tier = remaining_reserve / price; // Nombre de tokens possibles dans ce palier
                total_tokens += tokens_in_tier.u128();
                remaining_reserve = Uint128::zero();
                break; // Fin du calcul
            }
        }

        Ok((Uint128::from(total_tokens), remaining_reserve))
    }

    fn calculate_sell_tokens(&self, token_amount: Uint128) -> Result<Uint128, String> {
        if self.completed {
            return Err("Bonding curve is completed".to_string());
        }
        if token_amount > self.current_supply {
            return Err("Not enough tokens to sell".to_string());
        }

        let mut remaining_tokens = token_amount.u128();
        let mut total_reserve = Uint128::zero();

        // Déterminer le palier actuel
        let current_tier = (self.current_supply.u128() / self.tokens_per_tier) as usize;
        let tokens_in_current_tier = self.current_supply.u128() % self.tokens_per_tier;

        // Parcours des paliers en sens inverse
        for (i, &price) in self
            .tier_prices
            .iter()
            .enumerate()
            .take(current_tier + 1)
            .rev()
        {
            let available_tokens_in_tier = if i == current_tier {
                tokens_in_current_tier // Tokens présents dans le palier actuel
            } else {
                self.tokens_per_tier // Tokens complets dans les paliers précédents
            };

            if remaining_tokens >= available_tokens_in_tier {
                // Peut vendre tous les tokens de ce palier
                total_reserve += Uint128::from(price.u128() * available_tokens_in_tier);
                remaining_tokens -= available_tokens_in_tier;
            } else {
                // Peut vendre partiellement dans ce palier
                total_reserve += Uint128::from(price.u128() * remaining_tokens);
                remaining_tokens = 0;
                break; // Fin du calcul
            }
        }

        // if remaining_tokens > 0 {
        //     return Err("Not enough tokens available in the bonding curve".to_string());
        // }

        Ok(total_reserve)
    }

    pub fn buy(&mut self, reserve_amount: Uint128) -> Result<BoughtEvent, String> {
        let (tokens_bought, remaining_reserve) = self.calculate_buy_tokens(reserve_amount)?;
        
        self.reserve_native_amount += reserve_amount - remaining_reserve;
        self.current_supply = self.current_supply + tokens_bought;
        if self.current_supply >= self.maximum_supply {
            self.completed = true;
        }
        
        Ok(BoughtEvent::new(tokens_bought, remaining_reserve))
    }

    pub fn sell(&mut self, token_amount: Uint128) -> Result<SoldEvent, String> {
        let total_reserve = self.calculate_sell_tokens(token_amount)?;

        // Mettre à jour les réserves et la supply
        self.current_supply -= token_amount;
        if self.current_supply <= Uint128::zero() {
            self.current_supply = Uint128::zero();
            self.reserve_native_amount = Uint128::zero();
            Ok(SoldEvent::new(total_reserve, Uint128::zero()))
        } else {
            self.reserve_native_amount -= total_reserve;
            Ok(SoldEvent::new(total_reserve, Uint128::zero()))
        }
    }

    pub fn calculate_buy_amount(&self, reserve_amount: Uint128) -> Result<Uint128, String> {
        let (tokens_bought, _) = self.calculate_buy_tokens(reserve_amount)?;
        Ok(tokens_bought)
    }

    pub fn calculate_sell_amount(&self, token_amount: Uint128) -> Result<Uint128, String> {
        self.calculate_sell_tokens(token_amount)
    }

    pub fn current_price(&self) -> Uint128 {
        let current_tier = (self.current_supply.u128() / self.tokens_per_tier) as usize;
        self.tier_prices[current_tier]
    }
}

#[cfg(test)]
mod test {

    fn approx_eq(a: u128, b: u128) -> bool {
        if a < b {
            let percent_value =
                Decimal::from_ratio(1u128, 1000u128) * Decimal::from_ratio(b, 1u128);
            return (b - a) < percent_value.to_uint_floor().u128();
        } else {
            let percent_value =
                Decimal::from_ratio(1u128, 1000u128) * Decimal::from_ratio(a, 1u128);
            return (a - b) < percent_value.to_uint_floor().u128();
        }
    }

    use cosmwasm_std::Decimal;

    use super::*;

    fn build_tier_prices() -> Vec<Uint128> {
        vec![
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(1u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(2u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(3u128),
            Uint128::from(4u128),
            Uint128::from(4u128),
            Uint128::from(4u128),
            Uint128::from(4u128),
            Uint128::from(4u128),
            Uint128::from(4u128),
            Uint128::from(5u128),
            Uint128::from(5u128),
            Uint128::from(5u128),
            Uint128::from(5u128),
            Uint128::from(6u128),
            Uint128::from(6u128),
            Uint128::from(6u128),
            Uint128::from(6u128),
            Uint128::from(7u128),
            Uint128::from(7u128),
            Uint128::from(7u128),
            Uint128::from(8u128),
            Uint128::from(8u128),
            Uint128::from(8u128),
            Uint128::from(9u128),
            Uint128::from(9u128),
            Uint128::from(10u128),
            Uint128::from(10u128),
            Uint128::from(11u128),
            Uint128::from(11u128),
            Uint128::from(12u128),
            Uint128::from(12u128),
            Uint128::from(13u128),
            Uint128::from(13u128),
            Uint128::from(14u128),
            Uint128::from(15u128),
            Uint128::from(16u128),
            Uint128::from(16u128),
            Uint128::from(17u128),
            Uint128::from(18u128),
            Uint128::from(19u128),
            Uint128::from(20u128),
            Uint128::from(21u128),
            Uint128::from(22u128),
            Uint128::from(23u128),
            Uint128::from(24u128),
            Uint128::from(25u128),
            Uint128::from(27u128),
        ]
    }

    #[test]
    fn create_bonding_curve_and_buy_all_supply() {
        let tier_prices: Vec<Uint128> = vec![
            Uint128::from(1u128),
            Uint128::from(2u128),
            Uint128::from(4u128),
            Uint128::from(10u128),
        ];
        let tokens_per_tier = 100_000_000_000u128;
        let maximum_supply = Uint128::from(400_000_000_000u128);
        let mut bonding_curve = BondingCurve::new(
            tier_prices.clone(),
            tokens_per_tier,
            maximum_supply,
            Uint128::zero(),
            Uint128::zero(),
        );
        let total_price = tier_prices
            .iter()
            .fold(0u128, |acc, x| acc + (x.u128() * tokens_per_tier));
        let bought = bonding_curve.buy(Uint128::from(total_price));
        assert!(bought.is_ok());
        let bought = bought.unwrap();
        assert_eq!(bought.tokens_bought, maximum_supply);
        assert_eq!(bought.rest_native_amount, Uint128::zero());
    }
    #[test]
    fn create_realistic_bonding_curve_and_buy_all_supply() {
        let tier_prices = build_tier_prices();
        let tokens_per_tier = 100_000_000_000u128;
        let maximum_supply = Uint128::from(12000000000000u128);
        let mut bonding_curve = BondingCurve::new(
            tier_prices.clone(),
            tokens_per_tier,
            maximum_supply,
            Uint128::zero(),
            Uint128::zero(),
        );
        let total_price = tier_prices
            .iter()
            .fold(0u128, |acc, x| acc + (x.u128() * tokens_per_tier));
        let bought = bonding_curve.buy(Uint128::from(total_price));
        println!("total price : {}", total_price);
        assert!(bought.is_ok());
        let bought = bought.unwrap();
        assert_eq!(bought.tokens_bought, maximum_supply);
        assert_eq!(bought.rest_native_amount, Uint128::zero());
    }

    #[test]
    fn create_realistic_bonding_curve_and_buy_all_supply_with_more_than_sufficient_native_amount() {
        let tier_prices = build_tier_prices();
        let tokens_per_tier = 100_000_000_000u128;
        let maximum_supply = Uint128::from(12000000000000u128);
        let mut bonding_curve = BondingCurve::new(
            tier_prices.clone(),
            tokens_per_tier,
            maximum_supply,
            Uint128::zero(),
            Uint128::zero(),
        );
        let total_price = tier_prices
            .iter()
            .fold(0u128, |acc, x| acc + (x.u128() * tokens_per_tier));
        let offer_amount = total_price * 2;
        let bought = bonding_curve.buy(Uint128::from(offer_amount));
        println!("total price : {}", total_price);
        assert!(bought.is_ok());
        let bought = bought.unwrap();
        assert_eq!(bought.tokens_bought, maximum_supply);
        assert_eq!(bought.rest_native_amount, Uint128::from(total_price));
    }

    #[test]
    fn buy_and_sell_same_amount() {
        let tier_prices = build_tier_prices();
        let tokens_per_tier = 100_000_000_000u128;
        let maximum_supply = Uint128::from(12000000000000u128);
        let mut bonding_curve = BondingCurve::new(
            tier_prices.clone(),
            tokens_per_tier,
            maximum_supply,
            Uint128::zero(),
            Uint128::zero(),
        );
        let first_tier_price = tier_prices[0] * Uint128::from(tokens_per_tier);
        let bought = bonding_curve.buy(first_tier_price);
        assert!(bought.is_ok());
        let bought = bought.unwrap();
        assert_eq!(bought.tokens_bought.u128(), tokens_per_tier);
        assert_eq!(
            bonding_curve.current_supply.u128(),
            bought.tokens_bought.u128()
        );
        assert_eq!(bought.rest_native_amount, Uint128::zero());
        let sold = bonding_curve.sell(bought.tokens_bought);
        assert!(sold.is_ok());
        let sold = sold.unwrap();
        assert_eq!(sold.reserve_token_bought, first_tier_price);
    }

    #[test]
    fn buy_10_times_and_sell_all_amount() {
        let tier_prices = build_tier_prices();
        let tokens_per_tier = 100_000_000_000u128;
        let maximum_supply = Uint128::from(12000000000000u128);
        let mut bonding_curve = BondingCurve::new(
            tier_prices.clone(),
            tokens_per_tier,
            maximum_supply,
            Uint128::zero(),
            Uint128::zero(),
        );
        let price = tier_prices[0] * Uint128::from(1_000_000_000_000u128);
        let mut bought_amount = vec![];
        for _ in 0..10 {
            let bought = bonding_curve.buy(price);
            assert!(bought.is_ok());
            let bought = bought.unwrap();
            assert_eq!(bought.rest_native_amount, Uint128::zero());
            bought_amount.push(bought.tokens_bought);
        }
        let amount_to_sell = bought_amount
            .iter()
            .fold(Uint128::zero(), |acc, x| acc + *x);
        let sold = bonding_curve.sell(amount_to_sell);
        assert!(sold.is_ok());

        assert_eq!(bonding_curve.current_supply, Uint128::zero());
        assert_eq!(bonding_curve.reserve_native_amount, Uint128::zero());
    }

    #[test]
    fn buy_10_times_and_sell_10_times_same_amount() {
        let tier_prices = build_tier_prices();
        let tokens_per_tier = 100_000_000_000u128;
        let maximum_supply = Uint128::from(12000000000000u128);
        let mut bonding_curve = BondingCurve::new(
            tier_prices.clone(),
            tokens_per_tier,
            maximum_supply,
            Uint128::zero(),
            Uint128::zero(),
        );
        let price = tier_prices[0] * Uint128::from(1_000_000_000_000u128);
        let mut bought_amount = vec![];
        for _ in 0..10 {
            let bought = bonding_curve.buy(price);
            assert!(bought.is_ok());
            let bought = bought.unwrap();
            assert_eq!(bought.rest_native_amount, Uint128::zero());
            bought_amount.push(bought.tokens_bought);
        }

        for b in bought_amount.iter().rev() {
            let sold = bonding_curve.sell(*b);
            assert!(sold.is_ok());
            let sold = sold.unwrap();
            assert!(approx_eq(sold.reserve_token_bought.u128(), price.u128()));
        }

        assert_eq!(bonding_curve.current_supply, Uint128::zero());
        assert_eq!(bonding_curve.reserve_native_amount, Uint128::zero());
    }

    #[test]
    fn buy_first_tiers() {
        let tier_prices: Vec<Uint128> = vec![
            Uint128::from(1u128),
            Uint128::from(2u128),
            Uint128::from(4u128),
            Uint128::from(10u128),
        ];
        let tokens_per_tier = 100_000u128;
        let maximum_supply = Uint128::from(400_000u128);
        let mut bonding_curve = BondingCurve::new(
            tier_prices.clone(),
            tokens_per_tier,
            maximum_supply,
            Uint128::zero(),
            Uint128::zero(),
        );
        let first_tier_price = tier_prices[0] * Uint128::from(tokens_per_tier);
        let bought = bonding_curve.buy(first_tier_price);
        assert!(bought.is_ok());
        let bought = bought.unwrap();
        assert_eq!(bought.tokens_bought.u128(), tokens_per_tier);
        assert_eq!(
            bonding_curve.current_supply.u128(),
            bought.tokens_bought.u128()
        );
        assert_eq!(bought.rest_native_amount, Uint128::zero());
    }
    #[test]
    fn buy_first_tier_and_half_second_tiers() {
        let tier_prices = build_tier_prices();
        let tokens_per_tier = 100_000_000_000u128;
        let maximum_supply = Uint128::from(12000000000000u128);
        let mut bonding_curve = BondingCurve::new(
            tier_prices.clone(),
            tokens_per_tier,
            maximum_supply,
            Uint128::zero(),
            Uint128::zero(),
        );
        let price = Uint128::from(150_000_000_000u128);
        let bought = bonding_curve.buy(price);
        assert!(bought.is_ok());
        let bought = bought.unwrap();
        assert_eq!(
            bought.tokens_bought.u128(),
            tokens_per_tier + (tokens_per_tier / 2)
        );
        assert_eq!(
            bonding_curve.current_supply.u128(),
            bought.tokens_bought.u128()
        );
        assert_eq!(bought.rest_native_amount, Uint128::zero());
    }
}
