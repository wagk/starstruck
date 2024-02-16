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

pub type Grid2D<T> = Vec<Vec<T>>;

struct Tile {}

/// Simple tiled grid
#[allow(dead_code)]
struct TileMap {
    map: Grid2D<Tile>,
}

#[allow(dead_code)]
struct TileMapParams {
    max_length: usize,
    max_width: usize,
    fill: TileMapFillType,
    poi_list: Vec<PointOfInterest>,
}

// Scaffolding for the generation algorithm.
enum PointOfInterest {}

// Might be overkill for now here, actually.
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

    fn generate(param: Self::Param) -> Self::World {
        let mut map = Grid2D::new();

        for _ in 0..param.max_length {
            let mut col = Vec::new();

            for _ in 0..param.max_width {
                col.push(Tile {})
            }

            map.push(col);
        }

        TileMap { map }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_parameters_respected() {
        const LENGTH: usize = 10;
        const WIDTH: usize = 5;

        let params = TileMapParams {
            max_length: LENGTH,
            max_width: WIDTH,
            fill: TileMapFillType::Full,
            poi_list: Vec::new(),
        };

        let TileMap { map } = TileMap::generate(params);

        assert_eq!(map.len(), LENGTH);
        for col in map.into_iter() {
            assert_eq!(col.len(), WIDTH);
        }
    }
}
