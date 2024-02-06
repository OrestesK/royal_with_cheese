use crate::board::Cell;
fn bullet(active_tiles: &mut Vec<Cell>) {
    active_tiles.retain_mut(|tile| match tile.cell_type {
        1 => {
            if tile.y >= 1 {
                tile.y -= 1;
                return true;
            } else {
                return false;
            }
        }
        2 => {
            if tile.y <= 52 {
                tile.y += 1;
                return true;
            } else {
                return false;
            }
        }
        3 => {
            if tile.x <= 210 {
                tile.x -= 1;
                return true;
            } else {
                return false;
            }
        }
        4 => {
            if tile.x >= 2 {
                tile.x += 1;
                return true;
            } else {
                return false;
            }
        }
        _ => return true,
    });
}
pub fn process_game(mut active_tiles: Vec<Cell>) -> Vec<Cell> {
    bullet(&mut active_tiles);
    active_tiles
}
