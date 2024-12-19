use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Clone)]
struct Move{
    x: i32,
    y: i32,
    cost : i32
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Pos{
    x: usize,
    y: usize
}

struct State{
    position: Pos,
    total_cost : i32
}

fn main()
{
    let input_data = fs::read_to_string("inputJ13.txt").unwrap();

    let machine_data = input_data.split("\n\n");

    let mut token_count = 0;
    for current in machine_data
    {
        let machine_splited = current.split("\n").collect::<Vec<_>>();

        let a_button_tmp = machine_splited[0].split(", Y+").collect::<Vec<_>>();
        let b_button_tmp = machine_splited[1].split(", Y+").collect::<Vec<_>>();
        let prize_tmp = machine_splited[2].split(", Y=").collect::<Vec<_>>();

        let a_x = a_button_tmp[0].split("X+").collect::<Vec<_>>()[1].parse::<i32>().unwrap();
        let a_y = a_button_tmp[1].parse::<i32>().unwrap();

        let b_x = b_button_tmp[0].split("X+").collect::<Vec<_>>()[1].parse::<i32>().unwrap();
        let b_y = b_button_tmp[1].parse::<i32>().unwrap();

        let p_x = prize_tmp[0].split("X=").collect::<Vec<_>>()[1].parse::<i32>().unwrap();
        let p_y = prize_tmp[1].parse::<i32>().unwrap();

        let move_list = [Move{x:a_x, y:a_y, cost:3}, Move{x:b_x, y:b_y, cost:1}].to_vec();
        let destination = Pos{x: p_x as usize, y: p_y as usize};

        let mut awaiting : VecDeque<State> = VecDeque::new();
        awaiting.push_back(State{position: Pos{x:0, y:0}, total_cost:0});

        let mut move_done : HashSet<Pos> = HashSet::new();

        while !awaiting.is_empty()
        {
            let current_state = awaiting.pop_front().unwrap();

            if current_state.position == destination
            {
                token_count += current_state.total_cost;
                break;
            }
            else if current_state.position.x > destination.x || current_state.position.y > destination.y
            {
                continue;
            }

            for current_move in &move_list
            {
                let new_pos = Pos{x: (current_state.position.x as i32+current_move.x) as usize, y: (current_state.position.y as i32+current_move.y) as usize};

                if move_done.contains(&new_pos)
                {
                    continue;
                }

                move_done.insert(new_pos.clone());
                awaiting.push_back(State{position: new_pos, total_cost: current_state.total_cost + current_move.cost})
            }
        }
    }

    println!("{}", token_count);
}