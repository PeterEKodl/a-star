extern crate rustbox;

use rustbox::*;
mod astar; use astar::*;

enum Mode
{
    Start,
    Goal,
    Wall,
    Free
}



struct Data
{
    pub grid: Grid,
    pub mode: Mode,
    pub start_pos: Option<Position>,
    pub goal_pos: Option<Position>

}

fn draw_grid(grid: &Grid, rustbox: &RustBox)
{
    for y in 0..grid.height
    {
        for x in 0..grid.width
        {
            match grid.get_cell(x, y).unwrap() 
            {
                Cell::Start => 
                {
                    rustbox.print_char(x, y, RB_BOLD, Color::Red, Color::Default, '@');
                },
                Cell::Goal =>
                {
                    rustbox.print_char(x, y, RB_BOLD, Color::Green, Color::Default, 'X'); 
                },
                Cell::Wall =>
                {
                    rustbox.print_char(x, y, RB_NORMAL, Color::White, Color::Default, '#');
                },
                Cell::Free => 
                {
                    rustbox.print_char(x, y, RB_NORMAL, Color::White, Color::Default, '.');
                },
                Cell::Path =>
                {
                    rustbox.print_char(x, y, RB_BOLD, Color::Yellow, Color::Default, '.');
                },
                Cell::Visited =>
                {
                    rustbox.print_char(x, y, RB_NORMAL, Color::Cyan, Color::Default, '.');
                }
            }
        }
    }
}

fn draw(rustbox: &RustBox, data: &Data)
{
    draw_grid(&data.grid, &rustbox);
    let mode:&str = match data.mode
    {
        Mode::Wall => "Wall",
        Mode::Free => "Free",
        Mode::Goal => "Goal",
        Mode::Start => "Start"
    };
    rustbox.print(0, 0, RB_REVERSE, Color::White, Color::Default, 
                  format!("[{}] | w Wall | f Free | g Goal | s Start | c Clear | p Solve | q Quit", mode).as_str());
}

fn handle_mouse(data: &mut Data, x: usize, y: usize)
{
    match data.mode
    {
        Mode::Free => data.grid.set_cell(x, y, Cell::Free),
        Mode::Wall => data.grid.set_cell(x, y, Cell::Wall),
        Mode::Goal => 
        {
            // Swaps out the cell.
            if let Option::Some(pos) = data.goal_pos
            {
                    data.grid.set_cell(pos.x as usize, pos.y as usize, Cell::Free);
            }
            data.goal_pos = Some(Position{x: x as isize, y: y as isize});
            data.grid.set_cell(x, y, Cell::Goal);

        },
        Mode::Start =>
        {
            // Swaps out the cell.
            if let Option::Some(pos) = data.start_pos
            {
                data.grid.set_cell(pos.x as usize, pos.y as usize, Cell::Free);
            }
            data.start_pos = Some(Position{x: x as isize, y: y as isize});
            data.grid.set_cell(x, y, Cell::Start); 
        }

    }
}
fn main() {
    let rustbox = RustBox::init(InitOptions::default()).unwrap();
    rustbox.set_input_mode(InputMode::EscMouse); 
    let mut data = Data 
    {
        mode: Mode::Wall,
        grid: Grid::new(rustbox.width(), rustbox.height()),
        start_pos: Option::None,
        goal_pos: Option::None
    };

    loop
    {
        draw(&rustbox, &data);
        rustbox.present();
        match rustbox.poll_event(false).unwrap()
        {
            Event::KeyEvent(c) => 
            {
                match c
                {
                    Key::Char('q') => break,
                    Key::Char('w') => data.mode = Mode::Wall,
                    Key::Char('f') => data.mode = Mode::Free,
                    Key::Char('g') => data.mode = Mode::Goal,
                    Key::Char('s') => data.mode = Mode::Start,
                    Key::Char('c') => data.grid.clear(),
                    Key::Char('p') => data.grid.create_path(data.start_pos, data.goal_pos),
                    _ => {}
                }
            },
            Event::MouseEvent(Mouse::Left, x, y) =>
            {
                handle_mouse(&mut data, x as usize, y as usize)
            },
            _ => {}
        }
    }
}
