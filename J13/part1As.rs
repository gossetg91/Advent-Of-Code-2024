use std::fs;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Clone)]
struct Move{
    mode_a : bool,
    x: i64,
    y: i64,
    cost : i64
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
struct Pos{
    x: usize,
    y: usize
}

fn dist(lhs:&Pos, rhs:&Pos) -> i64
{
    let x_diff : f64 = lhs.x as f64-rhs.x as f64;
    let y_diff : f64 = lhs.y as f64-rhs.y as f64;
    return (x_diff*x_diff + y_diff*y_diff).sqrt().floor() as i64;
}

#[derive(Eq, PartialEq, PartialOrd)]
struct State{
    position: Pos,
    in_a : bool,
    total_cost : i64,
    h_value : i64
}

impl Ord for State{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.total_cost+self.h_value)
            .cmp(&(other.total_cost+other.h_value))
    }
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

        let a_x = a_button_tmp[0].split("X+").collect::<Vec<_>>()[1].parse::<i64>().unwrap();
        let a_y = a_button_tmp[1].parse::<i64>().unwrap();

        let b_x = b_button_tmp[0].split("X+").collect::<Vec<_>>()[1].parse::<i64>().unwrap();
        let b_y = b_button_tmp[1].parse::<i64>().unwrap();

        let p_x = prize_tmp[0].split("X=").collect::<Vec<_>>()[1].parse::<i64>().unwrap();
        let p_y = prize_tmp[1].parse::<i64>().unwrap();

        let move_list_ori = [Move{mode_a:true, x:a_x, y:a_y, cost:3}, Move{mode_a:false, x:b_x, y:b_y, cost:1}].to_vec();

        let mut move_list : Vec<Move> = Vec::new();
        let mut mul = 1;
        for _i in 0..6
        {
            for current_move in &move_list_ori
            {
                move_list.push(Move{mode_a:current_move.mode_a, x:current_move.x*mul, y:current_move.y*mul, cost:current_move.cost*mul});
            }
            mul *= 10;
        }

        let destination = Pos{x: (p_x) as usize, y: (p_y) as usize};

        let mut awaiting : BinaryHeap<State> = BinaryHeap::new();


        let start_pos = Pos{x:0 , y:0};
        awaiting.push(State{position: start_pos.clone() ,total_cost:0,in_a:false, h_value: dist(&destination,&start_pos)});

        let mut move_done : HashSet<Pos> = HashSet::new();

        while !awaiting.is_empty()
        {
            let current_state = awaiting.pop().unwrap();

            if current_state.position == destination
            {
                token_count += current_state.total_cost;
                break;
            }

            for current_move in &move_list
            {
                if !current_move.mode_a && current_state.in_a
                {
                    continue;
                }

                let new_pos = Pos{x: (current_state.position.x as i64+current_move.x) as usize, y: (current_state.position.y as i64+current_move.y) as usize};

                if move_done.contains(&new_pos)
                {
                    continue;
                }

                if new_pos.x > destination.x || new_pos.y > destination.y
                {
                    continue;
                }

                move_done.insert(new_pos.clone());
                awaiting.push(State{position: new_pos.clone(),in_a:current_move.mode_a , total_cost: current_state.total_cost + current_move.cost, h_value: dist(&destination,&new_pos)});
            }
        }
    }

    println!("{}", token_count);
}