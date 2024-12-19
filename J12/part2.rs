use std::fs;
use std::collections::HashMap;

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

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Bound{
    hw : usize,
    direction: char
}

fn extract_section(start : Pos, map : &mut Vec<Vec<Lot>>) -> i32
{
    let neighbors = [Move{x:-1,y:0}, Move{x:0,y:-1}, Move{x:1,y:0}, Move{x:0,y:1}].to_vec();
    let selected_type = map[start.x][start.y].lot_type;

    let mut lot_stack : Vec<Pos> = Vec::new();
    lot_stack.push(start);

    let mut area = 0;

    let mut bounds : HashMap<Bound,Vec<usize>> = HashMap::new();

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
                let mut new_bound = Bound{direction:'N',hw:current.x};
                let mut bound_place = current.y;

                if current_neighbor.x == 1
                {
                    new_bound.direction = 'S';
                }
                else if current_neighbor.y == -1
                {
                    new_bound.direction = 'W';
                    new_bound.hw = current.y;
                    bound_place = current.x
                }
                else if current_neighbor.y == 1
                {
                    new_bound.direction = 'E';
                    new_bound.hw = current.y;
                    bound_place = current.x
                }

                if !bounds.contains_key(&new_bound)
                {
                    bounds.insert(new_bound, Vec::new());
                }

                bounds.get_mut(&new_bound).unwrap().push(bound_place);
            }
            else if !map[new_x as usize][new_y as usize].visited && !map[new_x as usize][new_y as usize].enqued
            {
                map[new_x as usize][new_y as usize].enqued = true;
                lot_stack.push(Pos{x:new_x as usize, y:new_y as usize});
            }
        }
    }

    let mut bound_count = 0;
    for (_key, mut sequence) in bounds.into_iter()
    {
        sequence.sort();

        bound_count += 1;

        for i in 1..sequence.len()
        {
            if sequence[i] > sequence[i-1]+1
            {
                bound_count += 1;
            }
        }
    }
    return area*bound_count;
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