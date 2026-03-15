<div align="center">

# ☁️ CloudFang

**Autonomous OpenStack SysOps Agent — Built with Rust**

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)
[![OpenStack](https://img.shields.io/badge/OpenStack-Compatible-red?style=for-the-badge&logo=openstack)](https://www.openstack.org/)
[![LLM Powered](https://img.shields.io/badge/LLM-Powered-green?style=for-the-badge&logo=openai)](https://openai.com/)
[![Production Ready](https://img.shields.io/badge/Status-Production_Ready-success?style=for-the-badge)](#-production-ready-features)

---

*A lightweight, LLM-powered SysOps agent that monitors, heals, backs up, and scales your OpenStack cloud infrastructure — autonomously.*

[**Overview**](#-overview) | [**Features**](#-key-features) | [**Production Readiness**](#-production-ready-features) | [**Architecture**](#-architecture) | [**Getting Started**](#-getting-started)

---

</div>

## 🇻🇳 Giới thiệu (Vietnamese)

**CloudFang** là một dự án của X-OR CLOUD, xây dựng từ mã nguồn mở OpenFang và chuyển sang sử dụng ngôn  ngữ RUST. Các AI Agent tự hành cũng được xây dựng bằng ngôn ngữ Rust, thiết kế chuyên biệt cho việc quản trị hạ tầng **OpenStack**. Dự án này mang đến khả năng "tự vận hành" cho hệ thống đám mây của bạn, giúp tự động hóa các tác vụ giám sát, khắc phục sự cố, sao lưu và mở rộng tài nguyên mà không cần sự can thiệp liên tục từ con người.

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
           │ Client / Retry │    │  Backup / Scale     │
           └────────────────┘    └─────────┬───────────┘
                        │                  │
                     ┌──▼──────────────────▼─┐
                     │    cloudfang-store     │
                     │  SQLite + Audit Log    │
                     └────────────────────────┘
```

---

## ✨ Key Features

| Feature | Description |
|---|---|
| 🤖 **LLM-Powered Decisions** | Uses GPT-4o / Ollama to analyze metrics and perform complex tool calls. |
| 👁️ **Autonomous Monitoring** | Continuous health checks for VMs, disk, network, and OpenStack services. |
| 🔧 **Self-Healing Remediation** | Automatically detects failures and attempts recovery (e.g., Hard Reboot). |
| 💾 **Scheduled Backups** | Intelligent snapshot orchestration for critical VMs and volumes. |
| 📈 **Intelligent Scaling** | Trend analysis of system load with actionable recommendations. |
| 🗃️ **Universal Persistence** | All actions, logs, and metrics are stored safely in a thread-safe SQLite DB. |
| 💬 **Natural Language Ops** | Interactive chat interface to query status: *"Which VMs are idle for 7 days?"* |

---

## 🚀 Production Ready Features

CloudFang has been upgraded to a **100% Enterprise-Ready** state:

- **Fully Functioning "Hands"**: 
  - `MonitorHand` talks directly to OpenStack to scan for VM in ERROR states.
  - `RemediateHand` intercepts error incidents and auto-reboots broken VMs.
  - `BackupHand` automatically creates backups (`snapshots`) of volumes `in-use`.
  - `ScaleHand` proactively scans instance memory diagnostics to suggest scaling up/down.
- **Resilient API Calls (Retry Mechanism)**: Includes automatic backoff retries and dynamic token refreshes for robust network interactions.
- **Real LLM Tools**: AI reasoning is now hooked up to real Cloud operations. You can ask CloudFang to list your VMs, or reboot them, and it handles the underlying OpenStack APIs via Tool Calls.
- **Secret Management**: OpenStack credentials are no longer restricted to plain-text TOML files. Uses robust Environment Variables injection (`OS_PASSWORD`, `OS_USERNAME`, etc.).
- **Telegram Alerting Integration**: Receive proactive notifications right in your Telegram whenever `RemediateHand` executes an automated recovery. 
- **Thread-safe Persistence Layer:** Safely execute multi-threaded loops and async database inserts using an `Arc<Mutex<Connection>>` mapped SQLite storage.
- **Unit & Mock Testing:** Tested via `mockito` to accurately mirror Keystone identity servers to prevent token regressions.

---

## 🏗️ Architecture — Crate-Based Workspace

CloudFang is organized as a modular Rust workspace for maximum maintainability:

- **`cloudfang-core`**: The brain. Contains the agent loop, LLM integration, and real OpenStack generic Tools mappings.
- **`cloudfang-ops`**: The hands. Custom OpenStack API client implementing Keystone (Auth), Nova (Compute), Cinder (Storage), and Retry Network patterns.
- **`cloudfang-hands`**: The agents. Defines specific autonomous behaviors.
- **`cloudfang-store`**: The memory. SQLite backend for incident tracking and metrics history.
- **`cloudfang-cli`**: The face. Command-line interface and background Daemon.

---

## 🚀 Getting Started

### Prerequisites

- **Rust** `1.75+` — [Install via rustup](https://rustup.rs/)
- **Access to an OpenStack Cluster** 
- **OpenAI API Key** or local **Ollama** instance.

### 1. Installation

```bash
git clone https://github.com/vulekaisar/cloudfang.git
cd cloudfang
cargo build --release
```

### 2. Configuration & Secrets

CloudFang natively supports `.env` and `cloudfang.toml`. Configure your connection details:

```bash
# Provide environment variables via .env 
echo "OS_AUTH_URL=https://my-openstack/v3" >> .env
echo "OS_USERNAME=admin" >> .env
echo "OS_PASSWORD=supersecret" >> .env
echo "OS_PROJECT_NAME=admin" >> .env
echo "OS_DOMAIN_NAME=Default" >> .env

# Optional: Enable Telegram Notifications
echo "TELEGRAM_BOT_TOKEN=YOUR_BOT_TOKEN" >> .env
echo "TELEGRAM_CHAT_ID=YOUR_CHAT_ID" >> .env
```

### 3. Usage

```bash
# Initialize and verify connectivity
cloudfang init

# Start the autonomous daemon
cloudfang start

# Enter interactive chat mode with the AI Agent
cloudfang chat
```

### 4. Docker Deployment

Launch standard instances using the native container solution:

```bash
docker build -t cloudfang-agent .
docker run -d --name cloudfang-daemon --env-file .env cloudfang-agent
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

# Hand Management
cloudfang hand list          # List available SysOps hands
cloudfang hand run monitor   # Manually trigger a monitor cycle

# Audit & Insights
cloudfang incidents          # List detected and resolved incidents
```

---

## 🗺️ Roadmap & Progress

- [x] **Phase 1**: Workspace setup & OpenStack API abstractions (`cloudfang-ops`)
- [x] **Phase 2**: LLM Agent Loop, DB Stores & Tool Calling (`cloudfang-core`)
- [x] **Phase 3**: Core SysOps Hands implementation (`cloudfang-hands`)
- [x] **Phase 4**: Advanced Self-Healing, Retries, and Telegram Alerting
- [x] **Phase 5**: Dockerization & Enterprise Refactoring
- [ ] **Phase 6**: Web Interface & External Plugin Support

---
## Về X-OR CLOUD
xProxy là sản phẩm của X-OR CLOUD — nền tảng cloud và AI có chủ quyền hàng đầu tại Việt Nam và khu vực Đông Nam Á.

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

---

<div align="center">

Built with 🦀 by the CloudFang Team.

*"Automating the Cloud, one Crate at a time."*

</div>
