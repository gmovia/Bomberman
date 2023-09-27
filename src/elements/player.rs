use crate::elements::blast::Blast;
use crate::lab::Maze;
use crate::types::position::Position;
use crate::utils::maker::Maker;

#[derive(Debug, Clone)]
/// Representa al enemigo.
pub struct Player {
    pub position: Position,
    lifes: usize,
}

impl Player {
    pub fn new(lifes: usize, position: Position) -> Player {
        Player { lifes, position }
    }

    pub fn code(&self) -> String {
        if self.lifes > 0 {
            format!("{}{}", 'F', self.lifes)
        } else {
            "_".to_string()
        }
    }

    pub fn typef(&self) -> char {
        'F'
    }

    /// Recibe un laberinto y una rafaga. Trata de disminuir la cantidad de vidas del jugador.
    /// Si la rafaga no ataco al jugador y el jugador todavia tiene vidas, disminuyo su cantidad, y actualizo el mapa.
    pub fn be_detonated(&mut self, maze: &mut Maze, blast: &mut Blast) -> bool {
        if self.lifes > 0 && !blast.attack_the_player(self) {
            self.lifes -= 1;
        }
        maze.maze[self.position.y][self.position.x] =
            Maker::new_player(self.lifes, Position::new(self.position.x, self.position.y));
        true
    }
}
