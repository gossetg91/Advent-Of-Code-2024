use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main()
{
    let input_data = fs::read_to_string("inputJ5.txt").unwrap();
    let splited = input_data.split("\n\n").collect::<Vec<_>>();

    let mut priorities : HashMap<_, HashSet<_>> = HashMap::new();

    for current in splited[0].split("\n")
    {
        let values : Vec<_> = current.split('|').collect::<Vec<_>>();
        if values.len() == 2
        {
            priorities.entry(values[0]).or_default().insert(values[1]);
        }
    }

    let mut final_counter : i32 = 0;

    for current_sequence in splited[1].split("\n")
    {
        let mut current_splited = current_sequence.split(",").collect::<Vec<_>>();
        let mut to_count = false;

        let mut i = 1;
        while i < current_splited.len()
        {
            for j in 0..i
            {
                if priorities.contains_key(current_splited[i]) && priorities[current_splited[i]].contains(current_splited[j])
                {
                    (current_splited[i], current_splited[j]) = (current_splited[j], current_splited[i]);
                    i -= 1;
                    to_count = true;

                    break;
                }
            }
            i += 1;
        }

        if to_count 
        {
            final_counter += current_splited[current_splited.len()/2].parse::<i32>().unwrap();
        }
    }

    println!("{}", final_counter);
}