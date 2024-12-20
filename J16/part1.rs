use std::fs;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct Node{
    accessible:bool,
    visited_n : bool,
    visited_s : bool,
    visited_e : bool,
    visited_w : bool
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
struct Position{
    x:usize,
    y:usize,
    // 0:N 1:S 2:E 3:W
    heading:usize
}

#[derive(Copy, Clone)]
struct Neighbor{
    x:i32,
    y:i32
}

#[derive(Eq, PartialEq, Copy, Clone )]
struct PrioEntry{
    position : Position,
    cost : i32,
    h_value : i32
}

impl Ord for PrioEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        return (other.cost+other.h_value).cmp(&(self.cost+self.h_value));
    }
}

impl PartialOrd for PrioEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_distance(p1 : &Position, p2 :&Position) -> i32
{
    let x_diff : f32 = ((p2.x as f32) - (p1.x as f32)) as f32;
    let y_diff : f32 = ((p2.y as f32) - (p1.y as f32)) as f32;

    return (x_diff*x_diff+y_diff*y_diff).sqrt().floor() as i32
}

fn main()
{
    let input_data = fs::read_to_string("./inputJ16.txt").unwrap();

    let mut map : Vec<Vec<Node>> = Vec::new();

    let mut start : Position = Position{x:0,y:0,heading:0};
    let mut end : Position = Position{x:0,y:0,heading:0};

    for current_row in input_data.split("\n")
    {
        if current_row == ""
        {
            continue;
        }

        map.push(Vec::new());

        let last_index = map.len()-1;
        for current_node in current_row.chars()
        {
            if current_node == '#'
            {
                map[last_index].push(Node{accessible:false, visited_n:false, visited_s:false, visited_e:false, visited_w:false});
            }
            else
            {
                if current_node == 'S'
                {
                    start = Position{x:last_index, y:map[last_index].len(), heading:2};
                }
                else if current_node == 'E'
                {
                    end = Position{x:last_index, y:map[last_index].len(), heading:2};
                }

                map[last_index].push(Node{accessible:true, visited_n:false, visited_s:false, visited_e:false, visited_w:false});
            }
        }
    }

    let neighbors = [ Neighbor{x:-1,y:0},Neighbor{x:1,y:0},Neighbor{x:0,y:1},Neighbor{x:0,y:-1}].to_vec();


    let mut awaiting : BinaryHeap<PrioEntry> = BinaryHeap::new();
    awaiting.push(PrioEntry{position:start, cost:0, h_value:get_distance(&start,&end)});

    let mut final_cost = 0;
    while !awaiting.is_empty()
    {
        let current = awaiting.pop().unwrap();

        if current.position.x == end.x && current.position.y == end.y
        {
            final_cost = current.cost;
            break;
        }

        if current.position.heading == 0
        {
            if map[current.position.x][current.position.y].visited_n
            {
                continue;
            }
            map[current.position.x][current.position.y].visited_n = true;
        }
        else if current.position.heading == 1
        {
            if map[current.position.x][current.position.y].visited_s
            {
                continue;
            }
            map[current.position.x][current.position.y].visited_s = true;
        }
        else if current.position.heading == 2
        {
            if map[current.position.x][current.position.y].visited_e
            {
                continue;
            }
            map[current.position.x][current.position.y].visited_e = true;
        }
        else if current.position.heading == 3
        {
            if map[current.position.x][current.position.y].visited_w
            {
                continue;
            }
            map[current.position.x][current.position.y].visited_w = true;
        }

        for i in 0..neighbors.len()
        {

            if i == current.position.heading
            {
                let current_neighbor = neighbors[i];

                let new_position = Position{x: (current.position.x as i32+current_neighbor.x) as usize, y:(current.position.y as i32+current_neighbor.y) as usize, heading: current.position.heading};

                if map[new_position.x][new_position.y].accessible
                {
                    awaiting.push(PrioEntry{position: new_position, cost: current.cost + 1, h_value: get_distance(&new_position,&end)});
                }
            }
            else
            {
                let new_position = Position{x: current.position.x as usize, y:current.position.y as usize, heading: i};
                if (i==0 || i==1) && (current.position.heading==2 || current.position.heading==3)
                {
                    awaiting.push(PrioEntry{position: new_position, cost: current.cost + 1000, h_value: current.h_value});
                }
                if (i==2 || i==3) && (current.position.heading==0 || current.position.heading==1)
                {
                    awaiting.push(PrioEntry{position: new_position, cost: current.cost + 1000, h_value: current.h_value});
                }
            }
        }
    }

    println!("{}", final_cost);
}
