
use priority_queue::PriorityQueue;
use std::cmp::Ordering;

use crate::matcher::order;

#[derive(Eq, PartialEq)]
struct PricePriority {
    price : u64,
    side : order::Side,
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
pub struct Glass {
    buy_queue: PriorityQueue<order::Order, PricePriority>,
    sell_queue: PriorityQueue<order::Order, PricePriority>,
}

impl Glass {
    fn get_queue(&mut self, side: order::Side) -> &mut PriorityQueue<order::Order, PricePriority> {
        if side == order::Side::Buy {
            return &mut self.buy_queue;
        }
        return &mut self.sell_queue;
    }

    pub fn pop(&mut self, side: order::Side) -> Option<order::Order> {
        let q = self.get_queue(side);
        let res = q.pop();
        if res.is_some() {
            let (i, _p) = res.unwrap();
            return Some(i)
        }
        None
    }

    pub fn push(&mut self, o : order::Order) {
        let side = o.side();
        let q = self.get_queue(side);
        let price = o.price();
        q.push(o, PricePriority{price : price, side : side});
    }

}
