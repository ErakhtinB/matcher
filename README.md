# Matcher

A trading order matching engine implemented in Rust.

## Order Properties

- **Side**: Buy or Sell
- **Price Limit**: For buy orders - maximum price willing to pay; for sell orders - minimum price willing to accept
- **Quantity**: Number of units to trade
- **User ID**: Unique identifier for the order owner
- **Order Type**: Limit, Fill-or-Kill, or Immediate-or-Cancel

## Order Types

### Limit (Lim)
- Can be executed in full or in part
- Any unfilled portion is queued for later execution
- If no immediate execution is possible, the entire order is queued

### Fill or Kill (Fok)
- Must be executed in full or not at all
- If there are sufficient matching orders in the book at acceptable prices, the order is executed
- Otherwise, it is canceled (not added to the queue)
- Cannot be partially executed

### Immediate or Cancel (Ioc)
- Any part that can be immediately executed will be
- The rest is canceled (not queued)

## Order Processing

- Orders are processed according to price-time priority (FIFO)
- For buy orders, lower prices have higher priority
- For sell orders, higher prices have higher priority
- Orders from the same user are not matched against each other
- Only Limit orders can become passive (queued) orders

## Usage

The system accepts orders from a CSV file and outputs the results to stdout:
```
cargo run -- <input_csv_file>
```

An example CSV file is included in the repository:
```
cargo run -- example.csv
```

### CSV Format
The input CSV file should have the following columns:
```
order_type,side,price,initial_qty,user_id
Lim,Buy,100,10,1
Lim,Sell,105,5,2
Fok,Buy,103,7,3
```

### Output
The program outputs the status of each order as it's processed:
- **Accepted**: Order has been received by the system
- **Queued**: Order has been placed in the order book
- **Canceled**: Order has been removed without execution
- **Executed**: Order has been fully executed
- **PartiallyExecuted**: Order has been partially executed
