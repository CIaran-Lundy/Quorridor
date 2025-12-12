use crate::quorridor::Quorridor;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self { Orientation::Horizontal }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WallPlacementResult {
    Success,
    NoWallsRemaining,
    Crosses,
    Overlaps,
    BlocksPath,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Wall {
    pub x: i64,
    pub y: i64,
    pub orientation: Orientation,
}

impl Wall {
    pub fn positions(&self) -> [(i64, i64); 2] {
        match self.orientation {
            Orientation::Horizontal => [(self.x, self.y), (self.x + 1, self.y)],
            Orientation::Vertical => [(self.x, self.y), (self.x, self.y + 1)],
        }
    }
}

pub fn place_wall(game: &mut Quorridor, x: i64, y: i64, orientation: Orientation) -> WallPlacementResult {
    let idx = game.active_player;
    
    if game.walls_remaining[idx] == 0 {
        return WallPlacementResult::NoWallsRemaining;
    }
    
    // In 18x18 grid, walls occupy 3 consecutive cells
    // Horizontal wall at (x,y) occupies (x,y), (x+1,y), (x+2,y)
    // Vertical wall at (x,y) occupies (x,y), (x,y+1), (x,y+2)
    let positions: Vec<(usize, usize)> = match orientation {
        Orientation::Horizontal => vec![(x as usize, y as usize), ((x+2) as usize, y as usize), ((x+4) as usize, y as usize)],
        Orientation::Vertical => vec![(x as usize, y as usize), (x as usize, (y+2) as usize), (x as usize, (y+4) as usize)],
    };
    
    // Check if any position is already occupied
    for (px, py) in &positions {
        if game.grid[*py][*px] {
            return WallPlacementResult::Overlaps;
        }
    }
    
    // Place the wall
    for (px, py) in positions {
        game.grid[py][px] = true;
    }
    
    game.walls_remaining[idx] -= 1;
    WallPlacementResult::Success
}
