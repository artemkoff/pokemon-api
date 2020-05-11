use serde::{Deserialize, Serialize};

use super::common::{Id, Integer, Name};
use super::resource::NamedResource;

/// Berries are small fruits that can provide HP and status condition restoration,
/// stat enhancement, and even damage negation when eaten by Pokémon.
/// Check out [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Berry) for greater detail.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Berry {
    /// The identifier for this resource.
    pub id: Id,

    /// The name for this resource.
    pub name: String,

    /// Time it takes the tree to grow one stage, in hours.
    /// Berry trees go through four of these growth stages before they can be picked.
    pub growth_time: Integer,

    /// The maximum number of these berries that can grow on one tree in Generation IV.
    pub max_harvest: Integer,

    /// The power of the move "Natural Gift" when used with this Berry.
    pub natural_gift_power: Integer,

    /// The size of this Berry, in millimeters.
    pub size: Integer,

    /// The smoothness of this Berry, used in making Pokéblocks or Poffins.
    pub smoothness: Integer,

    /// The speed at which this Berry dries out the soil as it grows.
    /// A higher rate means the soil dries more quickly.
    pub soil_dryness: Integer,

    /// The firmness of this berry, used in making Pokéblocks or Poffins.
    pub firmness: NamedResource,

    /// A list of references to each flavor a berry can have
    /// and the potency of each of those flavors in regard to this berry.
    pub flavors: Vec<BerryFlavorMap>,

    /// Berries are actually items. This is a reference to the item specific data for this berry.
    pub item: NamedResource,

    /// The type inherited by "Natural Gift" when used with this Berry.
    pub natural_gift_type: NamedResource,
}

/// Describes a flavor of a berry.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BerryFlavorMap {
    /// How powerful the referenced flavor is for this berry.
    pub potency: Integer,

    /// The referenced berry flavor.
    pub flavor: NamedResource,
}

/// Berries can be soft or hard.
/// Check out [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Category:Berries_by_firmness)
/// for greater detail.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BerryFirmness {
    /// The identifier for this resource.
    pub id: Id,

    /// The name for this resource.
    pub name: String,

    /// A list of the berries with this firmness.
    pub berries: Vec<NamedResource>,

    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
}

/// Flavors determine whether a Pokémon will benefit or suffer from eating a berry
/// based on their [nature](https://pokeapi.co/docs/v2.html/#natures).
/// Check out [Bulbapedia](http://bulbapedia.bulbagarden.net/wiki/Flavor) for greater detail.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BerryFlavor {
    /// The identifier for this resource.
    pub id: Id,

    /// The name for this resource.
    pub name: String,

    /// A list of the berries with this flavor.
    pub berries: Vec<FlavorBerryMap>,

    /// The contest type that correlates with this berry flavor.
    pub contest_type: NamedResource,

    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlavorBerryMap {
    /// How powerful the referenced flavor is for this berry.
    pub potency: Integer,

    /// The berry with the referenced flavor.
    pub berry: NamedResource,
}
