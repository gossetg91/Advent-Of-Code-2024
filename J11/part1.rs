use std::fs;

fn blink(data: &Vec<i128>) -> Vec<i128>
{
    let mut result : Vec<i128> = Vec::new();

    for current in data
    {
        if *current == 0
        {
            result.push(1);
        }
        else if current.to_string().len()%2 == 0
        {
            let tmp = current.to_string();

            result.push(tmp[..tmp.len()/2].parse().unwrap());
            result.push(tmp[tmp.len()/2..].parse().unwrap());
        }
        else
        {
            result.push(current*2024);
        }
    }

    return result;
}

fn main()
{
    let input_data = fs::read_to_string("./inputJ11.txt").unwrap();

    let mut stone_list : Vec<i128> = input_data.split(' ').map(|e| e.parse().unwrap()).collect();

    for _i in 0..25
    {
        stone_list = blink(&stone_list);
    }

    println!("{}", stone_list.len());
}