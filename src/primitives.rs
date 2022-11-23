pub trait PhysBoundingPrimitive {
    fn primitive(&self) -> PhysBoundingPrimitives;
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum PhysBoundingPrimitives {
    Flat(PhysBoundingPrimitives2D),
    Depthed(PhysBoundingPrimitives3D),
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum PhysBoundingPrimitives2D {
    // Polygon
    Triangle,
    Quadrilateral,
    Pentagon,
    Hexagon,
    Heptagon,

    // Curves
    Annulus,
    Arbelos,
    Circle,
    CircularSector,
    CircularSegment,
    Crescent,
    Lens,
    Lune,
    Quatrefoil,
    Realeaux,
    Salinon,

    // Misc
    Spiral,
    Astroid,
    Oval,
    Cardioid,
    Deltoid,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum PhysBoundingPrimitives3D {
    // 3D
    AABBox,
    OBBox,
    SSVol,
    HIVol,
    SphericalShells,
    Zonotopes,
    Spheres,
    Cones,
    Cylinders,
    Ellipsoids,
}
