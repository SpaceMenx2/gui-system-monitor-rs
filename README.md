# System Monitor made in Rust
This is a small project that i wanted to create while I am learning Rust, learning about **eframe** framework using `egui`.

Actually \(Tuesday 22th of August 2026\) it has two plots (graphics) that shows the RAM and CPU consumption in percentages of the last sixty (60) seconds, and also has a "Table of Processes" which shows the first fifty (50) processes sorted by RAM \(descending\).

It shows about Rust the following things:

1. Threads \(`std::thread::spawn()`\)
2. MPSC channels \(Intercommunication throught threads\)
3. Implementation of data structures \(VecDeque instead of Vec which is better for Circular buffers\)

It's still in development, so maybe it'll be finished ASAP!
