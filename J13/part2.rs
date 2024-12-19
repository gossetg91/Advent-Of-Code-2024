use std::fs;

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

        let a_x : f64 = a_button_tmp[0].split("X+").collect::<Vec<_>>()[1].parse::<i64>().unwrap() as f64;
        let a_y : f64 = a_button_tmp[1].parse::<i64>().unwrap() as f64;

        let b_x : f64 = b_button_tmp[0].split("X+").collect::<Vec<_>>()[1].parse::<i64>().unwrap() as f64;
        let b_y : f64 = b_button_tmp[1].parse::<i64>().unwrap() as f64;

        let p_x : f64 = (prize_tmp[0].split("X=").collect::<Vec<_>>()[1].parse::<i64>().unwrap() + 10000000000000) as f64;
        let p_y : f64 = (prize_tmp[1].parse::<i64>().unwrap()+10000000000000) as f64;

        let n_b = (p_x * a_y - a_x * p_y) / (a_y*b_x - a_x*b_y);
        let n_a = (p_y-n_b*b_y)/a_y;

        if n_a - ((n_a as i64) as f64) == 0.0 && n_b - ((n_b as i64) as f64) == 0.0 {
            token_count += n_a as i64*3 + n_b as i64;
        }
    }

    println!("{}", token_count);
}