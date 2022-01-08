pub mod order {
    #[derive(strum_macros::ToString, Debug)]
    pub enum OrderType {
        Lim,
        Fok,
        Ioc,
    }
    #[derive(strum_macros::ToString, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    #[derive(strum_macros::ToString, Debug)]
    pub enum ExternalEvent {
        Queued,
    }
    #[derive(strum_macros::ToString, Debug)]
    enum InternalEvent {
        Accepted,
        Canceled,
        Executed,
        PartiallyExecuted,
    }
    pub struct Order {
        order_type: OrderType,
        side: Side,
        price: u64,
        initial_qty: u64,
        current_qty: u64,
        user_id: u64,
    }
    impl Order {
        pub fn new(_order_type: OrderType, _side : Side, _price : u64, _initial_qty : u64, _user_id : u64) -> Order {
            let mut o = Order{ order_type: _order_type, side : _side, price : _price,
                initial_qty : _initial_qty, current_qty : _initial_qty, user_id : _user_id };
            o.print_due_inernal_event(InternalEvent::Accepted);
            o
        }
    }
    impl Order {
        pub fn reduce_quantity(mut self, qty: u64) -> Option<u64> {
            if self.current_qty < qty {
                None
            }
            else {
                self.current_qty -= qty;
                Some(self.current_qty)
            }
        }
    }
    impl Order {
        fn print_order_info(&mut self) {
            println!("{};{};{};{};{}",
            self.order_type.to_string(),
            self.side.to_string(),
            self.price,
            self.initial_qty,
            self.user_id);
        }
    }
    impl Order {
        fn print_due_inernal_event(&mut self, event: InternalEvent) {
            print!("{};", event.to_string());
            self.print_order_info();
        }
    }
    impl Order {
        pub fn print_due_external_event(&mut self, event: ExternalEvent) {
            print!("{};", event.to_string());
            self.print_order_info();
        }
    }
    impl Drop for Order {
        fn drop(&mut self) {
            match self.order_type {
                OrderType::Lim => self.print_due_inernal_event(InternalEvent::Executed),
                OrderType::Ioc => if self.current_qty == 0 {
                    self.print_due_inernal_event(InternalEvent::Executed)
                }
                else if self.current_qty < self.initial_qty {
                    self.print_due_inernal_event(InternalEvent::PartiallyExecuted)
                }
                else {
                    self.print_due_inernal_event(InternalEvent::Canceled)
                },
                OrderType::Fok => if self.current_qty != 0 {
                    self.print_due_inernal_event(InternalEvent::Canceled)
                }
                else {
                    self.print_due_inernal_event(InternalEvent::Executed)
                }
            }
        }
    }
    //implement drop for different types
}
