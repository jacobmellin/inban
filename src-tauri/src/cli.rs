extern crate diesel;
extern crate dotenv;

use clap::{Parser, Subcommand};

mod db;

#[derive(Parser)]
#[clap(name = "Inban CLI")]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    GetBooks,
    AddBook {
        book_name: String
    }
}


fn main() {
    let args = Cli::parse();

    match args.command {
         SubCommand::GetBooks => {
            println!("Here are the books that I know:\n");

            let books = db::get_books();
            for book in books {
                dbg!("{}: {}", book.id, book.name);
            }
        },
        SubCommand::AddBook { book_name } => {
            db::add_book(&book_name).expect("Failed to add book:");
        }
    }
}
