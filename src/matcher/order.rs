use uuid::Uuid;
use std::hash::{Hash, Hasher};
use strum::Display;
    
#[derive(Display, Debug, Copy, Clone)]
pub enum OrderType {
    Lim,
    Fok,
    Ioc,
}
#[derive(Display, Debug, Eq, PartialEq, Copy, Clone)]
pub enum Side {
    Buy,
    Sell,
}
#[derive(Display, Debug)]
pub enum ExternalEvent {
    Queued,
}
#[derive(Display, Debug)]
enum InternalEvent {
    Accepted,
    Canceled,
    Executed,
    PartiallyExecuted,
}
#[derive(Debug)]
pub struct Order {
    order_type: OrderType,
    side: Side,
    price: u64,
    initial_qty: u64,
    current_qty: u64,
    user_id: u64,
    internal_id: Uuid,
}

impl Order {
    pub fn new(_order_type: OrderType, _side : Side, _price : u64,
            _initial_qty : u64, _user_id : u64) -> Order {
            let o = Order{ order_type: _order_type, side : _side,
                price : _price, initial_qty : _initial_qty, current_qty : _initial_qty,
                user_id : _user_id, internal_id : Uuid::new_v4()};
            o.print_due_inernal_event(InternalEvent::Accepted);
            o
    }
}

impl Order {
    pub fn print_due_external_event(&self, event: ExternalEvent) {
        print!("{};", event);
        self.print_order_info();
    }

    pub fn side(&self) -> Side {
        return self.side;
    }

    pub fn price(&self) -> u64 {
        return self.price;
    }

    pub fn current_qty(&self) -> u64 {
        return self.current_qty;
    }

    pub fn order_type(&self) -> OrderType {
        return self.order_type;
    }

    pub fn reduce_quantity(&mut self, qty: u64) -> Option<u64> {
        if self.current_qty < qty {
            None
        }
        else {
            self.current_qty -= qty;
            Some(self.current_qty)
        }
    }

    fn print_due_inernal_event(&self, event: InternalEvent) {
        print!("{};", event);
        self.print_order_info();
    }

    fn print_order_info(&self) {
        println!("{};{};{};{};{};{}",
        self.order_type,
        self.side,
        self.price,
        self.initial_qty,
        self.user_id,
        self.internal_id);
    }
}

impl Drop for Order {
    fn drop(&mut self) {
        let const_self = &(*self);
        match const_self.order_type {
            OrderType::Lim => const_self.print_due_inernal_event(InternalEvent::Executed),
            OrderType::Ioc => if const_self.current_qty == 0 {
                const_self.print_due_inernal_event(InternalEvent::Executed)
            }
            else if const_self.current_qty < const_self.initial_qty {
                const_self.print_due_inernal_event(InternalEvent::PartiallyExecuted)
            }
            else {
                const_self.print_due_inernal_event(InternalEvent::Canceled)
            },
            OrderType::Fok => if const_self.current_qty != 0 {
                const_self.print_due_inernal_event(InternalEvent::Canceled)
            }
            else {
                const_self.print_due_inernal_event(InternalEvent::Executed)
            }
        }
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Order) -> bool {
        self.internal_id == other.internal_id
    }
}

impl Hash for Order {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.internal_id.hash(state);
    }
}

impl Eq for Order {}

