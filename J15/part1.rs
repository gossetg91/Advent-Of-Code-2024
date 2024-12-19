use std::fs;

struct Tile{
    has_box : bool,
    is_wall : bool
}

#[derive(Clone, Copy)]
struct Pos{
    x:usize,
    y:usize
}

fn move_tile(dir: char, position: &mut Pos, map : &mut Vec<Vec<Tile>>, init:bool) -> bool
{

    if map[position.x][position.y].is_wall
    {
        return false;
    }
    if !init && !map[position.x][position.y].has_box
    {
        return true;
    }


    let mut new_pos = position.clone();

    if dir=='^'
    {
        new_pos.x -= 1;
    }
    if dir=='v'
    {
        new_pos.x += 1;
    }
    if dir=='>'
    {
        new_pos.y += 1;
    }
    if dir=='<'
    {
        new_pos.y -= 1;
    }

    if move_tile(dir, &mut new_pos, map, false)
    {
        map[new_pos.x][new_pos.y].has_box = map[position.x][position.y].has_box;
        map[position.x][position.y].has_box = false;

        if init
        {
            *position = new_pos;
        }
        return true;
    }
    else
    {
        return false;
    }
}

fn main()
{
    let input_data = fs::read_to_string("inputJ15.txt").unwrap();


    let mut first_line = "";
    let mut first_finished = false;

    let mut move_sequence : Vec<char> = Vec::new();

    let mut map : Vec<Vec<Tile>> = Vec::new();
    let mut robot_pos = Pos{x:0,y:0};
    for current in input_data.split("\n")
    {
        if !first_finished
        {
            map.push(Vec::new());
            let last_index = map.len()-1;

            for current_char in current.chars()
            {
                if current_char == '@'
                {
                    robot_pos.x = last_index;
                    robot_pos.y = map[last_index].len();
                }
                map[last_index].push(Tile{has_box:current_char == 'O', is_wall:current_char == '#'});
            }
            first_finished = current == first_line;
        }
        else
        {
            for current_char in current.chars()
            {
                move_sequence.push(current_char);
            }
        }

        if first_line == ""
        {
            first_line = current;
            continue;
        }
    }

    for current_move in move_sequence
    {
        move_tile(current_move,&mut robot_pos,&mut map, true);
    }

    let mut final_count = 0;
    for i in 0..map.len()
    {
        for j in 0..map[i].len()
        {
            let current_tile = &map[i][j];
            if current_tile.has_box
            {
                final_count += i*100+j;
                print!("O");
            }
            else if current_tile.is_wall
            {
                print!("#");
            }
            else
            {
                print!(".");
            }
        }
        println!();
    }

    println!("{}",final_count);
}
