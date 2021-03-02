use amethyst::{
    renderer::SpriteSheet
};

use tiled;


enum PointType {
    Path,
    Collision
}

struct PathPoint {
    position:   (u32, u32),
    point_type: PointType
}

impl PathPoint {
    fn new (point: (u32, u32)) -> PathPoint {
        PathPoint {
            position:   point,
            point_type: PointType::Collision
        }
    }

    fn set_point(&mut self, point_type: PointType) {
        self.point_type = point_type;
    }
}

struct Path {
    map:        Vec<PathPoint>,
    columns:    u32,
    rows:       u32
}

impl Path {
    fn new (col_rows: (u32, u32)) -> Path {
        let vec = {
            let mut temp_vec = Vec::with_capacity(
                (col_rows.0 * col_rows.1) as usize
            );
            for i in 0..(col_rows.1 * col_rows.0) {
                let y = (i as f32 / col_rows.0 as f32).floor() as u32;
                let x = ((i-(y*col_rows.0)) as f32 / col_rows.1 as f32).floor() as u32;
                println!("Current Path: {:?}", (x, y));
                temp_vec.push(
                    PathPoint::new((x, y))
                );
            }

            temp_vec
        };

        Path {
            map:        vec,
            columns:    col_rows.0,
            rows:       col_rows.1
        }
    }
}

struct TileMap {
    sprite_sheets:  Vec<SpriteSheet>
}

pub struct Map {
    path:       Path,
    tilemap:    TileMap
}

impl Map {
    pub fn new (map: tiled::Map) -> Map {
        Map {
            path:       Path::new((map.width, map.height)),
            tilemap:    TileMap{sprite_sheets: Vec::new()}
        }
    }
}
