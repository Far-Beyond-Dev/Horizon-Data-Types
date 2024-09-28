# 🌌 Horizon Data Types

## Table of Contents
- [🎯 Purpose](#-purpose)
- [📦 Contents](#-contents)
- [🚀 Usage](#-usage)
  - [📡 Working with Events](#-working-with-events)
  - [🔄 Using Transforms](#-using-transforms)
  - [👥 Managing Players](#-managing-players)
  - [🌍 Working with World Data](#-working-with-world-data)
  - [🖥️ Using ChildServer](#️-using-childserver)
- [📝 Note for Developers](#-note-for-developers)
- [🤝 Contributing](#-contributing)
- [📜 License](#-license)

Horizon Data Types is a Rust crate that contains the major data structures and types used throughout the Horizon project. This crate serves as a central repository for shared data types, allowing both the main Horizon application and its associated crates to access these types without encountering circular dependency issues.

## 🎯 Purpose

The main purposes of this crate are:

1. To provide a single source of truth for common data structures used in Horizon.
2. To prevent circular dependencies by separating data type definitions from their implementations.
3. To ensure consistency in data representation across different parts of the Horizon project.

## 📦 Contents

This crate defines several important structures and types, including:

### 🌐 Network and Event Handling

- `Event`: Represents an event in the Horizon system, including its origin, data, and propagation distance.
- `ChildServer`: Defines the structure and behavior of child servers in Horizon's distributed architecture.

### 🎭 World Objects and Transforms

- `TrajectoryPoint`: Represents a point in a trajectory, including time, facing, and position.
- `Rotation`, `Translation`, `Location`, `Vec3D`, `Scale3D`: Various structures for representing 3D transformations and positions.
- `Transform`: A composite structure representing a full 3D transformation.
- `Vec2D`: Represents a 2D vector.

### 👤 Player Management

- `Player`: Represents a player in the Horizon system, including their connection info, transform data, and animation state.
- `PlayerManager`: Manages the collection of active players.

### 🌎 World Data Structures

- `Chunk`, `Region`: Structures for representing and organizing world data.
- `Actor`: Represents an actor in the world, including its location and metadata.
- `Planet`: Represents a planet, combining actor data and regions.

## 🚀 Usage

To use this crate in your Horizon-related project, add the following to your `Cargo.toml`:

```toml
[dependencies]
horizon_data_types = { path = "path/to/horizon_data_types" }
```

Then, in your Rust code, you can import and use the types as needed. Here are some examples of how to use various types from the crate:

### 📡 Working with Events

```rust
use horizon_data_types::Event;

fn handle_event(event: Event) {
    println!("Received event from origin: ({}, {}, {})", 
             event.origin.0, event.origin.1, event.origin.2);
    println!("Event data: {}", event.data);
    println!("Propagation distance: {}", event.propagation_distance);
}
```

### 🔄 Using Transforms

```rust
use horizon_data_types::{Transform, Translation, Rotation, Scale3D};

fn create_transform() -> Transform {
    Transform {
        location: Some(Translation { x: 10.0, y: 20.0, z: 30.0 }),
        rotation: Some(Rotation { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }),
        translation: None,
        scale3D: Scale3D { x: 1.0, y: 1.0, z: 1.0 },
    }
}
```

### 👥 Managing Players

```rust
use horizon_data_types::{Player, PlayerManager};
use socketioxide::extract::SocketRef;

fn add_new_player(socket: SocketRef, player_manager: &PlayerManager) {
    let player = Player::new(socket, "player_1".to_string());
    let notify = player_manager.add_player(player.id.clone());
    
    // Use the notify object to handle player-specific events
}
```

### 🌍 Working with World Data

```rust
use horizon_data_types::{Actor, Location, Planet, Region};
use std::collections::HashMap;

fn create_planet() -> Planet {
    Planet {
        actor_data: Actor {
            location: Location { x: 0.0, y: 0.0, z: 0.0 },
            meta_tags: vec![
                {
                    let mut map = HashMap::new();
                    map.insert("name".to_string(), "Earth".to_string());
                    map
                }
            ],
        },
        object_file: vec![
            Region {
                location: (0, 0),
                chunks: 64, // Assuming 8x8 chunk grid
            }
        ],
    }
}
```

### 🖥️ Using ChildServer

```rust
use horizon_data_types::{ChildServer, Coordinate};
use std::net::SocketAddr;

async fn setup_child_server() {
    let id = 1;
    let coordinate = Coordinate { x: 0, y: 0, z: 0 };
    let parent_addr: SocketAddr = "127.0.0.1:8000".parse().unwrap();
    let local_addr: SocketAddr = "127.0.0.1:8001".parse().unwrap();

    let child_server = ChildServer::new(id, coordinate, parent_addr, local_addr).await;
    
    // Use child_server methods to handle events, determine propagation, etc.
}
```

These examples demonstrate how to use some of the main types defined in the Horizon Data Types crate. For more detailed information on each type and its methods, please refer to the API documentation.

## 📝 Note for Developers

When adding new types or modifying existing ones, please ensure that:

1. The changes are backwards-compatible where possible.
2. Any new types are well-documented with comments explaining their purpose and usage.
3. The changes are reflected in the main Horizon documentation.

## 🤝 Contributing

Contributions to Horizon Data Types are welcome. Please ensure that your contributions adhere to the existing code style and include appropriate documentation and tests.

## 📜 License

Apache 2.0
