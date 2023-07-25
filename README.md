Generic Robot Framework
===

[![Rust](https://github.com/Generic-Robot-Framework/Generic-Robot-Framework/actions/workflows/rust.yml/badge.svg)](https://github.com/Generic-Robot-Framework/Generic-Robot-Framework/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Example node:

```rust
use std::sync::{Arc, Mutex};

use client_example::msg::example_message::Position2DMessage;

use generic_robot_framework::main_loop;
use generic_robot_framework::models::tnode::Node;
use generic_robot_framework::models::tpub::Publisher;
use generic_robot_framework::models::tsub::Subscriber;

fn main() {
    // Create some atomic data
    let atomic_data = Arc::new(Mutex::new(0));

    // Creating and registering the Node
    let node = Node::register("my_node", 1000);

    // Creating a Subscriber
    let _subscriber: Subscriber<Position2DMessage, Arc<Mutex<u64>>> = Subscriber::new(
        "my_new_topic",
        test_handle,
        Some(Arc::clone(&atomic_data))
    );

    // Creating a Publisher
    let publisher: Publisher<Position2DMessage> = Publisher::new("my_new_topic");

    // Creating a Position2DMessage
    let message = Position2DMessage {
        x: 1,
        y: 2
    };

    // Publishing the message
    publisher.publish(message);

    // Main application loop, checks if the node is Ok
    main_loop!(node, {
        // Print our atomic data
        println!("{}", *atomic_data.lock().unwrap());

        // Sleeping at Node rate
        node.sleep();
    });
}

// Called when a new message is received
fn test_handle(message: Position2DMessage, atomic_data: Option<Arc<Mutex<u64>>>) {
    // Print the received message
    println!("{}", message);

    // Getting the atomic data mutex
    let arc = atomic_data.unwrap();
    let mut data = arc.lock().unwrap();

    // Changing atomic data
    *data = message.y
}
```

Example message:

```rust
use generic_robot_framework::register_message;

register_message!(Position2DMessage {
    x: u64,
    y: u64,
});
```

## Workspace architecture:

```yaml
project_workspace: # GRF package typed "Workspace"
  src:
    packages:
      example_adapter:   # GRF package typed "Adapter"
        ...
      example_resource:  # GRF package typed "Resource"
        ...
      example_package:   # GRF package typed "Module"
        src:
          msg:           # Folder containing messages structs
            - example_message.rs
          bin:           # Folder containing nodes scripts
            - example_node.rs
        - Cargo.toml
        - Cargo.lock
  - Cargo.toml
  - Cargo.lock
```
