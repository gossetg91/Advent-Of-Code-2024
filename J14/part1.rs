use std::fs;

struct Position{
    x: i32,
    y: i32
}

fn main()
{
    let x_size = 103;
    let y_size = 101;
    let input_data = fs::read_to_string("inputJ14.txt").unwrap();

    let mut positions : Vec<Position> = Vec::new();

    for current in input_data.split("\n")
    {
        let input_splited = current.split(" v=").collect::<Vec<_>>();

        let init_pos = input_splited[0].split("p=").collect::<Vec<_>>()[1].split(",").map(|e| e.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let vector = input_splited[1].split(",").map(|e| e.parse::<i32>().unwrap()).collect::<Vec<_>>();

        let mut pos_x = (init_pos[1] + vector[1]*100) % x_size;
        let mut pos_y = (init_pos[0] + vector[0]*100) % y_size;

        if pos_x >= x_size
        {
            pos_x %= x_size;
        }
        else if pos_x < 0
        {
            pos_x += x_size;
        }

        if pos_y >= y_size
        {
            pos_y %= y_size;
        }
        else if pos_y < 0
        {
            pos_y += y_size;
        }

        positions.push(Position{x:pos_x , y:pos_y});
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for current in positions
    {
        if current.x < x_size/2 && current.y < y_size/2
        {
            q1 += 1;
        }
        else if current.x > x_size/2 && current.y < y_size/2
        {
            q3 += 1;
        }
        else if current.x < x_size/2 && current.y > y_size/2
        {
            q2 += 1;
        }
        else if current.x > x_size/2 && current.y > y_size/2
        {
            q4 += 1;
        }
    }

    println!("{}",q1*q2*q3*q4);
}