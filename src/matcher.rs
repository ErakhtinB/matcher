pub mod matcher {

use std::collections::VecDeque;

mod order;
mod glass;

#[derive(Default)]
pub struct Matcher {
    g : glass::Glass,
}

#[derive(Copy, Clone)]
enum MatchResult {
    Ok,
    Discrepancy,
    SameUser,
    SameSide,
}

impl Matcher {
	fn process_lim(&mut self, mut o : order::Order) {
        o = self.common_processing(o);
        if o.current_qty() != 0 {
            o.print_due_external_event(order::ExternalEvent::Queued);
            self.g.push(o);
        }
    }
	fn process_ioc(&mut self, o : order::Order) {
        self.common_processing(o);
    }
	fn process_fok(&mut self, o : order::Order) {
        self.common_processing(o);
    }
    fn opposite_side(&self, side : order::Side) -> order::Side {
        if side == order::Side::Buy {
            return order::Side::Sell;
        }
        return order::Side::Buy;
    }
    fn orders_match(&self, lhs : &order::Order, rhs : &order::Order) -> MatchResult {
        let lhs_side = lhs.side();
        let rhs_side = rhs.side();
        if lhs_side == rhs_side {
            return MatchResult::SameSide;
        }
        if lhs.user_id() == rhs.user_id() {
            return MatchResult::SameUser;
        }
        let (mut a, mut b) = (0, 0);
        if lhs_side == order::Side::Buy {
            a = lhs.price();
            b = rhs.price();
        }
        else {
            a = rhs.price();
            b = lhs.price();
        }
        if a >= b {
            return MatchResult::Ok;
        }
        return MatchResult::Discrepancy;
    }
    pub fn proceed_record(&mut self, o : order::Order) {
        o.print_due_external_event(order::ExternalEvent::Accepted);
        match o.order_type() {
            order::OrderType::Lim => self.process_lim(o),
            order::OrderType::Ioc => self.process_ioc(o),
            order::OrderType::Fok => self.process_fok(o),
        }
    }
    fn common_processing(&mut self, mut o : order::Order) -> order::Order {
        let mut orders_to_recover: VecDeque<order::Order> = VecDeque::new();
        let o_side = self.opposite_side(o.side());
        loop {
            let opposite_order = self.g.pop(o_side);
            if opposite_order.is_none() {
                break;
            }
            let mut opposite_order = opposite_order.unwrap();
            match self.orders_match(&o, &opposite_order) {
                MatchResult::Ok => {
                        let order_current_qty = o.current_qty();
                        let opposite_order_current_qty = opposite_order.current_qty();
                        if order_current_qty > opposite_order_current_qty {
                            o.reduce_quantity(opposite_order_current_qty);
                            if o.order_type() == order::OrderType::Fok {
                                orders_to_recover.push_back(opposite_order);
                            }
                        }
                        else {
                            opposite_order.reduce_quantity(order_current_qty);
                            o.reduce_quantity(order_current_qty);
                            if opposite_order.current_qty() != 0 {
                                orders_to_recover.push_back(opposite_order);
                            }
                            break;
                        }
                    },
                MatchResult::SameSide => panic!("Orders of the same side"),
                MatchResult::SameUser => orders_to_recover.push_back(opposite_order),
                MatchResult::Discrepancy => {
                    orders_to_recover.push_back(opposite_order);
                    break; // put opposite order back, break, queue order
                },
            }
        }
        loop {
            let element = orders_to_recover.pop_back();
            if element.is_none() {
                break;
            }
            let element = element.unwrap();
            if o.order_type() == order::OrderType::Fok {
                if o.current_qty() == 0 {
                    if element.user_id() != o.user_id() {
                        continue;
                    }
                }
            }
            self.g.push(element);
        }
        o
    }
}

}
