<div align="center">

# ☁️ CloudFang

**Autonomous OpenStack SysOps Agent — Built with Rust**

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen?style=flat-square)]()
[![OpenStack](https://img.shields.io/badge/OpenStack-Compatible-red?style=flat-square&logo=openstack)](https://www.openstack.org/)

*A lightweight, LLM-powered SysOps agent that monitors, heals, backs up, and scales your OpenStack cloud infrastructure — autonomously.*

</div>

---

## 🌟 Overview

**CloudFang** is an autonomous AI agent built in Rust, designed specifically for **OpenStack cloud infrastructure operations**. Inspired by the OpenFang agentic framework but stripped down to its most powerful essentials, CloudFang gives your cloud a brain.

Instead of manually watching dashboards and reacting to incidents, CloudFang's **SysOps Hands** work in the background — watching, thinking, and acting on your behalf.

```
                     ┌─────────────────────────┐
                     │      cloudfang-cli       │
                     │    (CLI + TUI Dashboard) │
                     └────────────┬────────────┘
                                  │
                     ┌────────────▼────────────┐
                     │      cloudfang-core      │
                     │  Agent Runtime + LLM +   │
                     │  Scheduler + Tools       │
                     └──┬──────────────────┬───┘
                        │                  │
           ┌────────────▼───┐    ┌─────────▼──────────┐
           │ cloudfang-ops  │    │  cloudfang-hands    │
           │ OpenStack APIs │    │  Monitor / Remediate│
           │ Nova, Neutron  │    │  Backup / Scale     │
           │ Cinder, Glance │    └─────────┬───────────┘
           └────────────────┘             │
                        │                 │
                     ┌──▼─────────────────▼──┐
                     │    cloudfang-store     │
                     │  SQLite + Audit Log    │
                     └───────────────────────┘
```

---

## ✨ Key Features

| Feature | Description |
|---|---|
| 🤖 **LLM-Powered Decisions** | Uses GPT-4o / Ollama to analyze metrics and decide on actions |
| 👁️ **Autonomous Monitoring** | Every 5 minutes: checks VM health, disk, network, service status |
| 🔧 **Self-Healing Remediation** | Detects failures and auto-restarts VMs, clears disk, reconnects network |
| 💾 **Scheduled Backups** | Daily snapshots of VMs & volumes at 2 AM with automatic cleanup |
| 📈 **Intelligent Scaling** | Analyzes load every 15 minutes and recommends or executes scale up/down |
| 🗃️ **Full Audit Trail** | Every agent action is logged to SQLite with timestamps and outcomes |
| 💬 **Conversational Interface** | Ask `cloudfang chat` natural language questions about your infrastructure |

---

## 🏗️ Architecture — 5-Crate Workspace

```
cloudfangproject/
├── Cargo.toml                    # Workspace root
├── cloudfang.toml                # Runtime configuration
├── cloudfang.toml.example        # Configuration template
└── crates/
    ├── cloudfang-core/           # 🧠 Agent runtime, LLM client, scheduler
    │   └── src/
    │       ├── agent.rs          # Agent loop: task → LLM → tool → action
    │       ├── config.rs         # Config loader (cloudfang.toml)
    │       ├── llm.rs            # LLM client (OpenAI-compatible + Ollama)
    │       ├── scheduler.rs      # Cron-based Hand scheduler
    │       └── tools.rs          # Tool registry for LLM function-calling
    ├── cloudfang-ops/            # ☁️ OpenStack API clients
    │   └── src/
    │       ├── keystone.rs       # Auth (token, projects, users)
    │       ├── nova.rs           # Compute (VMs, reboot, migrate, console)
    │       ├── neutron.rs        # Network (networks, subnets, floating IPs)
    │       ├── cinder.rs         # Block Storage (volumes, snapshots)
    │       ├── glance.rs         # Image service
    │       ├── heat.rs           # Orchestration (stacks)
    │       └── metrics.rs        # Metrics (Ceilometer/Gnocchi)
    ├── cloudfang-hands/          # 🤲 Autonomous SysOps Hands
    │   └── src/
    │       ├── monitor.rs        # Monitor Hand (every 5 min)
    │       ├── remediate.rs      # Remediate Hand (event-driven)
    │       ├── backup.rs         # Backup Hand (daily 2 AM)
    │       └── scale.rs          # Scale Hand (every 15 min)
    ├── cloudfang-store/          # 🗄️ SQLite persistence & audit log
    └── cloudfang-cli/            # 💻 CLI interface & TUI dashboard
```

---

## 🤲 The Four Hands

CloudFang's autonomous operations are powered by four **Hands** — AI agents that run on independent schedules:

### 👁️ Monitor Hand — Every 5 Minutes
Continuously checks the health of your whole cluster:
- VM status (running, error, paused)
- Disk usage across all volumes
- Network latency & floating IP reachability
- OpenStack service health (Nova, Neutron, Cinder...)

### 🔧 Remediate Hand — Event-Driven
When Monitor detects an issue, Remediate kicks in automatically:
- Restart failed/error-state VMs
- Clear disk space on over-utilized volumes
- Reconnect broken network ports
- Escalate to alert if auto-fix fails

### 💾 Backup Hand — Daily at 2:00 AM
Runs a nightly snapshot cycle:
- Snapshot all VMs and volumes marked as critical
- Rotate and clean up snapshots older than retention policy
- Log all snapshot operations to audit trail

### 📈 Scale Hand — Every 15 Minutes
Analyzes load trends and acts on them:
- Pull CPU, memory, disk, and network metrics via Gnocchi/Ceilometer
- Send to LLM for analysis and recommendation
- Suggest or execute scale-up/scale-down actions

---

## 🚀 Getting Started

### Prerequisites

- **Rust** `1.75+` — [Install via rustup](https://rustup.rs/)
- **An OpenStack cluster** (or mock mode for development)
- **An LLM API Key**: OpenAI API key, *or* a local [Ollama](https://ollama.ai/) instance

### 1. Clone the Repository

```bash
git clone https://github.com/vulekaisar/cloudfang.git
cd cloudfang
```

### 2. Configure CloudFang

Copy the example config and fill in your credentials:

```bash
cp cloudfang.toml.example cloudfang.toml
```

Edit `cloudfang.toml`:

```toml
[openstack]
auth_url     = "http://your-openstack-host:5000/v3"
username     = "admin"
password     = "your-password"
project_name = "admin"
domain_name  = "Default"

[llm]
provider = "openai"               # or "ollama" for local LLM
api_key  = "sk-..."               # Leave empty for Ollama
model    = "gpt-4o-mini"          # or "llama3" for Ollama
base_url = "https://api.openai.com/v1"

[store]
db_path = "cloudfang.db"

[hands]
monitor_interval_secs      = 300   # 5 minutes
backup_cron                = "0 2 * * *"
scale_check_interval_secs  = 900   # 15 minutes
```

### 3. Build the Project

```bash
# Build all 5 crates
cargo build --workspace

# Or build optimized release binary
cargo build --release --workspace
```

### 4. Run CloudFang

```bash
# Initialize system check
cloudfang init

# Start the daemon (all Hands active)
cloudfang start

# Check system overview
cloudfang status
```

---

## 💻 CLI Reference

```bash
# System Management
cloudfang init                        # Verify config & connectivity
cloudfang start                       # Start daemon (all Hands)
cloudfang status                      # System overview

# Hand Control
cloudfang hand status                 # Check all Hand statuses
cloudfang hand activate monitor       # Activate the Monitor Hand
cloudfang hand activate backup        # Trigger a backup cycle now
cloudfang hand activate scale         # Run a scale check now

# Direct OpenStack Operations
cloudfang ops vm list                 # List all VMs
cloudfang ops vm reboot <id>          # Reboot a specific VM
cloudfang ops volume list             # List all volumes
cloudfang ops network list            # List all networks

# AI Chat Interface
cloudfang chat                        # Interactive natural language mode

# History & Audit
cloudfang incidents                   # View incident history
cloudfang audit                       # View full audit trail
```

### Example Chat Session

```
$ cloudfang chat

🤖 CloudFang Agent ready. Ask me anything about your cloud.

> Which VMs are using more than 90% CPU?
Thinking...
🤖 Found 3 VMs with CPU usage > 90%:
   - vm-prod-api-03 (94.2%) — Running, Project: production
   - vm-worker-07   (91.8%) — Running, Project: batch-jobs
   - vm-db-replica  (90.1%) — Running, Project: database

   Recommendation: Consider scaling vm-prod-api-03 or migrating to a larger flavor.

> Snapshot all production VMs right now
Thinking...
🤖 Starting snapshot cycle for project "production"...
   ✅ vm-prod-api-01 — snapshot created: snap-a1b2c3
   ✅ vm-prod-api-02 — snapshot created: snap-d4e5f6
   ✅ vm-prod-api-03 — snapshot created: snap-g7h8i9
   Done. 3 snapshots created in 47s.
```

---

## 🛠️ Tech Stack

| Component | Technology |
|---|---|
| **Language** | Rust 2021 Edition |
| **Async Runtime** | Tokio |
| **LLM Client** | `async-openai` + Rig framework |
| **HTTP Client** | Reqwest (rustls-tls) |
| **Serialization** | Serde + serde_json + TOML |
| **Database** | SQLite via `rusqlite` (bundled) |
| **CLI** | Clap v4 (derive API) |
| **Scheduler** | Cron expression parser |
| **Error Handling** | `anyhow` + `thiserror` |
| **Logging** | `tracing` + `tracing-subscriber` |

---

## 🗺️ Development Roadmap

| Phase | Contents | Status |
|---|---|---|
| **Phase 1** | Workspace setup + `cloudfang-ops` (Keystone auth + Nova) + CLI skeleton | ✅ Done |
| **Phase 2** | `cloudfang-core` (agent loop + LLM client + tool registry) | ✅ Done |
| **Phase 3** | `cloudfang-hands` (all 4 Hands) + `cloudfang-store` (SQLite) | 🚧 In Progress |
| **Phase 4** | TUI dashboard + multi-cloud support + streaming alerts | 🔮 Planned |

---

## 🧪 Development

### Running Tests

```bash
# Unit tests for all crates
cargo test --workspace

# With output
cargo test --workspace -- --nocapture
```

### Code Quality

```bash
# Check for errors without compiling
cargo check --workspace

# Lint (enforce zero warnings)
cargo clippy --workspace -- -D warnings

# Auto-format code
cargo fmt --all
```

### Project-Specific Commands

```bash
# Build only the CLI binary
cargo build -p cloudfang-cli

# Run with verbose logging
RUST_LOG=debug cloudfang start

# Check OpenStack connectivity only
cloudfang ops vm list --dry-run
```

---

## 🔐 Security Notes

> [!WARNING]
> **Never commit your `cloudfang.toml` to Git.** It contains your OpenStack credentials and LLM API keys. The file is already listed in `.gitignore`.

- Store sensitive credentials in `cloudfang.toml` (gitignored)
- For production, consider using environment variable overrides or a secrets manager
- The SQLite `cloudfang.db` should also be excluded from version control

---

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-new-hand`
3. Commit your changes with conventional commits: `git commit -m "feat(hands): add network-repair hand"`
4. Push and open a Pull Request

---

## 📄 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

---

<div align="center">

Built with ⚡ Rust & 🤖 LLM by the CloudFang team.

*"Your cloud ops team, running 24/7 — in Rust."*

</div>
