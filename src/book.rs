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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::order::{Order, OrderType, Side};

    #[test]
    fn test_book_push_and_pop() {
        let mut book = Book::default();

        // Create and push a buy order
        let buy_order = Order::new(OrderType::Lim, Side::Buy, 100, 10, 1);
        book.push(buy_order.clone());

        // Create and push a sell order
        let sell_order = Order::new(OrderType::Lim, Side::Sell, 110, 5, 2);
        book.push(sell_order.clone());

        // Test that we can get the orders back in the right order
        assert_eq!(book.pop(Side::Buy).unwrap(), buy_order);
        assert_eq!(book.pop(Side::Sell).unwrap(), sell_order);

        // Test that the queues are now empty
        assert!(book.pop(Side::Buy).is_none());
        assert!(book.pop(Side::Sell).is_none());
    }

    #[test]
    fn test_book_priority() {
        let mut book = Book::default();

        // Push buy orders with different prices
        let buy_order1 = Order::new(OrderType::Lim, Side::Buy, 100, 10, 1);
        let buy_order2 = Order::new(OrderType::Lim, Side::Buy, 102, 10, 2);
        book.push(buy_order1.clone());
        book.push(buy_order2.clone());

        // Higher priced buy order should have priority
        assert_eq!(book.pop(Side::Buy).unwrap(), buy_order2);
        assert_eq!(book.pop(Side::Buy).unwrap(), buy_order1);

        // Push sell orders with different prices
        let sell_order1 = Order::new(OrderType::Lim, Side::Sell, 105, 10, 3);
        let sell_order2 = Order::new(OrderType::Lim, Side::Sell, 103, 10, 4);
        book.push(sell_order1.clone());
        book.push(sell_order2.clone());

        // Lower priced sell order should have priority
        assert_eq!(book.pop(Side::Sell).unwrap(), sell_order2);
        assert_eq!(book.pop(Side::Sell).unwrap(), sell_order1);
    }

    #[test]
    fn test_peek_mut() {
        let mut book = Book::default();

        // Initially empty
        assert!(book.peek_mut(Side::Buy).is_none());

        let order = Order::new(OrderType::Lim, Side::Buy, 100, 10, 1);
        book.push(order);

        // Now we should be able to peek and modify
        let peeked = book.peek_mut(Side::Buy).unwrap();
        assert_eq!(peeked.price(), 100);

        // Modify the order quantity
        peeked.reduce_quantity(5);

        // Verify the change was applied
        let modified = book.pop(Side::Buy).unwrap();
        assert_eq!(modified.current_qty(), 5);
    }
}
