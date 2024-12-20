use std::fs;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Ordering;

struct Node{
    accessible:bool,
    visited_n : bool,
    visited_s : bool,
    visited_e : bool,
    visited_w : bool,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd,Hash)]
struct Position{
    x:usize,
    y:usize,
    // 0:N 1:S 2:E 3:W
    heading:usize
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point{
    x: usize,
    y: usize
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

fn dfs(p: Position,end: Position,saved_true : &mut HashSet<Position> ,saved : &mut HashSet<Point>, best_cost:i32, current_cost:i32 ,min_costs: &mut HashMap<Position,i32>,map: &mut Vec<Vec<Node>>) -> bool
{
    if current_cost > best_cost
    {
        return false;
    }

    if !min_costs.contains_key(&p)
    {
        min_costs.insert(p,current_cost);
    }
    else if min_costs[&p] > current_cost
    {
        *min_costs.get_mut(&p).unwrap() = current_cost;
    }

    if p.x == end.x && p.y == end.y
    {
        saved_true.insert(p);
        saved.insert(Point{x: p.x,y: p.y});
        return true;
    }

    if p.heading == 0
    {
        if map[p.x][p.y].visited_n
        {
            return false
        }
        map[p.x][p.y].visited_n = true;
    }
    else if p.heading == 1
    {
        if map[p.x][p.y].visited_s
        {
            return false
        }
        map[p.x][p.y].visited_s = true;
    }
    else if p.heading == 2
    {
        if map[p.x][p.y].visited_e
        {
            return false
        }
        map[p.x][p.y].visited_e = true;
    }
    else if p.heading == 3
    {
        if map[p.x][p.y].visited_w
        {
            return false
        }
        map[p.x][p.y].visited_w = true;
    }

    let mut ok = false;
    for i in 0..NEIGHBORS.len()
    {

        if i == p.heading
        {
            let current_neighbor = NEIGHBORS[i];

            let new_position = Position{x: (p.x as i32+current_neighbor.x) as usize, y:(p.y as i32+current_neighbor.y) as usize, heading: p.heading};

            if min_costs.contains_key(&new_position) && min_costs[&new_position] <= current_cost+1
            {
                ok |= saved_true.contains(&new_position) && (current_cost+1) == min_costs[&new_position];
                continue;
            }

            if map[new_position.x][new_position.y].accessible
            {
                ok |= dfs(new_position,end,saved_true ,saved, best_cost, current_cost+1,min_costs, map);
            }
        }
        else
        {
            let new_position = Position{x: p.x as usize, y:p.y as usize, heading: i};

            if min_costs.contains_key(&new_position) && min_costs[&new_position] <= current_cost+1000
            {
                ok |= saved_true.contains(&new_position) && (current_cost+1000) == min_costs[&new_position];
                continue;
            }

            if (i==0 || i==1) && (p.heading==2 || p.heading==3)
            {
                ok |= dfs(new_position,end,saved_true, saved, best_cost, current_cost+1000,min_costs, map);
            }
            if (i==2 || i==3) && (p.heading==0 || p.heading==1)
            {
                ok |= dfs(new_position,end,saved_true ,saved, best_cost, current_cost+1000,min_costs, map);
            }
        }
    }

    if p.heading == 0
    {
        map[p.x][p.y].visited_n = false;
    }
    else if p.heading == 1
    {
        map[p.x][p.y].visited_s = false;
    }
    else if p.heading == 2
    {
        map[p.x][p.y].visited_e = false;
    }
    else if p.heading == 3
    {
        map[p.x][p.y].visited_w = false;
    }

    if ok
    {
        saved_true.insert(p);
        saved.insert(Point{x: p.x,y: p.y});
        return true;
    }

    return false;
}

const NEIGHBORS : [Neighbor;4] = [ Neighbor{x:-1,y:0},Neighbor{x:1,y:0},Neighbor{x:0,y:1},Neighbor{x:0,y:-1}];

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



    let mut awaiting : BinaryHeap<PrioEntry> = BinaryHeap::new();
    awaiting.push(PrioEntry{position:start, cost:0, h_value:get_distance(&start,&end)});

    let mut best_cost = 0;
    while !awaiting.is_empty()
    {
        let current = awaiting.pop().unwrap();

        if current.position.x == end.x && current.position.y == end.y
        {
            best_cost = current.cost;
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

        for i in 0..NEIGHBORS.len()
        {

            if i == current.position.heading
            {
                let current_neighbor = NEIGHBORS[i];

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

    for current in &mut map
    {
        for current_node in &mut *current
        {
            current_node.visited_n =false;
            current_node.visited_s =false;
            current_node.visited_w =false;
            current_node.visited_e =false;
        }
    }

    let mut builded_set : HashSet<Point> = HashSet::new();
    let mut builded_set2 : HashSet<Position> = HashSet::new();
    let mut min_costs : HashMap<Position,i32> = HashMap::new();
    dfs(start,end,&mut builded_set2,&mut builded_set,best_cost,0,&mut min_costs,&mut map);
    //Extraction of all shortestPath
    println!("{}", builded_set.len());
}
