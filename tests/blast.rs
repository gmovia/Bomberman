#[cfg(test)]
mod tests {

    use taller1_tp1_bomberman::element::blast::Blast;
    use taller1_tp1_bomberman::lab::Maze;
    use taller1_tp1_bomberman::types::direction::Direction;
    use taller1_tp1_bomberman::types::position::Position;

    #[test]
    fn test_01_create_blast() {
        let blast = Blast::new(Position::new(0, 0), Direction::Right, 3, 'B');
        assert_eq!(0, blast.position.x);
        assert_eq!(0, blast.position.y);
        assert_eq!(3, blast.scope);
    }

    #[test]
    fn test_02_move_to_right() {
        let mut maze: Maze = Maze::new("_ _ _ _");
        let mut blast = Blast::new(Position::new(0, 0), Direction::Right, 3, 'B');

        blast.desplace(&mut maze);

        assert_eq!(3, blast.position.x);
        assert_eq!(0, blast.position.y);
        assert_eq!(0, blast.scope);
    }

    #[test]
    fn test_03_move_to_right() {
        let mut maze: Maze = Maze::new("_ _ _ _");
        let mut blast = Blast::new(Position::new(0, 0), Direction::Right, 5, 'B');

        blast.desplace(&mut maze);

        assert_eq!(3, blast.position.x);
        assert_eq!(0, blast.position.y);
        assert_eq!(2, blast.scope);
    }

    #[test]
    fn test_04_move_to_right() {
        let mut maze: Maze = Maze::new("_ _ _ _");
        let mut blast = Blast::new(Position::new(1, 0), Direction::Right, 5, 'B');

        blast.desplace(&mut maze);

        assert_eq!(3, blast.position.x);
        assert_eq!(0, blast.position.y);
        assert_eq!(3, blast.scope);
    }

    #[test]
    fn test_05_move_to_left() {
        let mut maze: Maze = Maze::new("_ _ _ _");
        let mut blast = Blast::new(Position::new(2, 0), Direction::Left, 5, 'B');

        blast.desplace(&mut maze);

        assert_eq!(0, blast.position.x);
        assert_eq!(0, blast.position.y);
        assert_eq!(3, blast.scope);
    }

    #[test]
    fn test_06_move_to_up() {
        let mut maze: Maze = Maze::new("_ _ _\n_ _ _\n_ _ _");
        let mut blast = Blast::new(Position::new(2, 1), Direction::Up, 3, 'B');

        blast.desplace(&mut maze);

        assert_eq!(2, blast.position.x);
        assert_eq!(0, blast.position.y);
        assert_eq!(2, blast.scope);
    }

    #[test]
    fn test_07_move_to_down() {
        let mut maze: Maze = Maze::new("_ _ _\n_ _ _\n_ _ _");
        let mut blast = Blast::new(Position::new(2, 1), Direction::Down, 3, 'B');

        blast.desplace(&mut maze);

        assert_eq!(2, blast.position.x);
        assert_eq!(2, blast.position.y);
        assert_eq!(2, blast.scope);
    }

    #[test]
    fn test_08_move_to_up_and_deviate_to_right() {
        let mut maze: Maze = Maze::new("_ DR _\n_ _ _\n_ _ _");
        let mut blast = Blast::new(Position::new(1, 1), Direction::Up, 3, 'B');

        blast.position.x = 1;
        blast.position.y = 0;
        blast.scope -= 1;

        blast.deviate_to_right(&mut maze);

        assert_eq!(2, blast.position.x);
        assert_eq!(0, blast.position.y);
        assert_eq!(1, blast.scope);
    }

    #[test]
    fn test_09_move_to_up_and_deviate_to_left() {
        let mut maze: Maze = Maze::new("_ DR _\n_ _ _\n_ _ _");
        let mut blast = Blast::new(Position::new(1, 1), Direction::Up, 3, 'B');

        blast.position.x = 1;
        blast.position.y = 0;
        blast.scope -= 1;

        blast.deviate_to_left(&mut maze);

        assert_eq!(0, blast.position.x);
        assert_eq!(0, blast.position.y);
        assert_eq!(1, blast.scope);
    }

    #[test]
    fn test_09_move_to_right_and_deviate_to_up() {
        let mut maze: Maze = Maze::new("_ _ _\n_ _ DU\n_ _ _");
        let mut blast = Blast::new(Position::new(1, 1), Direction::Right, 3, 'B');

        blast.position.x = 2;
        blast.position.y = 1;
        blast.scope -= 1;

        blast.deviate_to_up(&mut maze);

        assert_eq!(2, blast.position.x);
        assert_eq!(0, blast.position.y);
        assert_eq!(1, blast.scope);
    }

    #[test]
    fn test_10_move_to_right_and_deviate_to_down() {
        let mut maze: Maze = Maze::new("_ _ _\n_ _ DU\n_ _ _");
        let mut blast = Blast::new(Position::new(1, 1), Direction::Right, 3, 'B');

        blast.position.x = 2;
        blast.position.y = 1;
        blast.scope -= 1;

        blast.deviate_to_down(&mut maze);

        assert_eq!(2, blast.position.x);
        assert_eq!(2, blast.position.y);
        assert_eq!(1, blast.scope);
    }
}
