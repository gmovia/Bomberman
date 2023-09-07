pub struct Maze{
    maze: Vec<Vec<String>>
}

impl Maze{
    pub fn new(maze: &str) -> Maze{
        Maze{maze: Self::generate_matrix(maze)}
    }

    pub fn detonate(&mut self, x: usize, y: usize) -> Result<String, String>{
        if self.maze[x][y].contains('B'){
            self.detonate_bomb(x, y);
        }
        else{
            return Err("ERR".to_string());
        }
        Ok(self.generate_string_maze())
    }

    pub fn detonate_bomb(&mut self, x: usize, y: usize){
        let scoup = 1;
        self.maze[x][y] = "_".to_string();

        for row in x..x+scoup+1{
            if row < self.maze.len(){
                if self.maze[row][y].contains('B'){
                    self.detonate_bomb(row, y);
                }
            }
        }

        for column in y..y+scoup+1{
            if column < self.maze[0].len(){
                if self.maze[x][column].contains('B'){
                    self.detonate_bomb(x, column);
                }
            }
        }
    }

    pub fn generate_matrix(maze: &str) -> Vec<Vec<String>>{
        let mut matrix: Vec<Vec<String>> = Vec::new();
        let rows: Vec<String> = maze.split('\n').map(String::from).collect();
        for column in rows{
            matrix.push(column.split_whitespace().map(String::from).collect())
        }
        matrix
    }

    pub fn generate_string_maze(&self) -> String{
        let mut string_maze: String = String::new();
        for (index, row) in self.maze.iter().enumerate(){
            string_maze.push_str(&row.join(" "));
            if index != self.maze.len() - 1{
                string_maze.push_str("\n");
            }
        }
        string_maze
    }
}

