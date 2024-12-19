use std::fs;

struct File{
    position : usize,
    size : usize,
    idx : i32
}

fn main()
{
    let input_data = fs::read_to_string("inputJ9.txt").unwrap().chars().map(|e| e.to_string().parse::<usize>().unwrap()).collect::<Vec<_>>();

    let mut hole_data : Vec<File> = Vec::new();
    let mut insert_data : Vec<File> = Vec::new();


    let mut current_pos = 0;
    let mut current_idx = 0;

    for i in 0..input_data.len()
    {
        if i%2 == 0
        {
            insert_data.push(File{position: current_pos,size: input_data[i], idx:current_idx});
            current_idx += 1;
        }
        else
        {
            hole_data.push(File{position: current_pos,size: input_data[i],idx:-1});
        }

        current_pos += input_data[i];
    }

    insert_data.reverse();

    for current in &mut insert_data
    {
        for current_hole in &mut hole_data
        {
            if current_hole.position > current.position
            {
                break;
            }

            if current_hole.size >= current.size
            {
                current.position = current_hole.position;
                (current_hole.position, current_hole.size) = (current_hole.position+current.size, current_hole.size-current.size);
                break;
            }
        }
    }

    let mut check_sum : i128= 0;
    for current in insert_data
    {
        for i in 0..current.size
        {
            check_sum += (current.position+i) as i128 * current.idx as i128;
        }
    }

    println!("{}", check_sum);
}