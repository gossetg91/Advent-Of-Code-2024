use std::fs;

fn check(array : &Vec<i32>) -> bool
{
    let mut good = true;
    let mut previous : i32 = 0;

    for current in array.windows(2)
    {
        if !(1..4).contains(&(current[0] - current[1]).abs()) ||
            previous != 0 && previous.signum() != (current[0] - current[1]).signum()
        {
            good = false;
            break;
        }
        previous = current[0] - current[1];
    }

    return good;
}

fn main()
{

    let data = fs::read_to_string("inputJ2.txt").unwrap();
    let mut count = 0;

    for (_,element) in data.split("\n").enumerate()
    {
        let data_array = element.split(" ").map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
        if !check(&data_array)
        {
            let mut found = false;
            for i in 0..data_array.len()
            {
                if check(&(data_array.clone().into_iter().enumerate().filter(|&(j,_)| j != i ).map(|(_,j)| j).collect()))
                {
                    found = true;
                    break;
                }
            }

            if !found
            {
                continue;
            }
        }
        count += 1;
    }
    println!("{}", count);
}