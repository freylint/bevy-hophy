
pub struct CollisionCapability {
    dimension: PhysDimension,
    motion: CollisionSolvingStrategy,
    collider: PhysBoundingPrimitives,
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
