use std::fs;

struct MapNode{
    freq : char,
    antinode : bool
}

#[derive(Clone)]
struct Pos {
    x : i32,
    y : i32
}

fn check_in_dir(start_pos : Pos, frequency : char,map: &mut Vec<Vec<MapNode>>)
{
    for i in 0..map.len()
    {
        for j in 0..map[i].len()
        {
            if start_pos.x as usize != i && start_pos.y as usize != j && map[i][j].freq == frequency
            {
                let dist_x = (i as i32) - start_pos.x;
                let dist_y = (j as i32) - start_pos.y;

                let mut antinode1 = Pos{x: start_pos.x, y: start_pos.y};

                while antinode1.x >= 0 && antinode1.x < map.len() as i32 && antinode1.y >= 0 && antinode1.y < map[i].len() as i32
                {
                    map[antinode1.x as usize][antinode1.y as usize].antinode = true;
                    (antinode1.x, antinode1.y) = (antinode1.x-dist_x, antinode1.y-dist_y);
                }

                let mut antinode2 = Pos{x: start_pos.x+dist_x, y: start_pos.y+dist_y};

                if antinode2.x >= 0 && antinode2.x < map.len() as i32 && antinode2.y>= 0 && antinode2.y < map[i].len() as i32
                {
                    map[antinode2.x as usize][antinode2.y as usize].antinode = true;
                    (antinode2.x, antinode2.y) = (antinode2.x+dist_x, antinode2.y+dist_y);
                }
            }
        }
    }
}

fn main()
{
    let input_data = fs::read_to_string("inputJ8.txt").unwrap();

    let mut map : Vec<Vec<MapNode>> = Vec::new();

    for current_line in input_data.split('\n')
    {
        map.push(Vec::new());
        for current in current_line.chars()
        {
            let current_last = map.len()-1;
            map[current_last].push(MapNode{freq:current, antinode:false});
        }
    }

    for i in 0..(map.len() as i32)
    {
        for j in 0..(map[i as usize].len() as i32)
        {
            let freq = map[i as usize][j as usize].freq;

            if freq != '.'
            {
                check_in_dir(Pos{x:i, y:j}, freq, &mut map);
            }
        }
    }

    let mut final_count = 0;

    for i in 0..map.len()
    {
        for j in 0..map[i].len()
        {
            final_count += map[i][j].antinode as i32;
        }
    }

    println!("{}", final_count);
}