use crate::types::position::Position;
use crate::{lab::Maze, types::direction::Direction};

use super::player::Player;
#[derive(Debug, Clone)]
/// Representa la rafaga. Posee una posicion, una direccion, un alcance, un codigo y una lista de jugadores atacados.
/// La direccion puede ser izquierda, derecha, arriba o abajo.
/// El codigo reprensenta que tipo de rafaga es. Si es la rafaga de una bomba comun es 'B' y si es de una super bomba es 'S'.
pub struct Blast {
    pub position: Position,
    pub direction: Direction,
    pub scope: usize,
    pub code: char,
    pub players_attacked: Vec<Player>,
}

impl Blast {
    pub fn new(position: Position, direction: Direction, scope: usize, code: char) -> Blast {
        Blast {
            position,
            direction,
            scope,
            code,
            players_attacked: Vec::new(),
        }
    }

    /// Recibe un jugador y nos indica si fue o no atacado.
    /// Se fija si el jugador se encuentra entre los jugadores atacados.
    /// En caso afirmativo, devuelve true.
    /// En caso negativo, agrega el jugador a la lista de jugadores atacados y devuelve false.
    pub fn attack_the_player(&mut self, player: &mut Player) -> bool {
        for player_attacked in &self.players_attacked {
            if player_attacked.position.equals(player.position.clone()) {
                return true;
            }
        }
        self.players_attacked.push(player.clone());
        false
    }

    /// Recibe una posicion y desplaza la rafaga a esa posicion. Luego devuelve un booleano indicando si se pudo desplazaro no.
    /// Si la nueva posicion es diferente a la actual y la rafaga todavia puede moverse porque posee alcance, actualizo la posiciion y disminuyo su alcance. Luego, devuelvo true.
    /// Si no puede desplazarse entonces devuelve false.
    fn move_to_position(&mut self, position: Position) -> bool {
        if !(self.position.equals(position.clone())) && self.scope > 0 {
            self.scope -= 1;
            self.position = position;
            return true;
        }
        false
    }

    /// Recibe el laberinto. Desvia la rafaga hacia la derecha, cambiando su direccion.
    /// En el caso de que me pueda mover a la celda derecha entonces comienzo a desplazarme con el alcance restante.
    pub fn deviate_to_right(&mut self, maze: &mut Maze) {
        self.direction = Direction::Right;
        if self.move_to_position(self.position.right()) {
            self.desplace(maze);
        }
    }

    /// Recibe el laberinto. Desvia la rafaga hacia la izquierda, cambiando su direccion.
    /// En el caso de que me pueda mover a la celda izquierda entonces comienzo a desplazarme con el alcance restante.
    pub fn deviate_to_left(&mut self, maze: &mut Maze) {
        self.direction = Direction::Left;
        if self.move_to_position(self.position.left()) {
            self.desplace(maze);
        }
    }

    /// Recibe el laberinto. Desvia la rafaga hacia arriba, cambiando su direccion.
    /// En el caso de que me pueda mover a la celda superior entonces comienzo a desplazarme con el alcance restante.
    pub fn deviate_to_up(&mut self, maze: &mut Maze) {
        self.direction = Direction::Up;
        if self.move_to_position(self.position.up()) {
            self.desplace(maze);
        }
    }

    /// Recibe el laberinto. Desvia la rafaga hacia abajo, cambiando su direccion.
    /// En el caso de que me pueda mover a la celda inferior entonces comienzo a desplazarme con el alcance restante.
    pub fn deviate_to_down(&mut self, maze: &mut Maze) {
        self.direction = Direction::Down;
        if self.move_to_position(self.position.down()) {
            self.desplace(maze);
        }
    }

    /// Recibe el laberinto. Desplaza la rafaga desde un punto hasta otro con su respectivo alcance y su respectiva direccion.
    pub fn desplace(&mut self, maze: &mut Maze) {
        let (start, end, a, b, c, d, reverse) = self.match_direction();

        if reverse {
            for index in (start..=end).rev() {
                if !self.match_element_and_apply(maze, a, b, c, d, index) {
                    break;
                }
            }
        } else {
            for index in start..=end {
                if !self.match_element_and_apply(maze, a, b, c, d, index) {
                    break;
                }
            }
        }
    }

    /// Recibe el laberinto, un iterador y coeficientes de calculo. Tanto los coeficientes de calculo como los iteradores seran usados para calcular la posicion a la que deseamos acceder.
    /// Luego de acceder al elemento que se encuentra en la respectiva posicion, la detonamos y devolvemos un booleano, que indica si el iterador debe finalizar o no.
    /// Ejemplo. Si encuentra una pared (W), devuelve false. Si encuentra vacio (_), devuelve true.
    fn match_element_and_apply(
        &mut self,
        maze: &mut Maze,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        index: usize,
    ) -> bool {
        if maze.is_in_maze(
            self.position.x * a + index * c,
            self.position.y * b + index * d,
        ) {
            self.move_to_position(Position::new(
                self.position.x * a + index * c,
                self.position.y * b + index * d,
            ));
            return maze.maze[self.position.y][self.position.x]
                .clone()
                .apply(maze, self);
        }
        false
    }

    /// En base a la direccion actual de la rafaga, devuelve el conjunto de variables que nos permitiran recorrer las posiciones.
    /// Devuelve el inicio del itearador, el fin, si vamos a recorrer en reverso y los coeficientes de calculo.
    /// La formula que utilizamos es (x*a+index*c, y*b+index*d). Si nos tenemos que mover en horizontal entonces los coeficientes seran 0 1 1 0, por ende si los reemplazamos en la expresion obtendremos (index, y). Esto indica que solo recorremos la direccion horizontal, ya que index sera variable e y fijo.
    fn match_direction(&self) -> (usize, usize, usize, usize, usize, usize, bool) {
        match self.direction {
            Direction::Up => (
                if self.scope > self.position.y {
                    0
                } else {
                    self.position.y - self.scope
                },
                self.position.y,
                1,
                0,
                0,
                1,
                true,
            ),
            Direction::Down => (
                self.position.y,
                self.position.y + self.scope,
                1,
                0,
                0,
                1,
                false,
            ),
            Direction::Left => (
                if self.scope > self.position.x {
                    0
                } else {
                    self.position.x - self.scope
                },
                self.position.x,
                0,
                1,
                1,
                0,
                true,
            ),
            Direction::Right => (
                self.position.x,
                self.position.x + self.scope,
                0,
                1,
                1,
                0,
                false,
            ),
        }
    }
}
