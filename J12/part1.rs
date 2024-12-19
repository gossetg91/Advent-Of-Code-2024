use std::fs;

struct Lot{
    lot_type: char,
    visited: bool,
    enqued: bool
}

struct Pos{
    x: usize,
    y: usize
}

#[derive(Clone)]
struct Move{
    x:i32,
    y:i32
}

fn extract_section(start : Pos, map : &mut Vec<Vec<Lot>>) -> i32
{
    let neighbors = [Move{x:-1,y:0}, Move{x:0,y:-1}, Move{x:1,y:0}, Move{x:0,y:1}].to_vec();
    let selected_type = map[start.x][start.y].lot_type;

    let mut lot_stack : Vec<Pos> = Vec::new();
    lot_stack.push(start);

    let mut area = 0;
    let mut perimeter = 0;

    while !lot_stack.is_empty()
    {
        let current = lot_stack.pop().unwrap();
        map[current.x][current.y].visited = true;

        area += 1;

        for current_neighbor in &neighbors
        {
            let new_x = current.x as i32 + current_neighbor.x ;
            let new_y = current.y as i32 + current_neighbor.y ;


            if new_x < 0 || new_x >= map.len() as i32 
            || new_y < 0 || new_y >= map[new_x as usize].len() as i32||
            map[new_x as usize][new_y as usize].lot_type != selected_type
            {
                perimeter += 1;
            }
            else if !map[new_x as usize][new_y as usize].visited && !map[new_x as usize][new_y as usize].enqued
            {
                map[new_x as usize][new_y as usize].enqued = true;
                lot_stack.push(Pos{x:new_x as usize, y:new_y as usize});
            }
        }
    }

    return area*perimeter;
}

fn main()
{
    let input_data = fs::read_to_string("./inputJ12.txt").unwrap();
    let mut map : Vec<Vec<Lot>> = Vec::new();

    for current in input_data.split('\n')
    {
        map.push(Vec::new());
        let last_index = map.len() - 1;

        for current_lot in current.chars()
        {
            map[last_index].push(Lot{lot_type: current_lot, visited:false, enqued:false});
        }
    }

    let mut total_cost = 0;
    for i in 0..map.len()
    {
        for j in 0..map[i].len()
        {
            if !map[i][j].visited
            {
                total_cost += extract_section(Pos{x:i,y:j},&mut map);
            }
        }
    }
    println!("{}", total_cost);
}