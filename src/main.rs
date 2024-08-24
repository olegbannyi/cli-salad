use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "Fruit Salad",
    version = "1.0",
    author = "Oleg Bannyi <oleg.bannyi@gmail.com>",
    about = "Creates a fruit salad"
)]

struct Opts {
    #[clap(short, long, default_value = "0")]
    fruit_number: usize,

    #[clap(short, long)]
    user_list: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    if opts.user_list {
        let mut fruits = Vec::new();
        loop {
            println!("Enter a fruit to add to the salad or type 'done' to finish");
            let mut fruit = String::new();
            std::io::stdin().read_line(&mut fruit).unwrap();
            let fruit = fruit.trim();
            if fruit == "done" {
                break;
            }
            fruits.push(fruit.to_string());
        }

        match cli_salad::create_fruit_salad(0, fruits) {
            Ok(salad) => println!("Salad: {:?}", salad),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        println!("Creating salad with {} fruits", opts.fruit_number);
        match cli_salad::create_fruit_salad(opts.fruit_number, vec![]) {
            Ok(salad) => println!("Salad: {:?}", salad),
            Err(e) => eprintln!("Error: {}", e), 
        }
    }
}
