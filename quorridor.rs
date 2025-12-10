#[derive(Clone, Debug, Default, PartialEq)]
pub struct Quorridor {
    pub player_pieces: [Piece; 2],
    pub active_player: usize,
    pub walls: [Wall; 18],
    pub walls_remaining: [usize; 2],  // Each player can place up to 9 walls
    //pub number_of_walls_remaining: i32
}

trait Coordinates {
    fn coords(&self) -> (i64, i64);
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Piece {
    pub x: i64,
    pub y: i64,
    //pub number_of_walls_remaining: i32
}

impl Coordinates for Piece {
    fn coords(&self) -> (i64, i64) { (self.x, self.y) }
}

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
    // Returns the two positions this wall occupies
    pub fn positions(&self) -> [(i64, i64); 2] {
        match self.orientation {
            Orientation::Horizontal => [(self.x, self.y), (self.x + 1, self.y)],
            Orientation::Vertical => [(self.x, self.y), (self.x, self.y + 1)],
        }
    }
    
    // Check if this wall crosses another wall (different orientations intersecting)
    pub fn crosses(&self, other: &Wall) -> bool {
        // Walls of the same orientation can't cross (they're parallel)
        if self.orientation == other.orientation {
            return false;
        }
        
        match (self.orientation, other.orientation) {
            (Orientation::Horizontal, Orientation::Vertical) => {
                // self is horizontal at (x, y) spanning to (x+1, y)
                // other is vertical at (vx, vy) spanning to (vx, vy+1)
                // They cross if vx is in [x, x+1] and y is in [vy, vy+1]
                let vx = other.x;
                let vy = other.y;
                vx >= self.x && vx <= self.x + 1 && self.y >= vy && self.y <= vy + 1
            }
            (Orientation::Vertical, Orientation::Horizontal) => {
                // self is vertical, other is horizontal
                other.crosses(self)
            }
            _ => false,
        }
    }
}

impl Coordinates for Wall {
    fn coords(&self) -> (i64, i64) { (self.x, self.y) }
}

pub fn wall_collision(game: &Quorridor, x: i64, y: i64) -> bool {
    game.walls.iter().any(|wall| {
        if wall.x == 99 { return false; }  // Skip uninitialized walls
        wall.positions().contains(&(x, y))
    })
}

pub fn player_collision(game: &Quorridor, player_idx: usize, x: i64, y: i64) -> bool {
    for (i, piece) in game.player_pieces.iter().enumerate() {
        if i != player_idx && piece.coords() == (x, y) {
            return true;
        }
    }
    false
}

pub fn move_player_left(game: &mut Quorridor) {
    let idx = game.active_player;
    let current_x = game.player_pieces[idx].x;
    let current_y = game.player_pieces[idx].y;
    let candidate_x = current_x - 1;
    
    // Check bounds and collisions
    if candidate_x >= 0 && !wall_collision(game, candidate_x, current_y) && !player_collision(game, idx, candidate_x, current_y) {
        game.player_pieces[idx].x = candidate_x;
    }
}

pub fn move_player_right(game: &mut Quorridor) {
    let idx = game.active_player;
    let current_x = game.player_pieces[idx].x;
    let current_y = game.player_pieces[idx].y;
    let candidate_x = current_x + 1;
    
    // Check wall between current and candidate position
    if !wall_collision(game, candidate_x, current_y) && !player_collision(game, idx, candidate_x, current_y) {
        game.player_pieces[idx].x = candidate_x;
    }
}

pub fn move_player_up(game: &mut Quorridor) {
    let idx = game.active_player;
    let current_x = game.player_pieces[idx].x;
    let current_y = game.player_pieces[idx].y;
    let candidate_y = current_y + 1;
    
    // Check wall between current and candidate position
    if !wall_collision(game, current_x, candidate_y) && !player_collision(game, idx, current_x, candidate_y) {
        game.player_pieces[idx].y = candidate_y;
    }
}

pub fn move_player_down(game: &mut Quorridor) {
    let idx = game.active_player;
    let current_x = game.player_pieces[idx].x;
    let current_y = game.player_pieces[idx].y;
    let candidate_y = current_y - 1;
    
    // Check bounds and collisions
    if candidate_y >= 0 && !wall_collision(game, current_x, candidate_y) && !player_collision(game, idx, current_x, candidate_y) {
        game.player_pieces[idx].y = candidate_y;
    }
}

// BFS to find shortest path distance for a player to their goal
pub fn shortest_path_to_goal(game: &Quorridor, player_idx: usize) -> Option<usize> {
    use std::collections::{VecDeque, HashSet};
    
    let start = game.player_pieces[player_idx];
    let goal_y = if player_idx == 0 { 8 } else { 0 };
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back((start.x, start.y, 0usize));
    visited.insert((start.x, start.y));
    
    while let Some((x, y, dist)) = queue.pop_front() {
        // Check if we reached the goal
        if y == goal_y {
            return Some(dist);
        }
        
        // Try all four directions
        let moves = [
            (x + 1, y), // Right
            (x - 1, y), // Left
            (x, y + 1), // Up
            (x, y - 1), // Down
        ];
        
        for (nx, ny) in moves {
            // Check bounds
            if nx < 0 || nx >= 9 || ny < 0 || ny >= 9 {
                continue;
            }
            
            // Check if already visited
            if visited.contains(&(nx, ny)) {
                continue;
            }
            
            // Check wall collision (walls block movement)
            if wall_collision(game, nx, ny) {
                continue;
            }
            
            // Check player collision (can't move to opponent's square)
            if player_collision(game, player_idx, nx, ny) {
                continue;
            }
            
            visited.insert((nx, ny));
            queue.push_back((nx, ny, dist + 1));
        }
    }
    
    None // No path found
}

// Check if both players have a valid path to their goals
pub fn both_players_have_path(game: &Quorridor) -> bool {
    shortest_path_to_goal(game, 0).is_some() && shortest_path_to_goal(game, 1).is_some()
}

pub fn place_wall(game: &mut Quorridor, x: i64, y: i64, orientation: Orientation) -> WallPlacementResult {
    let idx = game.active_player;
    
    // Check if player has walls remaining
    if game.walls_remaining[idx] == 0 {
        return WallPlacementResult::NoWallsRemaining;
    }
    
    let new_wall = Wall { x, y, orientation };
    
    // Check if wall crosses any existing walls (different orientations intersecting)
    let crosses_existing = game.walls.iter().any(|w| {
        if w.x == 99 { return false; }
        new_wall.crosses(w)
    });
    
    if crosses_existing {
        return WallPlacementResult::Crosses;
    }
    
    // Check if wall overlaps with existing walls (same orientation sharing positions)
    let overlaps = game.walls.iter().any(|w| {
        if w.x == 99 { return false; }
        
        if w.orientation == new_wall.orientation {
            let new_positions = new_wall.positions();
            return w.positions().iter().any(|pos| new_positions.contains(pos));
        }
        
        false
    });
    
    if overlaps {
        return WallPlacementResult::Overlaps;
    }
    
    // Temporarily place the wall to check if paths still exist
    let wall_index = if idx == 0 {
        9 - game.walls_remaining[idx]
    } else {
        9 + (9 - game.walls_remaining[idx])
    };
    
    let old_wall = game.walls[wall_index];
    game.walls[wall_index] = new_wall;
    
    // Check if both players still have a path
    if both_players_have_path(game) {
        // Wall is valid, keep it and decrement counter
        game.walls_remaining[idx] -= 1;
        return WallPlacementResult::Success;
    } else {
        // Wall blocks a path, revert and reject
        game.walls[wall_index] = old_wall;
        return WallPlacementResult::BlocksPath;
    }
}