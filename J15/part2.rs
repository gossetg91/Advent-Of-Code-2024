use std::fs;

struct Tile{
    has_box : bool,
    is_wall : bool,
    is_right: bool
}

#[derive(Clone, Copy)]
struct Pos{
    x:usize,
    y:usize
}

fn move_tile(dir: char, position: &mut Pos, map : &mut Vec<Vec<Tile>>, init:bool, temp:bool) -> bool
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

    if dir=='^' {
        new_pos.x -= 1;
    }
    if dir=='v' {
        new_pos.x += 1;
    }
    if dir=='>' {
        new_pos.y += 1;
    }
    if dir=='<' {
        new_pos.y -= 1;
    }

    let mut can_move = move_tile(dir, &mut new_pos, map, false, true);

    if  can_move && map[position.x][position.y].has_box && (dir == '^' || dir == 'v') {
        let mut pos_alt = position.clone();
        let mut new_pos_alt = new_pos.clone();

        if map[position.x][position.y].is_right {
            pos_alt.y -= 1;
            new_pos_alt.y -= 1;
        }
        else{
            pos_alt.y += 1;
            new_pos_alt.y += 1;
        }

        can_move &= move_tile(dir, &mut new_pos_alt, map, false, temp);

        if can_move {
            if !temp
            {
                map[new_pos_alt.x][new_pos_alt.y].has_box = map[pos_alt.x][pos_alt.y].has_box;
                map[new_pos_alt.x][new_pos_alt.y].is_right = map[pos_alt.x][pos_alt.y].is_right;

                map[pos_alt.x][pos_alt.y].has_box = false;
                map[pos_alt.x][pos_alt.y].is_right = false;
            }
        }
    }

    if can_move {
        move_tile(dir, &mut new_pos, map, false, temp);
        if !temp
        {
            map[new_pos.x][new_pos.y].has_box = map[position.x][position.y].has_box;
            map[new_pos.x][new_pos.y].is_right = map[position.x][position.y].is_right;

            map[position.x][position.y].has_box = false;
            map[position.x][position.y].is_right = false;

            if init {
                *position = new_pos;
            }
        }
        return true;
    }
    else {
        return false;
    }
}

fn main() {
    let input_data = fs::read_to_string("inputJ15.txt").unwrap();


    let mut first_line = "";
    let mut first_finished = false;

    let mut move_sequence : Vec<char> = Vec::new();

    let mut map : Vec<Vec<Tile>> = Vec::new();
    let mut robot_pos = Pos{x:0,y:0};
    for current in input_data.split("\n") {
        if !first_finished {
            map.push(Vec::new());
            let last_index = map.len()-1;

            for current_char in current.chars() {
                if current_char == '@' {
                    robot_pos.x = last_index;
                    robot_pos.y = map[last_index].len();
                }

                if current_char == 'O' {
                    map[last_index].push(Tile{has_box:true, is_wall:false, is_right:false});
                    map[last_index].push(Tile{has_box:true, is_wall:false, is_right:true});
                }
                else {
                    map[last_index].push(Tile{has_box:false , is_wall:current_char == '#', is_right:false});
                    map[last_index].push(Tile{has_box:false , is_wall:current_char == '#', is_right:false});
                }
            }
            first_finished = current == first_line;
        }
        else {
            for current_char in current.chars()
            {
                move_sequence.push(current_char);
            }
        }

        if first_line == "" {
            first_line = current;
            continue;
        }
    }

    for current_move in move_sequence {
        move_tile(current_move,&mut robot_pos,&mut map, true, false);
    }

    let mut final_count = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let current_tile = &map[i][j];
            if current_tile.has_box {
                if current_tile.is_right {
                    print!("]");
                }
                else {
                    print!("[");
                    final_count += i*100+j;
                }
            }
            else if current_tile.is_wall {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }

    println!("{}",final_count);
}
