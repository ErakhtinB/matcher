use std::collections::VecDeque;

pub mod book;
pub mod order;

#[derive(Default)]
pub struct Matcher {
    g: book::Book,
    orders_to_recover: VecDeque<order::Order>,
}

#[derive(Copy, Clone)]
enum MatchResult {
    Ok,
    Discrepancy,
    SameUser,
    SameSide,
}

impl Matcher {
    pub fn new() -> Matcher {
        Matcher {
            ..Default::default()
        }
    }
}

fn choose_prices(lhs: &order::Order, rhs: &order::Order) -> (u64, u64) {
    if lhs.side() == order::Side::Buy {
        return (lhs.price(), rhs.price());
    } else {
        return (rhs.price(), lhs.price());
    }
}

fn opposite_side(o: &order::Order) -> order::Side {
    if o.side().eq(&order::Side::Buy) {
        return order::Side::Sell;
    } else {
        return order::Side::Buy;
    }
}

fn orders_match(lhs: &order::Order, rhs: &order::Order) -> MatchResult {
    let lhs_side = lhs.side();
    let rhs_side = rhs.side();
    if lhs_side == rhs_side {
        return MatchResult::SameSide;
    }
    if lhs.user_id() == rhs.user_id() {
        return MatchResult::SameUser;
    }
    let (a, b) = choose_prices(lhs, rhs);
    if a >= b {
        return MatchResult::Ok;
    }
    return MatchResult::Discrepancy;
}

impl Matcher {
    fn put_recovered_orders_back(&mut self) {
        let mut reconfig_needed = false;
        let mut border_price: u64 = 0;
        let mut side = order::Side::Buy;
        if let Some(o) = self.orders_to_recover.back() {
            side = o.side();
            border_price = o.price();
            if let Some(front) = self.g.peek_mut(o.side()) {
                if front.price() == border_price {
                    reconfig_needed = true;
                }
            }
        }
        if reconfig_needed {
            loop {
                if let Some(front) = self.g.peek_mut(side) {
                    if front.price() != border_price {
                        break;
                    }
                } else {
                    break;
                }
                if let Some(front) = self.g.pop(side) {
                    self.orders_to_recover.push_back(front);
                }
            }
        }
        while let Some(o) = self.orders_to_recover.pop_front() {
            self.g.push(o);
        }
    }
    fn process_lim(&mut self, mut o: order::Order) {
        o = self.common_processing(o);
        if o.current_qty() != 0 {
            o.print_due_external_event(order::ExternalEvent::Queued);
            self.g.push(o);
        }
    }
    fn process_ioc(&mut self, o: order::Order) {
        self.common_processing(o);
    }
    fn process_fok(&mut self, mut o: order::Order) {
        if !self.orders_to_recover.is_empty() {
            panic!("Orders to recover queue is not empty in the start of fok-processing");
        }
        while let Some(mut opposite_order) = self.g.pop(opposite_side(&o)) {
            match orders_match(&o, &opposite_order) {
                MatchResult::Ok => {
                    let order_current_qty = o.current_qty();
                    let opposite_order_current_qty = opposite_order.current_qty();
                    if order_current_qty > opposite_order_current_qty {
                        o.reduce_quantity(opposite_order_current_qty);
                        self.orders_to_recover.push_back(opposite_order);
                    } else {
                        opposite_order.reduce_quantity(order_current_qty);
                        o.reduce_quantity(order_current_qty);
                        if order_current_qty < opposite_order_current_qty {
                            self.orders_to_recover.push_back(opposite_order);
                        }
                        break;
                    }
                }
                MatchResult::SameSide => panic!("Orders of the same side"),
                MatchResult::SameUser | MatchResult::Discrepancy => {
                    self.orders_to_recover.push_back(opposite_order);
                    break;
                }
            }
        }
        self.put_recovered_orders_back();
    }
    pub fn proceed_record(&mut self, o: order::Order) {
        o.print_due_external_event(order::ExternalEvent::Accepted);
        match o.order_type() {
            order::OrderType::Lim => self.process_lim(o),
            order::OrderType::Ioc => self.process_ioc(o),
            order::OrderType::Fok => self.process_fok(o),
        }
    }
    fn common_processing(&mut self, mut o: order::Order) -> order::Order {
        let o_side = opposite_side(&o);
        while let Some(opposite_order) = self.g.peek_mut(o_side) {
            match orders_match(&o, &opposite_order) {
                MatchResult::Ok => {
                    let order_current_qty = o.current_qty();
                    let opposite_order_current_qty = opposite_order.current_qty();
                    if order_current_qty > opposite_order_current_qty {
                        o.reduce_quantity(opposite_order_current_qty);
                        self.g.pop(o_side);
                    } else {
                        opposite_order.reduce_quantity(order_current_qty);
                        o.reduce_quantity(order_current_qty);
                        if opposite_order_current_qty == order_current_qty {
                            self.g.pop(o_side);
                        }
                        break;
                    }
                }
                MatchResult::SameSide => panic!("Orders of the same side"),
                MatchResult::SameUser | MatchResult::Discrepancy => break,
            }
        }
        return o;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_order_matching() {
        // Create a matcher
        let mut matcher = Matcher::new();

        // Add a buy limit order
        let buy_order = order::Order::new(
            order::OrderType::Lim,
            order::Side::Buy,
            100, // price
            10,  // quantity
            1,   // user_id
        );

        // Add a matching sell limit order
        let sell_order = order::Order::new(
            order::OrderType::Lim,
            order::Side::Sell,
            95, // price (lower than buy price, so should match)
            5,  // quantity (half of buy quantity)
            2,  // user_id
        );

        // Process the orders
        matcher.proceed_record(buy_order);
        matcher.proceed_record(sell_order);

        // The buy order should be partially filled and remain in the book
        // The sell order should be fully executed

        // Add another sell order to match remaining buy quantity
        let sell_order2 = order::Order::new(
            order::OrderType::Lim,
            order::Side::Sell,
            98, // price (still lower than buy price)
            5,  // quantity (remaining buy quantity)
            3,  // user_id
        );

        matcher.proceed_record(sell_order2);

        // Now the buy order should be fully executed and removed from the book
    }

    #[test]
    fn test_fok_order_matching() {
        // Create a matcher
        let mut matcher = Matcher::new();

        // Add a buy limit order to the book
        let buy_limit = order::Order::new(
            order::OrderType::Lim,
            order::Side::Buy,
            100, // price
            10,  // quantity
            1,   // user_id
        );
        matcher.proceed_record(buy_limit);

        // Try to match with a FOK sell order that's too large to fill completely
        let large_fok_sell = order::Order::new(
            order::OrderType::Fok,
            order::Side::Sell,
            95, // price (acceptable)
            15, // quantity (more than available)
            2,  // user_id
        );

        // This should be canceled
        matcher.proceed_record(large_fok_sell);

        // Try a FOK sell that can be filled
        let matching_fok_sell = order::Order::new(
            order::OrderType::Fok,
            order::Side::Sell,
            95, // price
            10, // quantity (exact match)
            3,  // user_id
        );

        // This should execute
        matcher.proceed_record(matching_fok_sell);

        // The buy limit order should now be fully executed
    }

    #[test]
    fn test_ioc_order_matching() {
        // Create a matcher
        let mut matcher = Matcher::new();

        // Add a buy limit order to the book
        let buy_limit = order::Order::new(
            order::OrderType::Lim,
            order::Side::Buy,
            100, // price
            10,  // quantity
            1,   // user_id
        );
        matcher.proceed_record(buy_limit);

        // Match with an IOC sell order
        let ioc_sell = order::Order::new(
            order::OrderType::Ioc,
            order::Side::Sell,
            95, // price (acceptable)
            15, // quantity (more than available)
            2,  // user_id
        );

        // Should partially execute (10 units) and cancel the rest (5 units)
        matcher.proceed_record(ioc_sell);

        // The buy limit order should now be fully executed
    }
}
