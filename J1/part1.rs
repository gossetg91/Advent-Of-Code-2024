use std::fs;

fn main()
{
    let content = fs::read_to_string("inputJ1.txt").unwrap();

    let mut array1 : Vec<i32> = Vec::new();
    let mut array2 : Vec<i32> = Vec::new();

    for (_, element) in content.split("\n").enumerate()
    {
        let tmp : Vec<_> = element.split("   ").collect();
        array1.push(tmp[0].parse().unwrap());
        array2.push(tmp[1].parse().unwrap());
    }

    array1.sort();
    array2.sort();

    let mut sum = 0;
    for i in 0..array1.len()
    {
        sum += (array1[i] - array2[i]).abs();
    }

    println!("{}", sum);
}
