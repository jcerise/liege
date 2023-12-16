use noise::{NoiseFn, Perlin};
use rand::Rng;

pub enum GrassTileType {
    Grass1,
    Grass2,
    Grass3,
    Grass4,
    Grass5,
    Grass6,
    Grass7
}

impl GrassTileType {
    fn as_i32(&self) -> i32 {
        match self {
            GrassTileType::Grass1 => 1,
            GrassTileType::Grass2 => 2,
            GrassTileType::Grass3 => 3,
            GrassTileType::Grass4 => 4,
            GrassTileType::Grass5 => 5,
            GrassTileType::Grass6 => 6,
            GrassTileType::Grass7 => 7,
        }
    }
}

pub struct GameMap {
    pub tiles: Vec<i32>,
    pub noise_map: Vec<i32>,
    pub map_width: i32,
    pub map_height: i32
}

impl GameMap {
    pub fn new(map_width: i32, map_height: i32) -> Self {
        Self {
            tiles: Vec::new(),
            noise_map: Vec::new(),
            map_width,
            map_height
        }
    }

    pub fn generate_simple_map(&mut self) {
        // Generate a map of map_width by map_height dimensions, and fill each tile with a random grass index
        let mut rng = rand::thread_rng();
        let mut tiles = Vec::<i32>::new();

        for _ in 0..(self.map_width * self.map_height) {
            tiles.push(rng.gen_range(0..7));
        }

        self.tiles = tiles;
    }

    pub fn generate_noise_map(&mut self) {
        let mut map = Vec::with_capacity(self.map_width as usize * self.map_height as usize);

        let perlin = Perlin::new(1001010101);

        for y in 0..self.map_height {
            for x in 0..self.map_width {
                let scale = 10.;
                // Scale the coordinates to get different noise patterns
                let nx = x as f64 / scale;
                let ny = y as f64 / scale;

                // Generate the noise value at this point and push it to the map
                let value = perlin.get([nx, ny]);
                map.push(((value + 1.0) * (6.0 / 2.0)).floor() as i32);
            }
        }

        self.noise_map = map.clone();
    }

    pub fn map_index(&self, x: i32, y: i32) -> i32 {
        ((y * self.map_width) + x)
    }

    pub fn map_coords(&self, index: i32) -> (i32, i32) {
        (index % self.map_width, index / self.map_width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_map() {
        let game_map = GameMap::new(3, 3);

        assert_eq!(game_map.map_width, 3);
        assert_eq!(game_map.map_height, 3);
        assert_eq!(game_map.tiles.len(), 0);
    }

    #[test]
    fn test_generate_simple_map() {
        let mut game_map = GameMap::new(3, 3);
        game_map.generate_simple_map();

        assert_eq!(game_map.tiles.len(), 9);
        for tile in game_map.tiles {
            assert!(tile >= 0 && tile < 7);
        }
    }

    #[test]
    fn test_generate_noise_map() {
        let mut game_map = GameMap::new(3, 3);
        game_map.generate_noise_map();

        assert_eq!(game_map.noise_map.len(), 9);
        for noise_value in game_map.noise_map {
            assert!(noise_value >= 0 && noise_value <= 3);
        }
    }

    #[test]
    fn test_map_index() {
        let game_map = GameMap::new(3, 3);
        let index = game_map.map_index(1, 2);

        assert_eq!(index, 7);
    }

    #[test]
    fn test_map_coords() {
        let game_map = GameMap::new(3, 3);
        let coords = game_map.map_coords(7);

        assert_eq!(coords, (1, 2));
    }
}