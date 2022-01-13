pub mod matcher {

mod order;
mod glass;

pub struct Matcher {
    g : glass::Glass,
}

impl Matcher {
	fn process_lim(&self, o : order::Order) {
        panic!("Not implemented")
    }
	fn process_ioc(&self, o : order::Order) {
        panic!("Not implemented")
    }
	fn process_fok(&self, o : order::Order) {
        panic!("Not implemented")
    }
    fn opposite_side(side : order::Side) -> order::Side {
        if side == order::Side::Buy {
            return order::Side::Sell;
        }
        return order::Side::Buy;
    }
    pub fn proceed_record(&self, o : order::Order) {
        match o.order_type() {
            order::OrderType::Lim => self.process_lim(o),
            order::OrderType::Ioc => self.process_ioc(o),
            order::OrderType::Fok => self.process_fok(o),
        }
    }
}

}
