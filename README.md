# matcher
The order has the following properties:
    Side (side) - buying or selling
    Price limit (price: u64) – buy no more expensive / sell no cheaper
    Quantity of goods purchased (size: u64)
    User ID (u64)
    Type (see below)
Applications are of three types:
    Limit - executed in full or in part, and the remainder is queued for execution. If it is not possible to execute at the time of receipt, then it is queued in its entirety.
    Fill or kill - execute all or nothing. If there is a total number of applications in the queue (glass) with a price that satisfies the requested one, then such an application is satisfied. Otherwise, it is canceled (not added to the queue). Such an application cannot be partially executed.
Immediate or cancel - any part of such an order is executed, and the rest is canceled (not queued). Thus, incoming and passive applications are distinguished. Incoming - currently being serviced. Passive - previously serviced and queued for execution. Only Limit are passive. An incoming application can be reduced to several passive ones. Orders from one user are not reduced to each other.
The system must be able to accept applications as input (file) and output (stdout) the results of the match - queuing, execution, cancellation.

Orders are processed according to priority (price, FIFO). First the most attractive in terms of price (buy low, sell high), and then the order. Partial fulfillment - part of one application completely satisfies another application.
