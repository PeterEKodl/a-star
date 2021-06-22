extern crate rayon;
use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Position
{

    pub x: isize,
    pub y: isize

}

impl std::ops::Add for Position
{
    type Output = Self;
    fn add(self, other: Self) -> Self
    {
        Self{x: self.x + other.x, y: self.y + other.y}
    }
}

#[derive(Clone,Copy)]
struct Node
{
    pub pos:Position,
    pub parent_index:Option<usize>,
    pub g: usize,
    pub h: usize,
    pub open:bool
}

impl Node
{
    fn new(pos: Position, parent_index: Option<usize>, g: usize, h: usize) -> Self
    {
        Self
        {
            pos,
            parent_index,
            g,
            h,
            open: true
        }
    }
    fn f(&self) -> usize
    {
        self.g + self.h
    }

}

impl PartialEq for Node
{
    fn eq(&self, other: &Self) -> bool
    {
        self.f() == other.f()
    }
}

impl Eq for Node{}

impl PartialOrd for Node
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for Node
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        (self.f()).cmp(&other.f())       
    }
}

#[derive(Clone, Copy)]
pub enum Cell
{
    Start,
    Goal,
    Wall,
    Free,
    Path,
    Visited

}

pub struct Grid
{
    memory: std::vec::Vec<Cell>, 
    pub width: usize,
    pub height: usize
}

impl Grid
{

    pub fn new(width: usize, height: usize) -> Self
    {
        let mut memory: Vec<Cell> = std::vec::Vec::with_capacity(width*height);
        memory.resize(width*height, Cell::Free);
        Self {memory, width, height}
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell>
    {
        if x >= self.width || y >= self.height
        {
            return None;
        }
        Some(self.memory[x + y*self.width])
    }

    pub fn set_cell(&mut self, x: usize, y:usize, value:Cell)
    {
        self.memory[x + y*self.width] = value;
    }

    // Clears the entire grid
    pub fn clear(&mut self)
    {
        self.memory.fill(Cell::Free);
    }

    // Clears path and visited cells
    fn clear_path(&mut self)
    {
        for x in self.memory.iter_mut()
        {
            match *x
            {
                Cell::Path | Cell::Visited => *x = Cell::Free,
                _ => {}
            }
        }
    }

    

    fn solve(&mut self, start_pos: Option<Position>, goal_pos: Option<Position>) -> (Option<Vec<Position>>, Option<Vec<Node>>)
    {
        if start_pos.is_none() || goal_pos.is_none()
        {
            return (None, None);
        }
        self.clear_path();
        let start_pos = start_pos.unwrap();
        let goal_pos = goal_pos.unwrap();

        let heuristic = |a:&Position, b:&Position| ((a.x - b.x).pow(2) + (a.y-b.y).pow(2)) as usize; 
        
        
        let mut node_list = Vec::new();

        node_list.push(Node::new(start_pos, None, 0, heuristic(&start_pos, &goal_pos)));
        
        loop
        {
            // Get the smallest node from the open list, if there is none, the function returns
            // None
            let current_node_index: usize = match node_list.par_iter().enumerate().filter(|x| x.1.open).min_by(|a, b| a.1.cmp(b.1))
            {
                Some((value, _)) => value,
                None => return (None, None)
            };
           
            node_list[current_node_index].open = false;

            let mut offset = Position{x:-1, y:0};

            for _ in 0..4
            {
                std::mem::swap(&mut offset.y, &mut offset.x);
                offset.y = -offset.y;

                let node_pos = node_list[current_node_index].pos + offset;
                let mut node = Node::new(node_pos, Some(current_node_index), node_list[current_node_index].g + 1, heuristic(&node_pos, &goal_pos));
                if let Some(Cell::Wall) | None = self.get_cell(node_pos.x as usize, node_pos.y as usize)
                {
                    continue;
                }

                if node.pos == goal_pos
                {
                    let mut path = Vec::new();
                    loop
                    {
                        path.push(node.pos);
                        match node.parent_index
                        {
                            Some(index) => node = node_list[index],
                            None => return (Some(path), Some(node_list))
                        }
                    }
                }
                
                // Check if the node is in the open list.
                let identical_node = node_list.par_iter_mut().find_any(|x| x.pos == node.pos); 

                match identical_node
                {
                    Some(value) => 
                    {
                        if node.g < value.g && value.open
                        {
                            value.g = node.g;
                            value.parent_index = node.parent_index;
                        }
                    },
                    None =>  node_list.push(node)
                }

            }
        }
    }

    pub fn create_path(&mut self, start_pos: Option<Position>, goal_pos: Option<Position>)
    {
        let solution = self.solve(start_pos, goal_pos);
        // Add the visited nodes to the grid.
        if let Some(visited) = solution.1
        {
            for node in visited
            {
                if let Some(Cell::Free) = self.get_cell(node.pos.x as usize, node.pos.y as usize)
                {
                    self.set_cell(node.pos.x as usize, node.pos.y as usize, Cell::Visited);
                }
            }
        }
        // Add the path nodes to the grid.
        if let Some(path) = solution.0
        {
            for pos in path
            {
                if let Some(Cell::Free) | Some(Cell::Visited) = self.get_cell(pos.x as usize, pos.y as usize)
                {
                    self.set_cell(pos.x as usize, pos.y as usize, Cell::Path);
                }
            }
        }
        
        
    }

    
}




