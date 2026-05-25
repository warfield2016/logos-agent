//! `logos-agent` — one-command deploy and control plane for the agent module.
//!
//! ```text
//! logos-agent quickstart                   # first-time wizard (5 min target)
//! logos-agent deploy --remote <ssh-addr>   # ship to a remote Logos node
//! logos-agent configure spending.per_tx_lez 100
//! logos-agent invoke wallet.balance
//! logos-agent chat                         # owner TUI over Logos Messaging
//! logos-agent status
//! ```

use agent_core::runtime::Runtime;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "logos-agent", version, about = "Logos Autonomous AI Agent")]
struct Cli {
    /// Path to the agent config file.
    #[arg(
        long,
        env = "LOGOS_AGENT_CONFIG",
        default_value = "~/.logos-agent/config.toml"
    )]
    config: String,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// First-time interactive setup wizard.
    Quickstart,

    /// Deploy this agent to a remote machine running Logos Core headless.
    Deploy {
        #[arg(long)]
        remote: String,
    },

    /// Set a config key (e.g. `spending.per_tx_lez = 100`).
    Configure { key: String, value: String },

    /// Invoke a skill by name with JSON params.
    Invoke {
        skill: String,
        #[arg(default_value = "{}")]
        params_json: String,
    },

    /// List all registered skills.
    Skills,

    /// Print agent status (balance, pending approvals, active tasks).
    Status,

    /// Open the encrypted owner channel (TUI chat).
    Chat,

    /// Print the agent's AgentCard as JSON.
    Card,

    /// Reload the agent after config or skill changes.
    Reload,

    /// Generate an outline LP-0008 spec-compliant Agent Registry contract.
    InitRegistry {
        #[arg(long, default_value = "programs/agent-registry")]
        out: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    Runtime::init_global()?;

    match cli.cmd {
        Cmd::Quickstart => quickstart(),
        Cmd::Deploy { remote } => deploy_remote(&remote),
        Cmd::Configure { key, value } => configure(&cli.config, &key, &value),
        Cmd::Invoke { skill, params_json } => invoke(&cli.config, &skill, &params_json),
        Cmd::Skills => list_skills(&cli.config),
        Cmd::Status => status(&cli.config),
        Cmd::Chat => chat(&cli.config),
        Cmd::Card => print_card(&cli.config),
        Cmd::Reload => reload(&cli.config),
        Cmd::InitRegistry { out } => init_registry(out),
    }
}

fn quickstart() -> anyhow::Result<()> {
    println!("Logos Agent quickstart — target: 5 minutes to first deployed agent.");
    println!("Steps:");
    println!("  1. Generate identity (NPK/ISK keypair)        [TODO Week-2]");
    println!("  2. Claim faucet on LEZ devnet                  [TODO Week-2]");
    println!("  3. Configure owner NPK                         [TODO Week-2]");
    println!("  4. Set initial spending policy                 [TODO Week-2]");
    println!("  5. Publish AgentCard on discovery topic        [TODO Week-3]");
    Ok(())
}

fn deploy_remote(remote: &str) -> anyhow::Result<()> {
    println!("Deploy to {remote} — pending Week-2 (rsync + remote start)");
    Ok(())
}

fn configure(config: &str, key: &str, value: &str) -> anyhow::Result<()> {
    println!("[stub] configure({config}): {key} = {value}");
    Ok(())
}

fn invoke(config: &str, skill: &str, params_json: &str) -> anyhow::Result<()> {
    let agent = Runtime::create_agent(config)?;
    match agent.invoke_blocking(skill, params_json) {
        Ok(json) => {
            println!("{json}");
            Ok(())
        }
        Err(e) => {
            eprintln!("skill error: {e}");
            std::process::exit(1);
        }
    }
}

fn list_skills(config: &str) -> anyhow::Result<()> {
    let agent = Runtime::create_agent(config)?;
    let manifests = agent.list_skills();
    println!("{}", serde_json::to_string_pretty(&manifests)?);
    Ok(())
}

fn status(config: &str) -> anyhow::Result<()> {
    let agent = Runtime::create_agent(config)?;
    let status = agent.status_blocking();
    println!("{}", serde_json::to_string_pretty(&status)?);
    Ok(())
}

fn chat(_config: &str) -> anyhow::Result<()> {
    println!("[stub] owner-channel TUI — pending Week-2");
    Ok(())
}

fn print_card(config: &str) -> anyhow::Result<()> {
    let agent = Runtime::create_agent(config)?;
    let card_json = agent.invoke_blocking("agent.card", "{}")?;
    println!("{card_json}");
    Ok(())
}

fn reload(_config: &str) -> anyhow::Result<()> {
    println!("[stub] reload — pending Week-3");
    Ok(())
}

fn init_registry(out: PathBuf) -> anyhow::Result<()> {
    println!("[stub] init_registry at {out:?} — pending Week-3 LEZ program scaffolding");
    Ok(())
}
