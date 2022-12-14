#![doc = include_str!("../../../README.md")]

use bevy::reflect::DynamicStruct;
use primitives::*;

mod collision;
mod primitives;

/// An instance of a physics engine
pub trait StaticPhysInstance {
    fn build(raw: &'static PhysCapabilities) -> DynamicStruct;
}

/// The raw representation of a physics engines capabilities
pub struct PhysCapabilities {
    /// Collision capabilities of the physics engine
    coll_capabilities: &'static [collision::CollisionCapability],
}

impl PhysCapabilities {
    pub fn new(coll: &'static [collision::CollisionCapability]) -> Self {
        Self {
            coll_capabilities: coll,
        }
    }

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

    struct EmptyPhysEngine {}
}
