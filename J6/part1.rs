use std::fs;

struct Node {
    obstructed: bool,
    visited: bool
}

fn main()
{
    let input_data = fs::read_to_string("inputJ6.txt").unwrap();

    let mut map : Vec<Vec<Node>> = Vec::new();
    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;

    for current_line in input_data.split("\n")
    {
        map.push(Vec::new());
        for current_node in current_line.chars()
        {
            if current_node == '.'
            {
                map.last_mut().unwrap().push(Node{obstructed:false, visited:false});
            }
            else if current_node == '^'
            {
                map.last_mut().unwrap().push(Node{obstructed:false, visited:true});
                pos_x = (map.len() as i32)-1 ;
                pos_y = (map[map.len()-1].len() as i32) -1 ;
            }
            else
            {
                map.last_mut().unwrap().push(Node{obstructed:true, visited:false});
            }
        }
    }

    let mut direction_x : i32 = -1;
    let mut direction_y : i32 = 0;

    while pos_x+direction_x >= 0 && pos_x+direction_x < map.len() as i32 && pos_y+direction_y >= 0 && pos_y+direction_y < map[0].len() as i32
    {
        if map[(pos_x+direction_x) as usize][(pos_y+direction_y) as usize].obstructed
        {
            (direction_x,direction_y) = (direction_y, -direction_x);
        }
        else
        {
            pos_x += direction_x ;
            pos_y += direction_y ;

            if pos_x < 0 || pos_x >= map.len() as i32 || pos_y < 0 || pos_y >= map[0].len() as i32
            {
                break;
            }

            map[pos_x as usize][pos_y as usize].visited = true;
        }
    }

    let mut final_count = 0;

    for current_row in map
    {
        for current_node in current_row
        {
            final_count += current_node.visited as i32;
        }
    }

    println!("{}", final_count);
}