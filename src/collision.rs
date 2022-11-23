use crate::primitives::*;
use crate::PhysDimension;

/// A capability of a physics engine to provide a query with specific: dimensions, strategies, and
/// primitives
pub struct CollisionCapability {
    /// The dimensionality of the query
    ///
    /// ie: 2D vs 3D
    // TODO Make generic
    dimension: PhysDimension,
    /// The collision calculation strategy of the query
    // TODO make generic
    strategy: CollisionSolvingStrategy,
    /// The query to be provided by the physics engine
    query: CollisionQuery,
    /// The primitive doing the colliding
    // TODO make generic
    collider: PhysBoundingPrimitives,
    /// The primitive being collided with
    // TODO make generic
    collided: PhysBoundingPrimitives,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum CollisionQuery {
    /// Support for boolean collision detection
    Interference,

    /// The vertices of an overlap of two colliders
    ContactManifold,

    /// The point on the object which is closest to the other object
    NearestPoint,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum CollisionSolvingStrategy {
    Sequential,
    Simultaneos,
    Discrete,
    Continuous,
}
