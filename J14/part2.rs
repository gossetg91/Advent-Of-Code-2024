use std::fs;

#[derive(Clone, Copy)]
struct Position{
    x: i32,
    y: i32
}

const X_SIZE : i32 = 101;
const Y_SIZE : i32 = 103;

fn dfs(start: Position, map: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> i32
{
    let mut awaiting : Vec<Position> = Vec::new();

    let neighborhood = [Position{x:-1,y:0},Position{x:1,y:0},Position{x:0,y:-1},Position{x:0,y:1}].to_vec();

    awaiting.push(start);

    let mut section_size = 0;
    while !awaiting.is_empty()
    {
        let current = awaiting.pop().unwrap();
        visited[current.x as usize][current.y as usize] = true;
        section_size += 1;

        for current_neighbor in &neighborhood
        {
            let new_pos = Position{x:current.x+current_neighbor.x, y:current.y+current_neighbor.y};

            if new_pos.x >= X_SIZE || new_pos.x < 0 || new_pos.y >= Y_SIZE || new_pos.y < 0 
            || visited[new_pos.x as usize][new_pos.y as usize] || map[new_pos.x as usize][new_pos.y as usize] == 0
            {
                continue
            }

            awaiting.push(new_pos);
        }
    }
    return section_size;
}

fn main()
{
    let input_data = fs::read_to_string("inputJ14.txt").unwrap();

    let mut positions : Vec<Position> = Vec::new();
    let mut vectors : Vec<Position> = Vec::new();

    for current in input_data.split("\n")
    {
        let input_splited = current.split(" v=").collect::<Vec<_>>();

        let init_pos = input_splited[0].split("p=").collect::<Vec<_>>()[1].split(",").map(|e| e.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let vector = input_splited[1].split(",").map(|e| e.parse::<i32>().unwrap()).collect::<Vec<_>>();

        positions.push(Position{x:init_pos[0], y:init_pos[1]});
        vectors.push(Position{x:vector[0], y:vector[1]});
    }

    for epoch in 0..100000
    {
        for i in 0..positions.len()
        {
            positions[i].x += vectors[i].x;
            positions[i].y += vectors[i].y;
            
            if positions[i].x >= X_SIZE
            {
                positions[i].x %= X_SIZE;
            }
            if positions[i].y >= Y_SIZE
            {
                positions[i].y %= Y_SIZE;
            }

            if positions[i].x < 0
            {
                positions[i].x += X_SIZE;
            }
            if positions[i].y < 0
            {
                positions[i].y += Y_SIZE;
            }
        }

        let mut draw : Vec<Vec<i32>> = Vec::new();
        let mut visited: Vec<Vec<bool>> = Vec::new();

        for _i in 0..101
        {
            draw.push(Vec::new());
            visited.push(Vec::new());

            let last_index = draw.len()-1;
            for _j in 0..103
            {
                draw[last_index].push(0);
                visited[last_index].push(false);
            }
        }

        for position in &positions
        {
            draw[position.x as usize][position.y as usize] += 1;
        }

        let mut biggest_group = 0;
        for position in &positions
        {
            if !visited[position.x as usize][position.y as usize]
            {
                let section_size = dfs(*position, &draw, &mut visited);

                if section_size > biggest_group
                {
                    biggest_group = section_size;
                }

            }
        }

        if biggest_group > 30 
        {
            for current_row in &draw
            {
                for current_char in current_row
                {
                    if *current_char == 0
                    {
                        print!(" ");
                    }
                    else
                    {
                        print!("{}",current_char);
                    }
                }
                println!("");
            }
            println!("iter : {} ,Biggest : {}/{}", epoch+1, biggest_group, positions.len());
            break;
        }
   }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for current in &positions
    {
        if current.x < X_SIZE/2 && current.y < Y_SIZE/2
        {
            q1 += 1;
        }
        else if current.x > X_SIZE/2 && current.y < Y_SIZE/2
        {
            q3 += 1;
        }
        else if current.x < X_SIZE/2 && current.y > Y_SIZE/2
        {
            q2 += 1;
        }
        else if current.x > X_SIZE/2 && current.y > Y_SIZE/2
        {
            q4 += 1;
        }
    }

    println!("{}",q1*q2*q3*q4);
}