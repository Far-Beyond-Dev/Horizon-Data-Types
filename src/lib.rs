//! # Game Server Architecture
//!
//! This module provides data types and structures for a distributed game server architecture
//! with sophisticated event propagation in 3D space.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};

/// Represents a 3D vector in the game world.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3 {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Z coordinate
    pub z: f32,
}

impl Vector3 {
    /// Creates a new Vector3 instance.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate
    /// * `y` - The y coordinate
    /// * `z` - The z coordinate
    ///
    /// # Returns
    ///
    /// A new Vector3 instance
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::Vector3;
    ///
    /// let position = Vector3::new(1.0, 2.0, 3.0);
    /// assert_eq!(position.x, 1.0);
    /// assert_eq!(position.y, 2.0);
    /// assert_eq!(position.z, 3.0);
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Represents a game object in the world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameObject {
    /// Unique identifier for the game object
    pub id: Uuid,
    /// Position of the game object in 3D space
    pub position: Vector3,
    /// Type of the game object
    pub object_type: String,
    /// Additional properties of the game object
    pub properties: serde_json::Value,
}

impl GameObject {
    /// Creates a new GameObject instance.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the object in 3D space
    /// * `object_type` - The type of the object
    /// * `properties` - Additional properties of the object
    ///
    /// # Returns
    ///
    /// A new GameObject instance with a randomly generated UUID
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::{GameObject, Vector3};
    /// use serde_json::json;
    ///
    /// let position = Vector3::new(10.0, 20.0, 30.0);
    /// let object = GameObject::new(
    ///     position,
    ///     "Tree".to_string(),
    ///     json!({"height": 5, "fruit": "apple"})
    /// );
    ///
    /// assert_eq!(object.object_type, "Tree");
    /// assert_eq!(object.position.x, 10.0);
    /// assert_eq!(object.properties["height"], 5);
    /// ```
    pub fn new(position: Vector3, object_type: String, properties: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            object_type,
            properties,
        }
    }
}

/// Represents a player in the game world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// Unique identifier for the player
    pub id: Uuid,
    /// Username of the player
    pub username: String,
    /// Current position of the player
    pub position: Vector3,
    /// Current health of the player
    pub health: f32,
    /// Inventory of the player
    pub inventory: HashMap<String, u32>,
    /// Additional player stats
    pub stats: serde_json::Value,
}

impl Player {
    /// Creates a new Player instance.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the player
    /// * `position` - The initial position of the player
    /// * `health` - The initial health of the player
    /// * `stats` - Additional player stats
    ///
    /// # Returns
    ///
    /// A new Player instance with a randomly generated UUID and empty inventory
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::{Player, Vector3};
    /// use serde_json::json;
    ///
    /// let player = Player::new(
    ///     "Alice".to_string(),
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     100.0,
    ///     json!({"strength": 10, "agility": 15})
    /// );
    ///
    /// assert_eq!(player.username, "Alice");
    /// assert_eq!(player.health, 100.0);
    /// assert_eq!(player.stats["strength"], 10);
    /// assert!(player.inventory.is_empty());
    /// ```
    pub fn new(username: String, position: Vector3, health: f32, stats: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            position,
            health,
            inventory: HashMap::new(),
            stats,
        }
    }

    /// Adds an item to the player's inventory.
    ///
    /// # Arguments
    ///
    /// * `item` - The name of the item
    /// * `quantity` - The quantity of the item to add
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::{Player, Vector3};
    /// use serde_json::json;
    ///
    /// let mut player = Player::new(
    ///     "Bob".to_string(),
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     100.0,
    ///     json!({})
    /// );
    ///
    /// player.add_to_inventory("Sword".to_string(), 1);
    /// assert_eq!(player.inventory.get("Sword"), Some(&1));
    ///
    /// player.add_to_inventory("Sword".to_string(), 1);
    /// assert_eq!(player.inventory.get("Sword"), Some(&2));
    /// ```
    pub fn add_to_inventory(&mut self, item: String, quantity: u32) {
        *self.inventory.entry(item).or_insert(0) += quantity;
    }
}

/// Represents an event in the game world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    /// Unique identifier for the event
    pub id: Uuid,
    /// Type of the event
    pub event_type: String,
    /// Position where the event occurred
    pub position: Vector3,
    /// Radius of effect for the event
    pub radius: f32,
    /// Additional data associated with the event
    pub data: serde_json::Value,
}

impl GameEvent {
    /// Creates a new GameEvent instance.
    ///
    /// # Arguments
    ///
    /// * `event_type` - The type of the event
    /// * `position` - The position where the event occurred
    /// * `radius` - The radius of effect for the event
    /// * `data` - Additional data associated with the event
    ///
    /// # Returns
    ///
    /// A new GameEvent instance with a randomly generated UUID
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::{GameEvent, Vector3};
    /// use serde_json::json;
    ///
    /// let event = GameEvent::new(
    ///     "Explosion".to_string(),
    ///     Vector3::new(50.0, 50.0, 0.0),
    ///     10.0,
    ///     json!({"damage": 50, "effects": ["fire", "smoke"]})
    /// );
    ///
    /// assert_eq!(event.event_type, "Explosion");
    /// assert_eq!(event.radius, 10.0);
    /// assert_eq!(event.data["damage"], 50);
    /// ```
    pub fn new(event_type: String, position: Vector3, radius: f32, data: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            position,
            radius,
            data,
        }
    }
}

/// Represents a spatial partition in the game world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialPartition {
    /// Unique identifier for the partition
    pub id: Uuid,
    /// Minimum corner of the partition's bounding box
    pub min: Vector3,
    /// Maximum corner of the partition's bounding box
    pub max: Vector3,
}

impl SpatialPartition {
    /// Creates a new SpatialPartition instance.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum corner of the partition's bounding box
    /// * `max` - The maximum corner of the partition's bounding box
    ///
    /// # Returns
    ///
    /// A new SpatialPartition instance with a randomly generated UUID
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::{SpatialPartition, Vector3};
    ///
    /// let partition = SpatialPartition::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(100.0, 100.0, 100.0)
    /// );
    ///
    /// assert_eq!(partition.min.x, 0.0);
    /// assert_eq!(partition.max.x, 100.0);
    /// ```
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Self {
            id: Uuid::new_v4(),
            min,
            max,
        }
    }

    /// Checks if a point is within this spatial partition.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to check
    ///
    /// # Returns
    ///
    /// `true` if the point is within the partition, `false` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::{SpatialPartition, Vector3};
    ///
    /// let partition = SpatialPartition::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(100.0, 100.0, 100.0)
    /// );
    ///
    /// assert!(partition.contains(&Vector3::new(50.0, 50.0, 50.0)));
    /// assert!(!partition.contains(&Vector3::new(150.0, 150.0, 150.0)));
    /// ```
    pub fn contains(&self, point: &Vector3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }

    /// Checks if this spatial partition intersects with another.
    ///
    /// # Arguments
    ///
    /// * `other` - The other spatial partition to check against
    ///
    /// # Returns
    ///
    /// `true` if the partitions intersect, `false` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::{SpatialPartition, Vector3};
    ///
    /// let partition1 = SpatialPartition::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(100.0, 100.0, 100.0)
    /// );
    ///
    /// let partition2 = SpatialPartition::new(
    ///     Vector3::new(50.0, 50.0, 50.0),
    ///     Vector3::new(150.0, 150.0, 150.0)
    /// );
    ///
    /// let partition3 = SpatialPartition::new(
    ///     Vector3::new(200.0, 200.0, 200.0),
    ///     Vector3::new(300.0, 300.0, 300.0)
    /// );
    ///
    /// assert!(partition1.intersects(&partition2));
    /// assert!(!partition1.intersects(&partition3));
    /// ```
    pub fn intersects(&self, other: &SpatialPartition) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }
}

/// Represents a game server in the distributed architecture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameServer {
    /// Unique identifier for the game server
    pub id: Uuid,
    /// Spatial partition representing the server's area of responsibility
    pub partition: SpatialPartition,
    /// Set of player IDs currently managed by this server
    pub players: HashSet<Uuid>,
    /// Set of game object IDs currently managed by this server
    pub objects: HashSet<Uuid>,
}

impl GameServer {
    /// Creates a new GameServer instance.
    ///
    /// # Arguments
    ///
    /// * `partition` - The spatial partition representing the server's area
    ///
    /// # Returns
    ///
    /// A new GameServer instance with a randomly generated UUID and empty sets of players and objects
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::{GameServer, SpatialPartition, Vector3};
    ///
    /// let partition = SpatialPartition::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(100.0, 100.0, 100.0)
    /// );
    /// let server = GameServer::new(partition);
    ///
    /// assert!(server.players.is_empty());
    /// assert!(server.objects.is_empty());
    /// ```
    pub fn new(partition: SpatialPartition) -> Self {
        Self {
            id: Uuid::new_v4(),
            partition,
            players: HashSet::new(),
            objects: HashSet::new(),
        }
    }

    /// let partition = SpatialPartition::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(100.0, 100.0, 100.0)
    /// );
    /// let mut server = GameServer::new(partition);
    ///
    /// let event = GameEvent::new(
    ///     "Explosion".to_string(),
    ///     Vector3::new(50.0, 50.0, 50.0),
    ///     10.0,
    ///     json!({"damage": 50})
    /// );
    ///
    /// let overflows = server.process_event(&event);
    /// assert!(!overflows);
    /// ```
    pub fn process_event(&mut self, event: &GameEvent) -> bool {
        // Process the event for all relevant entities
        // This is a simplified implementation; in a real system, you'd update
        // players and objects affected by the event

        // Check if the event overflows the server's boundaries
        !self.partition.contains(&event.position) || 
        event.radius > (self.partition.max.x - self.partition.min.x).min(
            (self.partition.max.y - self.partition.min.y).min(
                self.partition.max.z - self.partition.min.z
            )
        ) / 2.0
    }
}

/// Represents a cluster of game servers managed by a master server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCluster {
/// Unique identifier for the cluster
pub id: Uuid,
/// Spatial partition representing the cluster's area of responsibility
pub partition: SpatialPartition,
/// Map of game server IDs to GameServer instances in this cluster
pub servers: HashMap<Uuid, GameServer>,
}

impl ServerCluster {
    /// Creates a new ServerCluster instance.
    ///
    /// # Arguments
    ///
    /// * `partition` - The spatial partition representing the cluster's area
    ///
    /// # Returns
    ///
    /// A new ServerCluster instance with a randomly generated UUID and empty map of servers
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::{ServerCluster, SpatialPartition, Vector3};
    ///
    /// let partition = SpatialPartition::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(1000.0, 1000.0, 1000.0)
    /// );
    /// let cluster = ServerCluster::new(partition);
    ///
    /// assert!(cluster.servers.is_empty());
    /// ```
    pub fn new(partition: SpatialPartition) -> Self {
        Self {
            id: Uuid::new_v4(),
            partition,
            servers: HashMap::new(),
        }
    }

    /// Adds a game server to the cluster.
    ///
    /// # Arguments
    ///
    /// * `server` - The GameServer to add to the cluster
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::{ServerCluster, GameServer, SpatialPartition, Vector3};
    ///
    /// let cluster_partition = SpatialPartition::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(1000.0, 1000.0, 1000.0)
    /// );
    /// let mut cluster = ServerCluster::new(cluster_partition);
    ///
    /// let server_partition = SpatialPartition::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(100.0, 100.0, 100.0)
    /// );
    /// let server = GameServer::new(server_partition);
    ///
    /// cluster.add_server(server);
    /// assert_eq!(cluster.servers.len(), 1);
    /// ```
    pub fn add_server(&mut self, server: GameServer) {
        self.servers.insert(server.id, server);
    }

    /// Propagates an event to relevant servers within the cluster.
    ///
    /// # Arguments
    ///
    /// * `event` - The GameEvent to propagate
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the event overflows the cluster's boundaries
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::{ServerCluster, GameServer, SpatialPartition, Vector3, GameEvent};
    /// use serde_json::json;
    ///
    /// let cluster_partition = SpatialPartition::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(1000.0, 1000.0, 1000.0)
    /// );
    /// let mut cluster = ServerCluster::new(cluster_partition);
    ///
    /// let server_partition = SpatialPartition::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(100.0, 100.0, 100.0)
    /// );
    /// let server = GameServer::new(server_partition);
    /// cluster.add_server(server);
    ///
    /// let event = GameEvent::new(
    ///     "Explosion".to_string(),
    ///     Vector3::new(50.0, 50.0, 50.0),
    ///     10.0,
    ///     json!({"damage": 50})
    /// );
    ///
    /// let overflows = cluster.propagate_event(&event);
    /// assert!(!overflows);
    /// ```
    pub fn propagate_event(&mut self, event: &GameEvent) -> bool {
        let mut cluster_overflow = false;

        for server in self.servers.values_mut() {
            if server.partition.contains(&event.position) || 
               server.partition.intersects(&SpatialPartition::new(
                   Vector3::new(event.position.x - event.radius, event.position.y - event.radius, event.position.z - event.radius),
                   Vector3::new(event.position.x + event.radius, event.position.y + event.radius, event.position.z + event.radius)
               )) {
                let server_overflow = server.process_event(event);
                cluster_overflow |= server_overflow;
            }
        }

        cluster_overflow || !self.partition.contains(&event.position)
    }
}

/// Represents the top-level master server managing multiple server clusters.
#[derive(Debug, Serialize, Deserialize)]
pub struct MasterServer {
/// Unique identifier for the master server
pub id: Uuid,
/// Map of cluster IDs to ServerCluster instances managed by this master server
pub clusters: HashMap<Uuid, ServerCluster>,
}

impl MasterServer {
    /// Creates a new MasterServer instance.
    ///
    /// # Returns
    ///
    /// A new MasterServer instance with a randomly generated UUID and empty map of clusters
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::MasterServer;
    ///
    /// let master = MasterServer::new();
    /// assert!(master.clusters.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            clusters: HashMap::new(),
        }
    }

    /// Adds a server cluster to the master server.
    ///
    /// # Arguments
    ///
    /// * `cluster` - The ServerCluster to add to the master server
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::{MasterServer, ServerCluster, SpatialPartition, Vector3};
    ///
    /// let mut master = MasterServer::new();
    ///
    /// let cluster_partition = SpatialPartition::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(1000.0, 1000.0, 1000.0)
    /// );
    /// let cluster = ServerCluster::new(cluster_partition);
    ///
    /// master.add_cluster(cluster);
    /// assert_eq!(master.clusters.len(), 1);
    /// ```
    pub fn add_cluster(&mut self, cluster: ServerCluster) {
        self.clusters.insert(cluster.id, cluster);
    }

    /// Propagates an event globally across all relevant clusters and servers.
    ///
    /// # Arguments
    ///
    /// * `event` - The GameEvent to propagate
    ///
    /// # Example
    ///
    /// ```
    /// use game_server_architecture::{MasterServer, ServerCluster, GameServer, SpatialPartition, Vector3, GameEvent};
    /// use serde_json::json;
    ///
    /// let mut master = MasterServer::new();
    ///
    /// let cluster_partition = SpatialPartition::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(1000.0, 1000.0, 1000.0)
    /// );
    /// let mut cluster = ServerCluster::new(cluster_partition);
    ///
    /// let server_partition = SpatialPartition::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(100.0, 100.0, 100.0)
    /// );
    /// let server = GameServer::new(server_partition);
    /// cluster.add_server(server);
    ///
    /// master.add_cluster(cluster);
    ///
    /// let event = GameEvent::new(
    ///     "GlobalEvent".to_string(),
    ///     Vector3::new(500.0, 500.0, 500.0),
    ///     1000.0,
    ///     json!({"impact": "high"})
    /// );
    ///
    /// master.propagate_event(&event);
    /// ```
    pub fn propagate_event(&mut self, event: &GameEvent) {
        for cluster in self.clusters.values_mut() {
            cluster.propagate_event(event);
        }
    }
}