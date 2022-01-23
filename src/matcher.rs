use std::collections::VecDeque;

pub mod glass;
pub mod order;

#[derive(Default)]
pub struct Matcher {
    g: glass::Glass,
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

fn opposite_side(side: order::Side) -> order::Side {
    if side == order::Side::Buy {
        return order::Side::Sell;
    }
    return order::Side::Buy;
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
    fn process_fok(&mut self, o: order::Order) {
        self.fok_processing(o);
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
        let o_side = opposite_side(o.side());
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
    fn fok_processing(&mut self, mut o: order::Order) -> order::Order {
        if !self.orders_to_recover.is_empty() {
            panic!("Orders to recover queue is not empty in the start of fok-processing");
        }
        while let Some(mut opposite_order) = self.g.pop(opposite_side(o.side())) {
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
        while let Some(e) = self.orders_to_recover.pop_back() {
            self.g.push(e);
        }
        return o;
    }
}
