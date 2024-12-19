use std::fs;
use std::collections::HashMap;

fn blink(data: i128) -> Vec<i128>
{
    let mut result : Vec<i128> = Vec::new();

    if data== 0
    {
        result.push(1);
    }
    else if data.to_string().len()%2 == 0
    {
        let tmp = data.to_string();

        result.push(tmp[..tmp.len()/2].parse().unwrap());
        result.push(tmp[tmp.len()/2..].parse().unwrap());
    }
    else
    {
        result.push(data*2024);
    }

    return result;
}

fn compute(value:i128, remaining:i32, memo_count: &mut HashMap<i128,HashMap<i32,i128>>) -> i128
{
    if remaining ==0
    {
        return 1;
    }

    let mut return_sum = 0;
    for current in blink(value)
    {
        if !memo_count.contains_key(&current) || !memo_count[&current].contains_key(&remaining)
        {
            let computed = compute(current, remaining-1, memo_count);
            if !memo_count.contains_key(&current)
            {
                memo_count.insert(current, HashMap::new());
            }

            memo_count.get_mut(&current).unwrap().insert(remaining, computed);
        }

        return_sum += memo_count[&current][&remaining];
    }

    return return_sum;

}

fn main()
{
    let input_data = fs::read_to_string("./inputJ11.txt").unwrap();

    let stone_list : Vec<i128> = input_data.split(' ').map(|e| e.parse().unwrap()).collect();
    let mut memo_count : HashMap<i128,HashMap<i32,i128>> = HashMap::new();

    let mut final_count = 0;
    for current in stone_list
    {
        final_count += compute(current, 75, &mut memo_count);
    }

    println!("{}", final_count);
}