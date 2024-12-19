use std::fs;

struct Position{
    x : usize,
    y : usize
}

struct Place{
    value: i8
}

fn dfs(start: Position, map : &mut Vec<Vec<Place>>) -> i32
{

    let mut stack : Vec<Position> = Vec::new();
    stack.push(start);

    let mut count = 0;

    while !stack.is_empty()
    {
        let current = stack.pop().unwrap();
        let current_value = map[current.x][current.y].value;

        if (current.x as i32)-1 >= 0 && map[current.x-1][current.y].value == current_value+1
        {
            stack.push(Position{x: current.x-1, y:current.y});
        }
        if current.x+1 < map.len() && map[current.x+1][current.y].value == current_value+1
        {
            stack.push(Position{x: current.x+1, y:current.y});
        }

        if (current.y as i32)-1 >= 0 && map[current.x][current.y-1].value == current_value+1
        {
            stack.push(Position{x: current.x, y:current.y-1});
        }
        if current.y+1 < map[current.x].len() && map[current.x][current.y+1].value == current_value+1
        {
            stack.push(Position{x: current.x, y:current.y+1});
        }

        count += (current_value == 9) as i32;
    }

    return count;
}

fn main()
{
    let input_data = fs::read_to_string("./inputJ10.txt").unwrap();

    let mut map : Vec<Vec<Place>> = Vec::new();

    for current in input_data.split('\n')
    {
        map.push(Vec::new());
        let current_last = map.len()-1;

        for current_value in current.chars().map(|e| e.to_string().parse::<i8>().unwrap())
        {
            map[current_last].push(Place{value: current_value});
        }
    }

    let mut final_score = 0;
    for i in 0..map.len()
    {
        for j in 0..map[i].len()
        {
            if map[i][j].value == 0
            {
                final_score += dfs(Position{x:i, y:j}, &mut map);
            }
        }
    }

    println!("{}", final_score);
}