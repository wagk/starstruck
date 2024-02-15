//! Map generation
//!
//! Consider implementing something like [Wave Function Collapse][1]. Sounds
//! cool.
//!
//! [1]: https://robertheaton.com/2018/12/17/wavefunction-collapse-algorithm/

// Design thoughts about how the map should be?

pub trait Seed {
    type Param;
    type World;

    fn generate(_param: Self::Param) -> Self::World;
}

/// Simple tiled gridd
struct TileMap {}

#[allow(dead_code)]
struct TileMapParams {
    max_length: usize,
    max_width: usize,
    fill: TileMapFillType,
    poi_list: Vec<PointOfInterest>,
}

// Scaffolding for the generation algorithm.
enum PointOfInterest {}

#[allow(dead_code)]
enum TileMapFillType {
    // Fill out only areas nearby points of interests, and let the rest generate
    // later
    Dynamic,
    // Fill out every area irregardless.
    Full,
}

impl Seed for TileMap {
    type Param = TileMapParams;
    type World = TileMap;

    fn generate(_param: Self::Param) -> Self::World {
        todo!()
    }
}
