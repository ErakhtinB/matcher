use criterion::{black_box, criterion_group, criterion_main, Criterion};
use matcher::{order, Matcher};

fn benchmark_limit_order_matching(c: &mut Criterion) {
    c.bench_function("simple_limit_order_match", |b| {
        b.iter(|| {
            // Create a matcher
            let mut matcher = Matcher::new();

            // Create a buy limit order
            let buy_order = order::Order::new(
                order::OrderType::Lim,
                order::Side::Buy,
                black_box(100), // price
                black_box(10),  // quantity
                black_box(1),   // user_id
            );

            // Create a matching sell limit order
            let sell_order = order::Order::new(
                order::OrderType::Lim,
                order::Side::Sell,
                black_box(95), // price (lower than buy price, so should match)
                black_box(5),  // quantity (half of buy quantity)
                black_box(2),  // user_id
            );

            // Process the orders
            matcher.proceed_record(buy_order);
            matcher.proceed_record(sell_order);
        })
    });
}

fn benchmark_fok_order_matching(c: &mut Criterion) {
    c.bench_function("fill_or_kill_order_match", |b| {
        b.iter(|| {
            // Create a matcher
            let mut matcher = Matcher::new();

            // Add a sell limit order to the book
            let sell_order = order::Order::new(
                order::OrderType::Lim,
                order::Side::Sell,
                black_box(100), // price
                black_box(10),  // quantity
                black_box(1),   // user_id
            );

            // Add a matching FOK buy order
            let fok_buy_order = order::Order::new(
                order::OrderType::Fok,
                order::Side::Buy,
                black_box(105), // price (higher than sell price, so should match)
                black_box(5),   // quantity (less than sell quantity)
                black_box(2),   // user_id
            );

            // Process the orders
            matcher.proceed_record(sell_order);
            matcher.proceed_record(fok_buy_order);
        })
    });
}

fn benchmark_ioc_order_matching(c: &mut Criterion) {
    c.bench_function("immediate_or_cancel_order_match", |b| {
        b.iter(|| {
            // Create a matcher
            let mut matcher = Matcher::new();

            // Add a buy limit order to the book
            let buy_order = order::Order::new(
                order::OrderType::Lim,
                order::Side::Buy,
                black_box(100), // price
                black_box(10),  // quantity
                black_box(1),   // user_id
            );

            // Add a matching IOC sell order
            let ioc_sell_order = order::Order::new(
                order::OrderType::Ioc,
                order::Side::Sell,
                black_box(95), // price (lower than buy price, so should match)
                black_box(5),  // quantity (half of buy quantity)
                black_box(2),  // user_id
            );

            // Process the orders
            matcher.proceed_record(buy_order);
            matcher.proceed_record(ioc_sell_order);
        })
    });
}

fn benchmark_multiple_orders(c: &mut Criterion) {
    c.bench_function("multiple_orders_processing", |b| {
        b.iter(|| {
            // Create a matcher
            let mut matcher = Matcher::new();

            // Create multiple buy and sell orders with different prices
            let orders = [
                // Buy orders
                order::Order::new(order::OrderType::Lim, order::Side::Buy, 100, 10, 1),
                order::Order::new(order::OrderType::Lim, order::Side::Buy, 101, 5, 2),
                order::Order::new(order::OrderType::Lim, order::Side::Buy, 102, 7, 3),
                // Sell orders
                order::Order::new(order::OrderType::Lim, order::Side::Sell, 99, 3, 4),
                order::Order::new(order::OrderType::Lim, order::Side::Sell, 98, 6, 5),
                order::Order::new(order::OrderType::Lim, order::Side::Sell, 97, 8, 6),
                // FOK and IOC orders
                order::Order::new(order::OrderType::Fok, order::Side::Buy, 103, 4, 7),
                order::Order::new(order::OrderType::Ioc, order::Side::Sell, 96, 9, 8),
            ];

            // Process all orders
            for order in orders {
                matcher.proceed_record(order);
            }
        })
    });
}

fn benchmark_large_order_book(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_order_book");

    // Adjust sample size for these longer-running benchmarks
    group.sample_size(10);

    group.bench_function("process_100_orders", |b| {
        b.iter(|| {
            // Create a matcher
            let mut matcher = Matcher::new();

            // Create 50 buy orders and 50 sell orders
            for i in 1..=50 {
                let buy_price = 100 + (i % 10);
                let sell_price = 110 - (i % 10);

                let buy_order = order::Order::new(
                    order::OrderType::Lim,
                    order::Side::Buy,
                    black_box(buy_price),
                    black_box(5 + (i % 5)),
                    black_box(i),
                );

                let sell_order = order::Order::new(
                    order::OrderType::Lim,
                    order::Side::Sell,
                    black_box(sell_price),
                    black_box(5 + (i % 5)),
                    black_box(i + 100),
                );

                matcher.proceed_record(buy_order);
                matcher.proceed_record(sell_order);
            }
        })
    });

    group.finish();
}

// Configure Criterion to run with minimal output
fn criterion_config() -> Criterion {
    Criterion::default()
        .with_output_color(true)
        .measurement_time(std::time::Duration::from_secs(5))
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = benchmark_limit_order_matching,
              benchmark_fok_order_matching,
              benchmark_ioc_order_matching,
              benchmark_multiple_orders,
              benchmark_large_order_book
}
criterion_main!(benches);
