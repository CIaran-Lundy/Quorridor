use crate::piece::Piece;
use crate::wall::{Wall, Orientation};
use itertools::iproduct;

pub use crate::piece::move_player;
pub use crate::wall::place_wall;

// 18x18 grid: players on odd positions (1,3,5,7,9,11,13,15,17), walls on even positions (0,2,4,6,8,10,12,14,16)
// Player position (1,1) represents square 0,0 in old system
// Player position (17,17) represents square 8,8 in old system

#[derive(Clone, Debug, PartialEq)]
pub struct Quorridor {
    pub player_pieces: [Piece; 2],
    pub active_player: usize,
    pub grid: [[bool; 18]; 18],  // true = wall present
    pub walls_remaining: [usize; 2],
}


impl Quorridor {
    pub fn wall_collision(&self, target_x: i64, target_y: i64) -> bool {
        // TODO: Implement with 18x18 grid
        // Check the even-positioned cells between current and target
        false
        
        /*
        let current_x = self.player_pieces[self.active_player].x;
        let current_y = self.player_pieces[self.active_player].y;
        
        for wall in &self.walls {
            if wall.x == 99 { continue; }
            
            match wall.orientation {
                Orientation::Horizontal => {
                    // Wall spans (wall.x, wall.y) to (wall.x+1, wall.y)
                    // Blocks vertical movement across y=wall.y boundary
                    if (current_y == wall.y - 1 && target_y == wall.y) || 
                       (current_y == wall.y && target_y == wall.y - 1) {
                        if current_x == wall.x || current_x == wall.x + 1 {
                            return true;
                        }
                    }
                }
                Orientation::Vertical => {
                    // Wall spans (wall.x, wall.y) to (wall.x, wall.y+1)
                    // Blocks horizontal movement across x=wall.x boundary
                    if (current_x == wall.x - 1 && target_x == wall.x) || 
                       (current_x == wall.x && target_x == wall.x - 1) {
                        if current_y == wall.y || current_y == wall.y + 1 {
                            return true;
                        }
                    }
                }
            }
        }
        false
        */
    }

    pub fn player_collision(&self, player_idx: usize, x: i64, y: i64) -> bool {
        let opponent_idx = 1 - player_idx;
        self.player_pieces[opponent_idx].x == x && self.player_pieces[opponent_idx].y == y
    }
    
    pub fn wall_crosses(&self, x: i64, y: i64, orientation: Orientation) -> bool {
        // TODO: Implement with 18x18 grid
        false
        /*
        self.walls.iter().any(|other| {
            if other.x == 99 { return false; }
            if orientation == other.orientation {
                return false;
            }
            match (orientation, other.orientation) {
                (Orientation::Horizontal, Orientation::Vertical) => {
                    other.x >= x && other.x <= x + 1 && y >= other.y && y <= other.y + 1
                }
                (Orientation::Vertical, Orientation::Horizontal) => {
                    other.x >= x && other.x <= x + 1 && other.y >= y && other.y <= y + 1
                }
                _ => false,
            }
        })
        */
    }
    
    pub fn wall_overlaps(&self, x: i64, y: i64, orientation: Orientation) -> bool {
        // TODO: Implement with 18x18 grid
        false
        /*
        let new_positions = match orientation {
            Orientation::Horizontal => [(x, y), (x + 1, y)],
            Orientation::Vertical => [(x, y), (x, y + 1)],
        };
        
        self.walls.iter().any(|other| {
            if other.x == 99 { return false; }
            if other.orientation != orientation { return false; }
            
            other.positions().iter().any(|pos| new_positions.contains(pos))
        })
        */
    }
    
    pub fn both_players_have_path(&self) -> bool {
        has_path_to_goal(self, 0) && has_path_to_goal(self, 1)
    }
    
    pub fn wall_blocks_path(&mut self, x: i64, y: i64, orientation: Orientation) -> bool {
        // TODO: Implement with 18x18 grid
        false
        /*
        let idx = self.active_player;
        let walls_placed = 9 - self.walls_remaining[idx];
        let wall_index = if idx == 0 {
            walls_placed
        } else {
            9 + walls_placed
        };
        
        let new_wall = Wall { x, y, orientation };
        let old_wall = self.walls[wall_index];
        self.walls[wall_index] = new_wall;
        
        let blocks = !self.both_players_have_path();
        
        self.walls[wall_index] = old_wall;
        blocks
        */
    }

    pub fn get_movement_moves(&self) -> Vec<crate::Move> {
        let mut moves = Vec::new();
        let current_x = self.player_pieces[self.active_player].x;
        let current_y = self.player_pieces[self.active_player].y;
        // In 18x18 grid, players move by 2 (from odd position to next odd position)
        for (dx, dy, mov) in [(0, 2, crate::Move::Up), 
                              (0, -2, crate::Move::Down), 
                              (-2, 0, crate::Move::Left), 
                              (2, 0, crate::Move::Right)] {
            let new_x = current_x + dx;
            let new_y = current_y + dy;
            // Players must stay on odd positions (1-17)
            if new_x >= 1 && new_x <= 17 && new_y >= 1 && new_y <= 17 {
                if !self.wall_collision(new_x, new_y) && !self.player_collision(self.active_player, new_x, new_y) {
                    moves.push(mov);
                }
            }
        }
        moves
    }

    fn validate_wall_move(&self, x: i64, y: i64, orientation: &Orientation) -> bool {
        if self.walls_remaining[self.active_player] == 0 {
            return false;
        }
        if orientation == &Orientation::Horizontal && x == 8 {
            return false;
        }
        if orientation == &Orientation::Vertical && y == 8 {
            return false;
        }
        if self.wall_crosses(x, y, *orientation) {
            return false;
        }
        if self.wall_overlaps(x, y, *orientation) {
            return false;
        }
        if self.clone().wall_blocks_path(x, y, *orientation) {
            return false;
        }
        true
    }

    pub fn get_wall_moves(&self) -> Vec<crate::Move> {
        let mut moves = Vec::new();
        
        if self.walls_remaining[self.active_player] == 0 {
            return moves;
        }
        
        // Walls occupy even positions (0,2,4,6,8,10,12,14,16)
        // Horizontal walls span 3 cells horizontally on even y
        // Vertical walls span 3 cells vertically on even x
        for (x, y, orientation) in iproduct!(0..17, 0..17, [Orientation::Horizontal, Orientation::Vertical].iter()) {
            // Only allow even positions for walls
            if x % 2 != 0 || y % 2 != 0 {
                continue;
            }
            if !self.validate_wall_move(x, y, orientation) {
                continue;
            }
            moves.push(crate::Move::PlaceWall(x, y, orientation.clone()));
        }
        moves
    }

    pub fn game_over(&self) -> bool {
        // Player 0 starts at y=1, wants to reach y=17
        // Player 1 starts at y=17, wants to reach y=1
        self.player_pieces[0].y >= 17 || self.player_pieces[1].y <= 1
    }
}

impl Default for Quorridor {
    fn default() -> Self {
        Quorridor {
            player_pieces: [
                Piece { x: 9, y: 1 },   // Player 0 starts at bottom middle (grid position 4,0 in old system)
                Piece { x: 9, y: 17 }   // Player 1 starts at top middle (grid position 4,8 in old system)
            ],
            active_player: 0,
            grid: [[false; 18]; 18],  // No walls initially
            walls_remaining: [10, 10],
        }
    }
}


pub fn shortest_path_to_goal(game: &Quorridor, player_idx: usize) -> Option<usize> {
    use std::collections::{VecDeque, HashSet};
    
    let start = game.player_pieces[player_idx];
    let goal_y = if player_idx == 0 { 17 } else { 1 };  // Updated for 18x18 grid
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back((start.x, start.y, 0usize));
    visited.insert((start.x, start.y));
    
    while let Some((x, y, dist)) = queue.pop_front() {
        if y == goal_y {
            return Some(dist);
        }
        
        // Move by 2 in 18x18 grid (odd position to odd position)
        let moves = [
            (x + 2, y),
            (x - 2, y),
            (x, y + 2),
            (x, y - 2),
        ];
        
        for (nx, ny) in moves {
            // Stay within odd positions (1-17)
            if nx < 1 || nx > 17 || ny < 1 || ny > 17 {
                continue;
            }
            
            if visited.contains(&(nx, ny)) {
                continue;
            }
            
            if game.wall_collision(nx, ny) {
                continue;
            }
            
            if game.player_collision(player_idx, nx, ny) {
                continue;
            }
            
            visited.insert((nx, ny));
            queue.push_back((nx, ny, dist + 1));
        }
    }
    
    None
}

pub fn has_path_to_goal(game: &Quorridor, player_idx: usize) -> bool {
    use std::collections::HashSet;
    
    let start = game.player_pieces[player_idx];
    let goal_y = if player_idx == 0 { 17 } else { 1 };  // Updated for 18x18 grid
    
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    
    stack.push((start.x, start.y));
    visited.insert((start.x, start.y));
    
    while let Some((x, y)) = stack.pop() {
        if y == goal_y {
            return true;
        }
        
        // Move by 2 in 18x18 grid (odd position to odd position)
        let moves = [
            (x + 2, y),
            (x - 2, y),
            (x, y + 2),
            (x, y - 2),
        ];
        
        for (nx, ny) in moves {
            // Stay within odd positions (1-17)
            if nx < 1 || nx > 17 || ny < 1 || ny > 17 {
                continue;
            }
            
            if visited.contains(&(nx, ny)) {
                continue;
            }
            
            if game.wall_collision(nx, ny) {
                continue;
            }
            
            if game.player_collision(player_idx, nx, ny) {
                continue;
            }
            
            visited.insert((nx, ny));
            stack.push((nx, ny));
        }
    }
    
    false
}