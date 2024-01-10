//! Map generation

#[allow(dead_code)]
pub struct Map;

#[allow(dead_code)]
pub struct Seed;

pub trait MapGenerator {
    fn generate(params: Seed) -> Map;
}

// TODO (pangt): default map generator, flesh out `Seed`, and etc.
//
// I think initially just randomly spawning a group of points and then giving
// each of them a sphere sounds like a good idea
