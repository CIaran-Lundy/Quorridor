use crate::quorridor::Quorridor;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Piece {
    pub x: i64,
    pub y: i64,
}

pub fn move_player(game: &mut Quorridor, dx: i64, dy: i64) {
    let idx = game.active_player;
    // In 18x18 grid, movements are by 2 (odd to odd)
    game.player_pieces[idx].x = game.player_pieces[idx].x + dx;
    game.player_pieces[idx].y = game.player_pieces[idx].y + dy;
}
