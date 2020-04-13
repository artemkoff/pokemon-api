use serde::{Deserialize, Serialize};

use super::resource::NamedResource;

/// Berries are small fruits that can provide HP and status condition restoration,
/// stat enhancement, and even damage negation when eaten by Pokémon.
/// Check out Bulbapedia for greater detail.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Berry {
    /// The identifier for this resource.
    pub id: i32,

    /// The name for this resource.
    pub name: String,

    /// Time it takes the tree to grow one stage, in hours.
    /// Berry trees go through four of these growth stages before they can be picked.
    pub growth_time: i32,

    /// The maximum number of these berries that can grow on one tree in Generation IV.
    pub max_harvest: i32,

    /// The power of the move "Natural Gift" when used with this Berry.
    pub natural_gift_power: i32,

    /// The size of this Berry, in millimeters.
    pub size: i32,

    /// The smoothness of this Berry, used in making Pokéblocks or Poffins.
    pub smoothness: i32,

    /// The speed at which this Berry dries out the soil as it grows.
    /// A higher rate means the soil dries more quickly.
    pub soil_dryness: i32,

    /// The firmness of this berry, used in making Pokéblocks or Poffins.
    pub firmness: NamedResource,

    /// Berries are actually items. This is a reference to the item specific data for this berry.
    pub item: NamedResource,

    /// The type inherited by "Natural Gift" when used with this Berry.
    pub natural_gift_type: NamedResource,
}
