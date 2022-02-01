use priority_queue::PriorityQueue;
use std::cmp::Ordering;

use crate::order;

#[derive(Eq, PartialEq)]
struct PricePriority {
    price: u64,
    side: order::Side,
}

impl PartialOrd for PricePriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PricePriority {
    fn cmp(&self, other: &Self) -> Ordering {
        let res = self.price.cmp(&other.price);
        if res == Ordering::Equal || self.side == order::Side::Buy {
            return res;
        }
        return res.reverse();
    }
}

#[derive(Default)]
pub struct Book {
    buy_queue: PriorityQueue<order::Order, PricePriority>,
    sell_queue: PriorityQueue<order::Order, PricePriority>,
}

impl Book {
    fn get_queue(&mut self, side: order::Side) -> &mut PriorityQueue<order::Order, PricePriority> {
        match side {
            order::Side::Buy => return &mut self.buy_queue,
            order::Side::Sell => return &mut self.sell_queue,
        }
    }

    pub fn pop(&mut self, side: order::Side) -> Option<order::Order> {
        if let Some(res) = self.get_queue(side).pop() {
            return Some(res.0);
        } else {
            return None;
        }
    }

    pub fn peek_mut(&mut self, side: order::Side) -> Option<&mut order::Order> {
        if let Some(res) = self.get_queue(side).peek_mut() {
            return Some(res.0);
        }
        return None;
    }

    pub fn push(&mut self, o: order::Order) {
        let side = o.side();
        let pp = PricePriority {
            price: o.price(),
            side: side,
        };
        self.get_queue(side).push(o, pp);
    }
}
