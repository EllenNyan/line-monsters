use super::{Model, Tile, TileRotation};

pub fn map() -> [[Tile; 16]; 12] {
    let map = [
        [
            Tile::new_rotation(0, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Quarter),
        ],
        [
            Tile::new_rotation(0, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Quarter),
        ],
        [
            Tile::new_rotation(0, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Corner, TileRotation::Half),
            Tile::new_rotation(1, Model::Wall, TileRotation::Half),
            Tile::new_rotation(1, Model::Wall, TileRotation::Half),
            Tile::new_rotation(1, Model::Corner, TileRotation::Quarter),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Quarter),
        ],
        [
            Tile::new_rotation(0, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(2, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(2, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Wall, TileRotation::Quarter),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Quarter),
        ],
        [
            Tile::new_rotation(0, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(2, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(2, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::InnerCorner, TileRotation::Quarter),
            Tile::new_rotation(1, Model::Wall, TileRotation::Half),
            Tile::new_rotation(1, Model::Corner, TileRotation::Quarter),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Quarter),
        ],
        [
            Tile::new_rotation(0, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(2, Model::Corner, TileRotation::Half),
            Tile::new_rotation(2, Model::Wall, TileRotation::Half),
            Tile::new_rotation(2, Model::Wall, TileRotation::Half),
            Tile::new_rotation(2, Model::Corner, TileRotation::Quarter),
            Tile::new_rotation(1, Model::InnerCorner, TileRotation::Quarter),
            Tile::new_rotation(1, Model::Wall, TileRotation::Half),
            Tile::new_rotation(1, Model::Wall, TileRotation::Half),
            Tile::new_rotation(1, Model::Corner, TileRotation::Quarter),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Quarter),
        ],
        [
            Tile::new_rotation(0, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(2, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(3, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(3, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(2, Model::Wall, TileRotation::Quarter),
            Tile::new_rotation(2, Model::Floor, TileRotation::Quarter),
            Tile::new_rotation(2, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(2, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Wall, TileRotation::Quarter),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Quarter),
        ],
        [
            Tile::new_rotation(0, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(2, Model::Corner, TileRotation::ThreeQuarters),
            Tile::new_rotation(2, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(2, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(2, Model::Corner, TileRotation::Zero),
            Tile::new_rotation(1, Model::InnerCorner, TileRotation::Zero),
            Tile::new_rotation(1, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(1, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(1, Model::Corner, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Quarter),
        ],
        [
            Tile::new_rotation(0, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Corner, TileRotation::ThreeQuarters),
            Tile::new_rotation(1, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(1, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(1, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(1, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(1, Model::Corner, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Quarter),
        ],
        [
            Tile::new_rotation(0, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Quarter),
        ],
        [
            Tile::new_rotation(0, Model::Wall, TileRotation::ThreeQuarters),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(1, Model::Floor, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Quarter),
        ],
        [
            Tile::new_rotation(0, Model::Corner, TileRotation::ThreeQuarters),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Wall, TileRotation::Zero),
            Tile::new_rotation(0, Model::Corner, TileRotation::Zero),
        ],
    ];
    map
}