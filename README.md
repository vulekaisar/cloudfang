<div align="center">

# ☁️ CloudFang

**Autonomous OpenStack SysOps Agent — Built with Rust**

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)
[![OpenStack](https://img.shields.io/badge/OpenStack-Compatible-red?style=for-the-badge&logo=openstack)](https://www.openstack.org/)
[![LLM Powered](https://img.shields.io/badge/LLM-Powered-green?style=for-the-badge&logo=openai)](https://openai.com/)

---

*A lightweight, LLM-powered SysOps agent that monitors, heals, backs up, and scales your OpenStack cloud infrastructure — autonomously.*

[**Overview**](#-overview) | [**Features**](#-key-features) | [**Architecture**](#-architecture) | [**Getting Started**](#-getting-started) | [**CLI Reference**](#-cli-reference)

---

</div>

## 🇻🇳 Giới thiệu (Vietnamese)

**CloudFang** là một AI Agent tự hành được xây dựng bằng ngôn ngữ Rust, thiết kế chuyên biệt cho việc quản trị hạ tầng **OpenStack**. Dự án này mang đến khả năng "tự vận hành" cho hệ thống đám mây của bạn, giúp tự động hóa các tác vụ giám sát, khắc phục sự cố, sao lưu và mở rộng tài nguyên mà không cần sự can thiệp liên tục từ con người.

Với CloudFang, SysOps Team có thể chuyển từ việc "phản ứng với sự cố" sang "giám sát chủ động" thông qua sức mạnh của các **SysOps Hands** (Cánh tay vận hành) được hỗ trợ bởi các mô hình ngôn ngữ lớn (LLM) như GPT-4o hoặc các model chạy local qua Ollama.

---

## 🌟 Overview

**CloudFang** is an autonomous AI agent built in Rust, designed specifically for **OpenStack cloud infrastructure operations**. Taking inspiration from the OpenFang framework but optimized for performance and reliability in Rust, CloudFang acts as an intelligent layer above your cloud APIs.

Instead of manually watching dashboards, CloudFang's **SysOps Hands** work in the background — observing state, reasoning about issues, and executing remediation.

```text
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
| 🤖 **LLM-Powered Decisions** | Uses GPT-4o / Ollama to analyze metrics and perform complex tool calls. |
| 👁️ **Autonomous Monitoring** | Continuous health checks for VMs, disk, network, and OpenStack services. |
| 🔧 **Self-Healing Remediation** | Automatically detects failures and attempts recovery (restart, clear disk, etc.). |
| 💾 **Scheduled Backups** | Intelligent snapshot orchestration for critical VMs and volumes. |
| 📈 **Intelligent Scaling** | Trend analysis of system load with actionable recommendations or auto-scaling. |
| 🗃️ **Universal Persistence** | All actions, logs, and metrics are stored in a local SQLite database for auditing. |
| 💬 **Natural Language Ops** | Interactive chat interface to query status: *"Which VMs are idle for 7 days?"* |

---

## 🏗️ Architecture — Crate-Based Workspace

CloudFang is organized as a modular Rust workspace for maximum maintainability:

- **`cloudfang-core`**: The brain. Contains the agent loop, LLM integration (via `async-openai` and `rig-core`), and the task scheduler.
- **`cloudfang-ops`**: The hands. Custom OpenStack API client implementing Keystone (Auth), Nova (Compute), Neutron (Network), Cinder (Storage), and more.
- **`cloudfang-hands`**: The agents. Defines specific autonomous behaviors (Monitor, Remediate, Backup, Scale).
- **`cloudfang-store`**: The memory. SQLite backend for incident tracking, metrics history, and audit logs.
- **`cloudfang-cli`**: The face. Command-line interface and planned TUI for human interaction.

---

## 🚀 Getting Started

### Prerequisites

- **Rust** `1.75+` — [Install via rustup](https://rustup.rs/)
- **Access to an OpenStack Cluster** (or environment variables for mock mode)
- **OpenAI API Key** or local **Ollama** instance.

### 1. Installation

```bash
git clone https://github.com/vulekaisar/cloudfang.git
cd cloudfang
cargo build --release
```

### 2. Configuration

Copy the template and fill in your OpenStack and LLM credentials:

```bash
cp cloudfang.toml.example cloudfang.toml
# Edit cloudfang.toml with your favorite editor
```

### 3. Usage

```bash
# Initialize and verify connectivity
./target/release/cloudfang init

# Start the autonomous daemon
./target/release/cloudfang start

# Enter interactive chat mode
./target/release/cloudfang chat
```

---

## 💻 CLI Reference

CloudFang provides a powerful CLI for both management and direct cloud operations:

```bash
# System Operations
cloudfang init               # Initialize DB and check connections
cloudfang status             # Show daemon and hand status
cloudfang start              # Run as a background daemon

# Manual Cloud Ops (Thin wrapper around cloudfang-ops)
cloudfang ops vm list        # List all virtual machines
cloudfang ops network list   # List all networks
cloudfang ops volume list    # List all block volumes

# Hand Management
cloudfang hand list          # List available SysOps hands
cloudfang hand run monitor   # Manually trigger a monitor cycle

# Audit & Insights
cloudfang audit              # Show the latest logs from the agent
cloudfang incidents          # List detected and resolved incidents
```

---

## 🗺️ Roadmap & Progress

- [x] **Phase 1**: Workspace setup & OpenStack API abstractions (`cloudfang-ops`)
- [x] **Phase 2**: LLM Agent Loop & Tool Calling (`cloudfang-core`)
- [x] **Phase 3**: Core SysOps Hands implementation (`cloudfang-hands`)
- [ ] **Phase 4**: Advanced TUI Dashboard (`cloudfang-cli`)
- [ ] **Phase 5**: Multi-cloud support and Alerting (Telegram/Slack)

---

## 🛠️ Tech Stack

- **Language:** Rust (2021 Edition)
- **Runtime:** Tokio (Async)
- **LLM Layer:** [Rig](https://github.com/0xPlaygrounds/rig) + `async-openai`
- **Networking:** Reqwest + Rustls
- **Database:** SQLite (via `rusqlite`)
- **CLI:** Clap v4

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

---

<div align="center">

Built with 🦀 by the CloudFang Team.

*"Automating the Cloud, one Crate at a time."*

</div>
