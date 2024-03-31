use colored::Colorize;
use clap::Parser;
use anyhow::{Context, Result};
use std::io::{self, Read};
#[derive(Parser)]
struct Options {

    #[clap(default_value = "Miaow!")]
    /// What does the cat say ?
    message: String,
    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,

    #[clap(short = 'i', long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool

}
fn main() {
    let mut options = Options::parse();
    let mut message = options.message.clone();
    let mut message_stdin = String::new();
   
    if options.stdin {
        match io::stdin().read_to_string(&mut message_stdin) {
            Ok(_) => {
                // Successfully read from stdin
                message = message_stdin;
            }
            Err(err) => {
                eprintln!("Error reading from stdin: {}", err);
                // You can choose to exit the program or handle the error differently
            }
        }
    }


    if check_woof(&message) {
        eprintln!("A cat shouldn't bark like a dog. 😿😿😿😿");
        options.dead = true;
    }
    if options.dead {
        dead();
    }
    else if !options.catfile.is_some() {
        alive(&mut message);
    }
    else if options.catfile.is_some() {
        if let Err(err) = read_from_file(&mut options, &mut message) {
            eprintln!("Error reading file: {}", err);
        }
    }

}
fn read_from_file(options: &mut Options, message: &mut String) -> Result<()> {
    match &options.catfile {
        Some(path) => {
            let cat_template =  std::fs::read_to_string(path).with_context(|| format!("Could not read file {:?}", path))?;
            // .expect( &format!("could not read file {:?}", path));
        let eye = "O";
        let eye = format!("{}",eye.red().bold());
        let cat_picture = cat_template.replace("{eye}",&eye);
        println!("{}", message.bright_yellow().underline().on_purple());
        println!("{}", &cat_picture);
        },
        None =>{
            dead();
        }
    }
    Ok(())
}

fn check_woof(message: &str) -> bool {
    // Regular expression pattern: match one or more 'o' characters after 'w' and before 'f'
    let pattern = regex::Regex::new(r"^w+o+f+$").unwrap();
    pattern.is_match(message)
}

fn alive(message: &mut String){
    println!("{}",message.bright_yellow().underline().on_purple());
    println!("");
    println!("\\");
    println!(" \\");
    println!("  \\");
    println!("       /\\___/\\");
    println!("      /       \\");
    println!("     l  u   u  l");
    println!("   --l----ω----l--");
    println!("      \\   ^   /     - Meow!");
    println!("        ======");
    println!("      /       \\ __");
    println!("      l        l\\ \\");
    println!("      l        l/ /   ");
    println!("      l  l l   l /");
    println!("      \\ ml lm /_/");
}

fn dead() {
    let eye = "X";
    println!("");
    println!("\\");
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\  ");
    println!("    ( {eye}.{eye} ) ",eye=eye.red().bold());
    println!("     > ^ <  ");
    println!("    /     \\ ");
    println!("   /       \\");
    println!("   \\       /");
    println!("    \\_____/ ");
    println!("     /   \\  ");
    println!("    /     \\ ");

}