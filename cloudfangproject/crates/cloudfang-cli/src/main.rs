//! CloudFang CLI — The main entry point.

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "cloudfang",
    version,
    about = "☁️🐺 CloudFang — OpenStack SysOps Agent",
    long_about = "CloudFang is an autonomous SysOps agent for OpenStack cloud infrastructure.\n\
                  Inspired by OpenFang. Built in Rust."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize CloudFang configuration
    Init,

    /// Start the autonomous daemon (all Hands active)
    Start,

    /// Show system status overview
    Status,

    /// Manage autonomous Hands
    Hand {
        #[command(subcommand)]
        action: HandAction,
    },

    /// Direct OpenStack operations
    Ops {
        #[command(subcommand)]
        action: OpsAction,
    },

    /// View incident history
    Incidents {
        /// Number of incidents to show
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    /// Interactive chat with the SysOps agent
    Chat,
}

#[derive(Subcommand)]
enum HandAction {
    /// List all available Hands
    List,
    /// Activate a Hand
    Activate {
        /// Hand name: monitor, remediate, backup, scale
        name: String,
    },
    /// Pause a Hand
    Pause { name: String },
    /// Show Hand status
    Status {
        /// Optional: specific Hand name
        name: Option<String>,
    },
    /// Run a Hand once (for testing)
    Run { name: String },
}

#[derive(Subcommand)]
enum OpsAction {
    /// VM operations
    Vm {
        #[command(subcommand)]
        action: VmAction,
    },
    /// Network operations
    Network {
        #[command(subcommand)]
        action: NetworkAction,
    },
    /// Volume operations
    Volume {
        #[command(subcommand)]
        action: VolumeAction,
    },
}

#[derive(Subcommand)]
enum VmAction {
    /// List all VMs
    List,
    /// Show VM details
    Show { id: String },
    /// Reboot a VM
    Reboot {
        id: String,
        /// Hard reboot
        #[arg(long)]
        hard: bool,
    },
    /// Start a stopped VM
    Start { id: String },
    /// Stop a running VM
    Stop { id: String },
    /// Get VM console log
    Log {
        id: String,
        #[arg(short, long, default_value = "50")]
        lines: u32,
    },
}

#[derive(Subcommand)]
enum NetworkAction {
    /// List all networks
    List,
    /// List floating IPs
    FloatingIps,
}

#[derive(Subcommand)]
enum VolumeAction {
    /// List all volumes
    List,
    /// Create a snapshot
    Snapshot {
        /// Volume ID to snapshot
        volume_id: String,
        /// Snapshot name
        #[arg(short, long)]
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env if exists
    dotenvy::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init => cmd_init().await?,
        Commands::Start => cmd_start().await?,
        Commands::Status => cmd_status().await?,
        Commands::Hand { action } => cmd_hand(action).await?,
        Commands::Ops { action } => cmd_ops(action).await?,
        Commands::Incidents { limit } => cmd_incidents(limit).await?,
        Commands::Chat => cmd_chat().await?,
    }

    Ok(())
}

async fn cmd_start() -> Result<()> {
    println!("☁️🐺 CloudFang Daemon Starting...");
    
    let config_path = std::path::Path::new("cloudfang.toml");
    let config = cloudfang_core::config::CloudFangConfig::load(config_path)?;
    let mut session = cloudfang_ops::OpenStackSession::new(config.to_credentials()).await?;
    let store = cloudfang_store::Store::open(std::path::Path::new("cloudfang.db"))?;

    let mut hands: Vec<Box<dyn cloudfang_hands::Hand>> = vec![
        Box::new(cloudfang_hands::monitor::MonitorHand::new()),
        Box::new(cloudfang_hands::remediate::RemediateHand::new()),
        Box::new(cloudfang_hands::backup::BackupHand::new()),
        Box::new(cloudfang_hands::scale::ScaleHand::new()),
    ];

    // Activate all hands for daemon mode
    for hand in hands.iter_mut() {
        hand.activate();
    }

    println!("✅ Daemon active. Press Ctrl+C to stop.");
    
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
    
    loop {
        interval.tick().await;
        
        for hand in hands.iter_mut() {
            if let Err(e) = hand.execute(&mut session, store.clone()).await {
                tracing::error!("Error executing hand {}: {}", hand.name(), e);
            }
        }
    }
}

async fn cmd_init() -> Result<()> {
    println!("☁️🐺 CloudFang — Initialization");
    println!("================================");

    let config_path = std::path::Path::new("cloudfang.toml");
    if config_path.exists() {
        println!("✅ cloudfang.toml already exists");
    } else {
        let template = include_str!("../../../cloudfang.toml.example");
        std::fs::write(config_path, template)?;
        println!("📝 Created cloudfang.toml — please edit it with your OpenStack credentials");
    }

    // Test database
    let _store = cloudfang_store::Store::open(std::path::Path::new("cloudfang.db"))?;
    println!("📦 Database initialized: cloudfang.db");

    println!("\n🚀 Next steps:");
    println!("  1. Edit cloudfang.toml with your OpenStack credentials");
    println!("  2. Run: cloudfang status");
    println!("  3. Run: cloudfang hand activate monitor");

    Ok(())
}

async fn cmd_status() -> Result<()> {
    println!("☁️🐺 CloudFang Status");
    println!("=====================");

    let config_path = std::path::Path::new("cloudfang.toml");
    if !config_path.exists() {
        println!("⚠️  No cloudfang.toml found. Run: cloudfang init");
        return Ok(());
    }

    let config = cloudfang_core::config::CloudFangConfig::load(config_path)?;
    println!("📋 Config loaded");
    println!("   OpenStack: {}", config.openstack.auth_url);
    println!("   Project:   {}", config.openstack.project_name);
    println!(
        "   LLM:       {} ({})",
        config.llm.provider, config.llm.model
    );

    // Show Hand statuses
    println!("\n🤖 Hands:");
    let hands: Vec<(&str, &str)> = vec![
        ("monitor", "Monitors VM health, disk, network"),
        ("remediate", "Auto-fixes common issues"),
        ("backup", "Snapshots VMs/volumes on schedule"),
        ("scale", "Auto-scales based on load"),
    ];
    for (name, desc) in hands {
        println!("   ⚫ {:<12} — {}", name, desc);
    }

    Ok(())
}

async fn cmd_hand(action: HandAction) -> Result<()> {
    use cloudfang_hands::Hand;

    let mut hands: Vec<Box<dyn Hand>> = vec![
        Box::new(cloudfang_hands::monitor::MonitorHand::new()),
        Box::new(cloudfang_hands::remediate::RemediateHand::new()),
        Box::new(cloudfang_hands::backup::BackupHand::new()),
        Box::new(cloudfang_hands::scale::ScaleHand::new()),
    ];

    match action {
        HandAction::List => {
            println!("🤖 Available Hands:");
            for hand in &hands {
                println!(
                    "   {} {:<12} — {}",
                    hand.state(),
                    hand.name(),
                    hand.description()
                );
            }
        }
        HandAction::Activate { name } => {
            if let Some(hand) = hands.iter_mut().find(|h| h.name() == name) {
                hand.activate();
                println!("✅ Activated Hand: {}", name);
            } else {
                println!(
                    "❌ Unknown Hand: {}. Available: monitor, remediate, backup, scale",
                    name
                );
            }
        }
        HandAction::Pause { name } => {
            if let Some(hand) = hands.iter_mut().find(|h| h.name() == name) {
                hand.pause();
                println!("⏸️  Paused Hand: {}", name);
            } else {
                println!("❌ Unknown Hand: {}", name);
            }
        }
        HandAction::Status { name } => {
            if let Some(name) = name {
                if let Some(hand) = hands.iter().find(|h| h.name() == name) {
                    println!("{}: {}", hand.name(), hand.state());
                } else {
                    println!("❌ Unknown Hand: {}", name);
                }
            } else {
                for hand in &hands {
                    println!("   {} {}", hand.state(), hand.name());
                }
            }
        }
        HandAction::Run { name } => {
            if let Some(hand) = hands.iter_mut().find(|h| h.name() == name) {
                let config_path = std::path::Path::new("cloudfang.toml");
                if !config_path.exists() {
                    anyhow::bail!("No cloudfang.toml found. Run 'cloudfang init' first.");
                }
                let config = cloudfang_core::config::CloudFangConfig::load(config_path)?;
                let mut session = cloudfang_ops::OpenStackSession::new(config.to_credentials()).await?;
                let store = cloudfang_store::Store::open(std::path::Path::new("cloudfang.db"))?;

                hand.activate();
                println!("🔄 Running Hand: {} ...", name);
                let report = hand.execute(&mut session, store).await?;
                println!("📊 Report:");
                println!("   Summary: {}", report.summary);
                println!("   Issues found: {}", report.issues_found);
                println!("   Issues resolved: {}", report.issues_resolved);
                for action in &report.actions_taken {
                    println!("   ✔ {}", action);
                }
            } else {
                println!("❌ Unknown Hand: {}", name);
            }
        }
    }

    Ok(())
}

async fn cmd_ops(action: OpsAction) -> Result<()> {
    let config_path = std::path::Path::new("cloudfang.toml");
    if !config_path.exists() {
        println!("⚠️  No cloudfang.toml found. Run: cloudfang init");
        return Ok(());
    }

    let config = cloudfang_core::config::CloudFangConfig::load(config_path)?;
    let creds = config.to_credentials();
    let mut session = cloudfang_ops::OpenStackSession::new(creds).await?;

    match action {
        OpsAction::Vm { action } => match action {
            VmAction::List => {
                let servers = cloudfang_ops::nova::list_servers(&mut session).await?;
                println!("🖥️  VMs ({} total):", servers.len());
                println!("   {:<38} {:<20} {:<10}", "ID", "NAME", "STATUS");
                println!("   {}", "-".repeat(70));
                for s in &servers {
                    println!("   {:<38} {:<20} {:<10}", s.id, s.name, s.status);
                }
            }
            VmAction::Show { id } => {
                let s = cloudfang_ops::nova::get_server(&mut session, &id).await?;
                println!("🖥️  VM Details:");
                println!("   ID:       {}", s.id);
                println!("   Name:     {}", s.name);
                println!("   Status:   {}", s.status);
                if let Some(host) = &s.host {
                    println!("   Host:     {}", host);
                }
            }
            VmAction::Reboot { id, hard } => {
                let rt = if hard {
                    cloudfang_ops::nova::RebootType::Hard
                } else {
                    cloudfang_ops::nova::RebootType::Soft
                };
                cloudfang_ops::nova::server_action(
                    &mut session,
                    &id,
                    cloudfang_ops::nova::ServerAction::Reboot(rt),
                )
                .await?;
                println!("✅ Rebooted VM {}", id);
            }
            VmAction::Start { id } => {
                cloudfang_ops::nova::server_action(
                    &mut session,
                    &id,
                    cloudfang_ops::nova::ServerAction::Start,
                )
                .await?;
                println!("✅ Started VM {}", id);
            }
            VmAction::Stop { id } => {
                cloudfang_ops::nova::server_action(
                    &mut session,
                    &id,
                    cloudfang_ops::nova::ServerAction::Stop,
                )
                .await?;
                println!("✅ Stopped VM {}", id);
            }
            VmAction::Log { id, lines } => {
                let log =
                    cloudfang_ops::nova::get_console_log(&mut session, &id, Some(lines)).await?;
                println!("📜 Console Log (last {} lines):\n{}", lines, log);
            }
        },
        OpsAction::Network { action } => match action {
            NetworkAction::List => {
                let nets = cloudfang_ops::neutron::list_networks(&mut session).await?;
                println!("🌐 Networks ({} total):", nets.len());
                for n in &nets {
                    println!("   {} — {} ({})", n.id, n.name, n.status);
                }
            }
            NetworkAction::FloatingIps => {
                let fips = cloudfang_ops::neutron::list_floating_ips(&mut session).await?;
                println!("🌐 Floating IPs ({} total):", fips.len());
                for f in &fips {
                    println!("   {} — {} ({})", f.id, f.floating_ip_address, f.status);
                }
            }
        },
        OpsAction::Volume { action } => match action {
            VolumeAction::List => {
                let vols = cloudfang_ops::cinder::list_volumes(&mut session).await?;
                println!("💾 Volumes ({} total):", vols.len());
                for v in &vols {
                    let name = v.name.as_deref().unwrap_or("(unnamed)");
                    println!("   {} — {} ({}GB, {})", v.id, name, v.size, v.status);
                }
            }
            VolumeAction::Snapshot { volume_id, name } => {
                let snap =
                    cloudfang_ops::cinder::create_snapshot(&mut session, &volume_id, &name, None)
                        .await?;
                println!("✅ Created snapshot: {} ({})", snap.id, snap.status);
            }
        },
    }

    Ok(())
}

async fn cmd_incidents(limit: usize) -> Result<()> {
    let store = cloudfang_store::Store::open(std::path::Path::new("cloudfang.db"))?;
    let incidents = store.list_incidents(limit)?;

    if incidents.is_empty() {
        println!("📋 No incidents recorded yet.");
        return Ok(());
    }

    println!("📋 Recent Incidents ({}):", incidents.len());
    for inc in &incidents {
        let resolved = if inc.resolved { "✅" } else { "⏳" };
        println!(
            "   {} {} [{}] {} — {}",
            inc.severity_emoji(),
            resolved,
            &inc.timestamp[..19],
            inc.resource_name.as_deref().unwrap_or(&inc.resource_id),
            inc.description
        );
    }

    Ok(())
}

async fn cmd_chat() -> Result<()> {
    println!("☁️🐺 CloudFang SysOps Chat");
    println!("=========================");
    println!("Ask me anything about your OpenStack infrastructure.");
    println!("Type 'quit' to exit.\n");

    let config_path = std::path::Path::new("cloudfang.toml");
    if !config_path.exists() {
        println!("⚠️  No cloudfang.toml found. Run: cloudfang init");
        return Ok(());
    }

    let config = cloudfang_core::config::CloudFangConfig::load(config_path)?;

    let llm_config = cloudfang_core::config::LlmConfig {
        provider: config.llm.provider.clone(),
        api_key: config.llm.api_key.clone(),
        model: config.llm.model.clone(),
        base_url: config.llm.base_url.clone(),
    };

    let system_prompt =
        "You are CloudFang, an intelligent SysOps agent for OpenStack cloud infrastructure. \
        You help users monitor, manage, and troubleshoot their OpenStack environment. \
        You can list VMs, check network status, manage volumes, and analyze system health. \
        Be concise and actionable in your responses.";

    let llm = cloudfang_core::llm::LlmClient::new(llm_config, system_prompt);
    let mut tools = cloudfang_core::tools::ToolRegistry::new();
    
    // Register real-world tools
    tools.register(Box::new(cloudfang_core::ops_tools::ListServersTool));
    tools.register(Box::new(cloudfang_core::ops_tools::ServerActionTool));

    let agent = cloudfang_core::agent::Agent::new(llm, tools);

    let creds = config.to_credentials();
    let mut session = cloudfang_ops::OpenStackSession::new(creds).await?;

    loop {
        print!("> ");
        use std::io::Write;
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("👋 Goodbye!");
            break;
        }

        if input.is_empty() {
            continue;
        }

        match agent.process(&mut session, input).await {
            Ok(response) => println!("🐺: {}\n", response),
            Err(e) => println!("❌ Error: {}\n", e),
        }
    }

    Ok(())
}
