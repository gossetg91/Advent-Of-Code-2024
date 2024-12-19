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
        let current_splited = current_sequence.split(",").collect::<Vec<_>>();
        let mut incorrect = false;

        for i in 1..current_splited.len()
        {
            for j in 0..i
            {
                if priorities[current_splited[i]].contains(current_splited[j])
                {
                    incorrect = true;
                    break;
                }
            }

            if incorrect
            {
                break;
            }
        }

        if !incorrect
        {
            final_counter += current_splited[current_splited.len()/2].parse::<i32>().unwrap();
        }
    }

    println!("{}", final_counter);
}