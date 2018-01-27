use std::env;

fn  main()
{
    let options: Vec<String> = env::args().map(|option| option.starts_with('-').collect());
    for option in options
    {
        println!("{}", option);
    }
}
