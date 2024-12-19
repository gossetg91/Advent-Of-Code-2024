fn main()
{
    let mut test : Vec<i32> = Vec::new();

    for i in 0..100
    {
        test.push(i);
    }

    for i in 0..100
    {
        for current in &test[i..]
        {
            print!("{} ,", current);
        }
        print!("\n");
    }

}