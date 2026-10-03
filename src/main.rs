use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ayvu")]
#[command(author = "Lumen Pink <hi@lumen.pink>")]
#[command(version = ayvu::VERSION)]
#[command(about = "Declarative comic strip generator inspired by Microsoft Comic Chat", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Render conversational text into a comic strip
    Render {
        /// Input file with dialogue markup (reads stdin if omitted)
        input: Option<String>,
        /// Output file (SVG format)
        #[arg(short, long, default_value = "strip.svg")]
        output: String,
        /// Strip title
        #[arg(short, long)]
        title: Option<String>,
    },
    /// Start HTTP microservice daemon (Joaju-style API)
    Serve {
        /// Host to bind
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// Port to listen on
        #[arg(short, long, default_value_t = 3100)]
        port: u16,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Render { input, output, title }) => {
            println!("Rendering comic to {} (input: {:?}, title: {:?})", output, input, title);
        }
        Some(Commands::Serve { host, port }) => {
            println!("Ayvu HTTP daemon listening on http://{}:{}", host, port);
        }
        None => {
            println!("Ayvu v{} - Declarative Comic Strip Generator", ayvu::VERSION);
            println!("Use --help for available commands.");
        }
    }
}
