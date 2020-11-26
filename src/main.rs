use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        std::process::exit(0);
    }

    let target = &args[1].to_lowercase();

    let path = env::current_dir()?;
    match path.to_str() {
        Some(path_str) => {
            let vec: Vec<&str> = path_str.split("/").collect();

            let mut counter = 1;
            for directory in vec.iter() {
                if directory.to_lowercase().contains(target) {
                    let new_path = vec[0..counter].join("/");
                    println!("{}", new_path);
                }
                counter += 1;
            }
        }
        None => println!("Path can't be split"),
    }
    Ok(())
}
