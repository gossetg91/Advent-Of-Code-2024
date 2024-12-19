use std::fs;
use std::collections::BTreeSet;
use std::cmp::Ordering;
struct Configuration {
    pos_x: i32,
    pos_y: i32,
    direction_x: i32,
    direction_y: i32
}

impl Ord for Configuration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pos_x.cmp(&other.pos_x).then(
            self.pos_y.cmp(&other.pos_y).then(
                self.direction_x.cmp(&other.direction_x).then(
                    self.direction_y.cmp(&other.direction_y)
                )
            )
        )
    }
}

impl PartialOrd for Configuration{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Configuration{
    fn eq(&self, other: &Self) -> bool {
        self.pos_x == other.pos_x && self.pos_y == other.pos_y && self.direction_x == other.direction_x && self.direction_y == other.direction_y
    }
}

impl Eq for Configuration {}


struct Node {
    obstructed: bool,
    visited: bool,
    init_node : bool
}

fn solve(map : &mut Vec<Vec<Node>>) -> bool
{
    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;

    let mut direction_x : i32 = -1;
    let mut direction_y : i32 = 0;

    let mut configurations_seen : BTreeSet<Configuration> = BTreeSet::new();
    let mut loops =false;

    for i in 0..map.len()
    {
        for j in 0..map[i].len()
        {
            map[i][j].visited = false;
            if map[i][j].init_node
            {
                pos_x = i as i32;
                pos_y = j as i32;
            }
        }
    }


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

            let new_configuration : Configuration = Configuration{pos_x:pos_x, pos_y:pos_y, direction_x:direction_x, direction_y:direction_y};

            if configurations_seen.contains(&new_configuration)
            {
                loops = true;
                break;
            }
            else
            {
                map[pos_x as usize][pos_y as usize].visited = true;
                configurations_seen.insert(new_configuration);
            }
        }
    }

    return loops;
}

fn main()
{
    let input_data = fs::read_to_string("inputJ6.txt").unwrap();

    let mut map : Vec<Vec<Node>> = Vec::new();

    for current_line in input_data.split("\n")
    {
        map.push(Vec::new());
        for current_node in current_line.chars()
        {
            if current_node == '.'
            {
                map.last_mut().unwrap().push(Node{obstructed:false, visited:false, init_node:false});
            }
            else if current_node == '^'
            {
                map.last_mut().unwrap().push(Node{obstructed:false, visited:true, init_node:true});
            }
            else
            {
                map.last_mut().unwrap().push(Node{obstructed:true, visited:false, init_node:false});
            }
        }
    }

    let mut final_count = 0;

    for i in 0..map.len()
    {
        for j in 0..map[i].len()
        {
            if map[i][j].obstructed
            {
                continue;
            }

            println!("Test courant : {}, {}", i, j);
            map[i][j].obstructed = true;
            final_count += solve(&mut map) as i32;
            map[i][j].obstructed = false;
        }
    }

    println!("{}", final_count);
}