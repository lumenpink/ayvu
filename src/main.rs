use clap::{Parser, Subcommand};
use std::io::Read;

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
        /// Strip layout (horizontal, grid, vertical)
        #[arg(short, long)]
        layout: Option<String>,
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
        Some(Commands::Render {
            input,
            output,
            title,
            layout,
        }) => {
            let content = match input {
                Some(ref path) if path != "-" => match std::fs::read_to_string(path) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Error reading input file '{}': {}", path, e);
                        std::process::exit(1);
                    }
                },
                _ => {
                    let mut buffer = String::new();
                    if let Err(e) = std::io::stdin().read_to_string(&mut buffer) {
                        eprintln!("Error reading from stdin: {}", e);
                        std::process::exit(1);
                    }
                    buffer
                }
            };

            let mut strip = match ayvu::parser::parse_dialogue_script(&content, title.as_deref()) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error parsing dialogue script: {}", e);
                    std::process::exit(1);
                }
            };

            if let Some(ref l) = layout {
                strip.layout = ayvu::model::StripLayout::from_str_loose(l);
            }

            let svg = ayvu::renderer::render_comic_strip_to_svg(&strip);
            if let Err(e) = std::fs::write(&output, svg) {
                eprintln!("Error writing output file '{}': {}", output, e);
                std::process::exit(1);
            }
            println!("✓ Comic strip successfully rendered to {}", output);
        }
        Some(Commands::Serve { host, port }) => {
            if let Err(e) = ayvu::server::run_server(&host, port) {
                eprintln!("HTTP server error: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            println!(
                "Ayvu v{} - Declarative Comic Strip Generator",
                ayvu::VERSION
            );
            println!("Use --help for available commands.");
        }
    }
}
