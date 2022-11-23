#![doc = include_str!("../README.md")]

use bevy::reflect::DynamicStruct;
use primitives::*;

mod primitives;
mod collision;

/// An instance of a physics engine
trait PhysInstance {
    fn build(raw: &'static PhysCapabilities) -> DynamicStruct;
}

/// The raw representation of a physics engines capabilities
pub struct PhysCapabilities {
    coll_capabilities: &'static [collision::CollisionCapability]
}


#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum PhysDimension {
    TwoDimensional,
    ThreeDimensional,
}


#[cfg(test)]
mod tests {
    use super::*;
}
