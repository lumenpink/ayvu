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
    /// Configuration management (OpenDesktop XDG)
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
    /// Check for available updates on Banca
    CheckUpdate {
        /// Distribution channel override
        #[arg(long)]
        channel: Option<String>,
    },
    /// Download and atomically install update from Banca
    Update {
        /// Distribution channel override
        #[arg(long)]
        channel: Option<String>,
        /// Force update even if version is pinned
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigCommands {
    /// Show current resolved configuration
    Show,
    /// Get current configuration in TOML format
    Get,
    /// Pin to a specific version or current version
    Pin {
        /// Version string to pin (default: current version)
        version: Option<String>,
    },
    /// Unpin version, allowing updates
    Unpin,
    /// Set distribution channel (stable, nightly)
    SetChannel {
        /// Channel name
        channel: String,
    },
    /// Enable or disable automatic update checks
    SetAutoCheck {
        /// Enable (true) or disable (false)
        enabled: bool,
    },
    /// Enable or disable automatic update application
    SetAutoUpdate {
        /// Enable (true) or disable (false)
        enabled: bool,
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
            #[cfg(not(target_arch = "wasm32"))]
            ayvu::updater::check_startup_notice(ayvu::VERSION);

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
        Some(Commands::Config { action }) => match action {
            ConfigCommands::Show => {
                let cfg = ayvu::config::Config::load();
                let cfg_path = ayvu::config::get_config_path();
                let state_path = ayvu::config::get_state_path();
                println!("Config file: {}", cfg_path.display());
                println!("State file:  {}", state_path.display());
                println!("Channel:     {}", cfg.channel);
                println!("Pinned:      {}", cfg.pinned);
                println!(
                    "Pin:         {}",
                    if cfg.pin.is_empty() {
                        "(none)"
                    } else {
                        &cfg.pin
                    }
                );
                println!("Auto-check:  {}", cfg.auto_check);
                println!("Interval:    {}h", cfg.check_interval_hours);
                println!("Auto-update: {}", cfg.auto_update);
            }
            ConfigCommands::Get => {
                let cfg = ayvu::config::Config::load();
                print!("{}", cfg.to_toml());
            }
            ConfigCommands::Pin { version } => {
                let mut cfg = ayvu::config::Config::load();
                cfg.pin_version(version);
                if let Err(e) = cfg.save() {
                    eprintln!("Erro ao salvar configuração: {}", e);
                    std::process::exit(1);
                }
                println!("Versão pinada em: {}", cfg.pin);
            }
            ConfigCommands::Unpin => {
                let mut cfg = ayvu::config::Config::load();
                cfg.unpin_version();
                if let Err(e) = cfg.save() {
                    eprintln!("Erro ao salvar configuração: {}", e);
                    std::process::exit(1);
                }
                println!("Pinagem de versão removida.");
            }
            ConfigCommands::SetChannel { channel } => {
                let mut cfg = ayvu::config::Config::load();
                cfg.set_channel(&channel);
                if let Err(e) = cfg.save() {
                    eprintln!("Erro ao salvar configuração: {}", e);
                    std::process::exit(1);
                }
                println!("Canal alterado para: {}", cfg.channel);
            }
            ConfigCommands::SetAutoCheck { enabled } => {
                let mut cfg = ayvu::config::Config::load();
                cfg.set_auto_check(enabled);
                if let Err(e) = cfg.save() {
                    eprintln!("Erro ao salvar configuração: {}", e);
                    std::process::exit(1);
                }
                println!("Auto-check alterado para: {}", cfg.auto_check);
            }
            ConfigCommands::SetAutoUpdate { enabled } => {
                let mut cfg = ayvu::config::Config::load();
                cfg.set_auto_update(enabled);
                if let Err(e) = cfg.save() {
                    eprintln!("Erro ao salvar configuração: {}", e);
                    std::process::exit(1);
                }
                println!("Auto-update alterado para: {}", cfg.auto_update);
            }
        },
        Some(Commands::CheckUpdate { channel }) => {
            let resolved_channel = ayvu::config::resolve_channel(channel.as_deref());
            println!("Canal ativo: {}", resolved_channel);
            println!("Versão atual: {}", ayvu::VERSION);
            match ayvu::updater::check_banca_manifest(&resolved_channel) {
                Ok(Some(latest)) => {
                    println!("Versão na Banca: {}", latest);
                    if latest != ayvu::VERSION {
                        println!("Nova versão disponível! Execute 'ayvu update' para atualizar.");
                    } else {
                        println!("O ayvu já está na versão mais recente do canal.");
                    }
                }
                Ok(None) => {
                    println!(
                        "Nenhuma versão encontrada para o canal '{}' na Banca.",
                        resolved_channel
                    );
                }
                Err(e) => {
                    eprintln!("Erro ao consultar a Banca: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Update { channel, force }) => {
            let cfg = ayvu::config::Config::load();
            if cfg.pinned && !force {
                eprintln!(
                    "Aviso: A versão está pinada em '{}'. Use --force para atualizar.",
                    cfg.pin
                );
                std::process::exit(1);
            }
            let resolved_channel = ayvu::config::resolve_channel(channel.as_deref());
            println!(
                "Verificando atualizações no canal '{}'...",
                resolved_channel
            );
            match ayvu::updater::check_banca_manifest(&resolved_channel) {
                Ok(Some(latest)) => {
                    if latest == ayvu::VERSION && !force {
                        println!("O ayvu já está atualizado ({}).", latest);
                        return;
                    }
                    println!("Atualizando para {}...", latest);
                    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                    let default_bin = std::path::PathBuf::from(home)
                        .join(".local")
                        .join("bin")
                        .join("ayvu");
                    let bin_path = std::env::current_exe().unwrap_or(default_bin);
                    println!(
                        "Binário em {} atualizado com sucesso para {}.",
                        bin_path.display(),
                        latest
                    );
                }
                Ok(None) => {
                    println!("Nenhuma versão disponível no canal '{}'.", resolved_channel);
                }
                Err(e) => {
                    eprintln!("Erro ao verificar atualização: {}", e);
                    std::process::exit(1);
                }
            }
        }
        None => {
            #[cfg(not(target_arch = "wasm32"))]
            ayvu::updater::check_startup_notice(ayvu::VERSION);

            println!(
                "Ayvu v{} - Declarative Comic Strip Generator",
                ayvu::VERSION
            );
            println!("Use --help for available commands.");
        }
    }
}
