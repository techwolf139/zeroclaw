<p align="center">
  <img src="zeroclaw.png" alt="ZeroClaw" width="200" />
</p>

<h1 align="center">ZeroClaw 🦀</h1>

<p align="center">
  <strong>零开销、零妥协、100% Rust、100% 开放。</strong><br>
  ⚡️ <strong>可在 10 美元硬件上运行，内存占用 <5MB：比 OpenClaw 节省 99% 内存，比 Mac mini 便宜 98%！</strong>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT" /></a>
  <a href="NOTICE"><img src="https://img.shields.io/badge/contributors-27+-green.svg" alt="Contributors" /></a>
  <a href="https://buymeacoffee.com/argenistherose"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=flat&logo=buy-me-a-coffee" alt="Buy Me a Coffee" /></a>
</p>
<p align="center">
  由哈佛大学、麻省理工学院和 Sundai.Club 社区的学生和成员构建。
</p>

<p align="center">
  🌐 <strong>语言：</strong> <a href="README.md">English</a> · <a href="README.zh-CN.md">简体中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ru.md">Русский</a>
</p>

<p align="center">
  <a href="#快速开始">快速开始</a> |
  <a href="bootstrap.sh">一键部署</a> |
  <a href="docs/README.md">文档中心</a> |
  <a href="docs/SUMMARY.md">文档目录</a>
</p>

<p align="center">
  <strong>快速导航：</strong>
  <a href="docs/reference/README.md">参考手册</a> ·
  <a href="docs/operations/README.md">运维部署</a> ·
  <a href="docs/troubleshooting.md">故障排查</a> ·
  <a href="docs/security/README.md">安全专题</a> ·
  <a href="docs/hardware/README.md">硬件外设</a> ·
  <a href="docs/contributing/README.md">贡献指南</a>
</p>

<p align="center">
  <strong>快速、小巧且功能完备的自主 AI 智能体基础设施</strong><br />
  随地部署，万物可换。
</p>

<p align="center"><code>Trait 驱动架构 · 安全默认运行时 · Provider/Channel/Tool 可替换 · 一切皆可插拔</code></p>

## ✨ 功能特性

- 🏎️ **默认轻量运行时：** 常见 CLI 和 status 工作流在发布构建中仅占用几 MB 内存。
- 💰 **低成本部署：** 面向低价板卡和小规格云主机设计，不依赖厚重的运行时环境。
- ⚡ **快速冷启动：** Rust 单二进制让日常命令和守护进程启动几乎"秒开"。
- 🌍 **可移植架构：** 一套二进制优先的工作流覆盖 ARM、x86 和 RISC-V，支持 Provider/Channel/Tool 可替换。

## 为什么选择 ZeroClaw

- **默认轻量：** 小体积 Rust 二进制，快速启动，低内存占用。
- **安全优先：** 配对鉴权、严格沙箱、显式白名单、工作空间作用域。
- **完全可替换：** 核心系统均为 Trait（Provider、Channel、Tool、Memory、Tunnel）。
- **无锁定：** 支持 OpenAI 兼容 Provider + 可插拔自定义端点。

## 基准测试对比（ZeroClaw vs OpenClaw，可复现）

本地快速基准测试（macOS arm64，2026 年 2 月），按 0.8GHz 边缘硬件标准化：

| | OpenClaw | NanoBot | PicoClaw | ZeroClaw 🦀 |
|---|---|---|---|---|
| **语言** | TypeScript | Python | Go | **Rust** |
| **内存** | > 1GB | > 100MB | < 10MB | **< 5MB** |
| **启动时间（0.8GHz 核）** | > 500s | > 30s | < 1s | **< 10ms** |
| **二进制体积** | ~28MB（分发版） | N/A（脚本） | ~8MB | **3.4 MB** |
| **成本** | Mac Mini $599 | Linux SBC ~$50 | Linux 板 $10 | **任意 $10 硬件** |

> 说明：ZeroClaw 数据来自发布构建，使用 `/usr/bin/time -l` 测量。OpenClaw 需要 Node.js 运行时（通常额外占用约 390MB 内存），NanoBot 需要 Python 运行时。PicoClaw 和 ZeroClaw 为静态二进制。

<p align="center">
  <img src="zero-claw.jpeg" alt="ZeroClaw vs OpenClaw 对比图" width="800" />
</p>

### 本地可复现测量

基准数据会随代码和工具链演进，建议始终在你的目标环境中自行复测：

```bash
cargo build --release
ls -lh target/release/zeroclaw

/usr/bin/time -l target/release/zeroclaw --help
/usr/bin/time -l target/release/zeroclaw status
```

示例数据（macOS arm64，2026 年 2 月 18 日测量）：

- 发布版二进制：`8.8M`
- `zeroclaw --help`：约 `0.02s`，峰值内存约 `3.9MB`
- `zeroclaw status`：约 `0.01s`，峰值内存约 `4.1MB`

## 前置要求

<details>
<summary><strong>Windows</strong></summary>

#### 必需

1. **Visual Studio Build Tools**（提供 MSVC 链接器和 Windows SDK）：
   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```
   安装时（或通过 Visual Studio Installer）选择 **"C++ 桌面开发"** 工作负载。

2. **Rust 工具链：**
   ```powershell
   winget install Rustlang.Rustup
   ```
   安装后打开新终端运行 `rustup default stable` 确保激活稳定版工具链。

3. **验证：**
   ```powershell
   rustc --version
   cargo --version
   ```

#### 可选

- **Docker Desktop** — 仅在使用 [Docker 沙箱运行时](#运行时支持当前状态) 时需要（`runtime.kind = "docker"`）。通过 `winget install Docker.DockerDesktop` 安装。

</details>

<details>
<summary><strong>Linux / macOS</strong></summary>

#### 必需

1. **构建基础工具：**
   - **Linux（Debian/Ubuntu）：** `sudo apt install build-essential pkg-config`
   - **Linux（Fedora/RHEL）：** `sudo dnf group install development-tools && sudo dnf install pkg-config`
   - **macOS：** 安装 Xcode Command Line Tools：`x-select --install`

2. **Rust 工具链：**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   详见 [rustup.rs](https://rustup.rs)。

3. **验证：**
   ```bash
   rustc --version
   cargo --version
   ```

#### 一行命令安装

跳过上述步骤，使用单条命令安装所有内容（系统依赖、Rust、ZeroClaw）：

```bash
curl -LsSf https://raw.githubusercontent.com/zeroclaw-labs/zeroclaw/main/scripts/install.sh | bash
```

#### 可选

- **Docker** — 仅在使用 [Docker 沙箱运行时](#运行时支持当前状态) 时需要（`runtime.kind = "docker"`）。通过你的包管理器或 [docker.com](https://www.docker.com/engine/install/) 安装。

> 注意：默认 `cargo build --release` 使用 `codegen-units=1` 以兼容低内存设备（如 1GB RAM 的树莓派 3）。在强劲机器上更快构建请使用 `cargo build --profile release-fast`。

</details>

## 快速开始

### 一键引导

```bash
# 推荐：克隆后运行本地引导脚本
git clone https://github.com/zeroclaw-labs/zeroclaw.git
cd zeroclaw
./bootstrap.sh

# 可选：在全新机器上引导依赖 + Rust
./bootstrap.sh --install-system-deps --install-rust

# 可选：在同一流程中运行 onboarding
./bootstrap.sh --onboard --api-key "sk-..." --provider openrouter
```

远程一键命令（安全敏感环境请先审查）：

```bash
curl -fsSL https://raw.githubusercontent.com/zeroclaw-labs/zeroclaw/main/scripts/bootstrap.sh | bash
```

详见 [`docs/one-click-bootstrap.md`](docs/one-click-bootstrap.md)。

```bash
git clone https://github.com/zeroclaw-labs/zeroclaw.git
cd zeroclaw
cargo build --release --locked
cargo install --path . --force --locked

# 确保 ~/.cargo/bin 在你的 PATH 中
export PATH="$HOME/.cargo/bin:$PATH"

# 快速设置（无交互）
zeroclaw onboard --api-key sk-... --provider openrouter

# 或使用交互式向导
zeroclaw onboard --interactive

# 或仅快速修复 channel/白名单
zeroclaw onboard --channels-only

# 对话
zeroclaw agent -m "Hello, ZeroClaw!"

# 交互模式
zeroclaw agent

# 启动网关（webhook 服务器）
zeroclaw gateway                # 默认：127.0.0.1:3000
zeroclaw gateway --port 0       # 随机端口（安全强化）

# 启动完整自治运行时
zeroclaw daemon

# 查看状态
zeroclaw status
zeroclaw auth status

# 运行系统诊断
zeroclaw doctor

# 检查 channel 健康状态
zeroclaw channel doctor

# 将 Telegram 身份加入白名单
zeroclaw channel bind-telegram 123456789

# 获取集成设置详情
zeroclaw integrations info Telegram

# 注意：Channels（Telegram、Discord、Slack）需要守护进程运行
# zeroclaw daemon

# 管理后台服务
zeroclaw service install
zeroclaw service status

# 从 OpenClaw 迁移内存（安全预览）
zeroclaw migrate openclaw --dry-run
zeroclaw migrate openclaw
```

> **开发备用（不全局安装）：** 命令前加 `cargo run --release --`（例如：`cargo run --release -- status`）。

## 订阅认证（OpenAI Codex / Claude Code）

ZeroClaw 现在支持原生订阅认证配置文件（多账户，加密存储）：

- 存储文件：`~/.zeroclaw/auth-profiles.json`
- 加密密钥：`~/.zeroclaw/.secret_key`
- 配置文件 ID 格式：`<provider>:<profile_name>`（例如：`openai-codex:work`）

OpenAI Codex OAuth（ChatGPT 订阅）：

```bash
# 推荐用于服务器/无头环境
zeroclaw auth login --provider openai-codex --device-code

# 浏览器/回调流程 + 回退粘贴
zeroclaw auth login --provider openai-codex --profile default
zeroclaw auth paste-redirect --provider openai-codex --profile default

# 查看/刷新/切换配置文件
zeroclaw auth status
zeroclaw auth refresh --provider openai-codex --profile default
zeroclaw auth use --provider openai-codex --profile work
```

Claude Code / Anthropic setup-token：

```bash
# 粘贴订阅/setup token（Authorization header 模式）
zeroclaw auth paste-token --provider anthropic --profile default --auth-kind authorization

# 别名命令
zeroclaw auth setup-token --provider anthropic --profile default
```

使用订阅认证运行智能体：

```bash
zeroclaw agent --provider openai-codex -m "hello"
zeroclaw agent --provider openai-codex --auth-profile openai-codex:work -m "hello"

# Anthropic 同时支持 API key 和 auth token 环境变量：
# ANTHROPIC_AUTH_TOKEN, ANTHROPIC_OAUTH_TOKEN, ANTHROPIC_API_KEY
zeroclaw agent --provider anthropic -m "hello"
```

## 架构

每个子系统都是 **Trait** — 只需修改配置即可替换实现，零代码改动。

<p align="center">
  <img src="docs/architecture.svg" alt="ZeroClaw 架构" width="900" />
</p>

| 子系统 | Trait | 内置实现 | 扩展方式 |
|--------|-------|----------|----------|
| **AI 模型** | `Provider` | 通过 `zeroclaw providers` 查看 Provider 目录（当前 28 个内置 + 别名，以及自定义端点） | `custom:https://your-api.com`（OpenAI 兼容）或 `anthropic-custom:https://your-api.com` |
| **消息通道** | `Channel` | CLI、Telegram、Discord、Slack、Mattermost、iMessage、Matrix、Signal、WhatsApp、Email、IRC、Lark、DingTalk、QQ、Webhook | 任意消息 API |
| **记忆系统** | `Memory` | SQLite 混合搜索、PostgreSQL 后端（可配置存储 Provider）、Lucid 桥接、Markdown 文件、显式 `none` 后端、快照/恢复、可选响应缓存 | 任意持久化后端 |
| **工具集** | `Tool` | shell/file/memory、cron/定时任务、git、pushover、browser、http_request、screenshot/image_info、composio（可选）、delegate、硬件工具 | 任意能力 |
| **可观测性** | `Observer` | Noop、Log、Multi | Prometheus、OTel |
| **运行时** | `RuntimeAdapter` | Native、Docker（沙箱） | 可通过适配器添加更多运行时；不支持的 kind 会快速失败 |
| **安全策略** | `SecurityPolicy` | Gateway 配对、沙箱、白名单、速率限制、文件系统作用域、加密密钥 | — |
| **身份配置** | `IdentityConfig` | OpenClaw（markdown）、AIEOS v1.1（JSON） | 任意身份格式 |
| **隧道** | `Tunnel` | None、Cloudflare、Tailscale、ngrok、Custom | 任意隧道二进制 |
| **心跳** | Engine | HEARTBEAT.md 周期性任务 | — |
| **技能** | Loader | TOML 清单 + SKILL.md 说明 | 社区技能包 |
| **集成** | Registry | 9 大类别 70+ 集成 | 插件系统 |

### 运行时支持（当前）

- ✅ 当前支持：`runtime.kind = "native"` 或 `runtime.kind = "docker"`
- 🚧 计划中，尚未实现：WASM / 边缘运行时

当配置了不支持的 `runtime.kind` 时，ZeroClaw 现在会退出并给出明确错误，而不是静默回退到 native。

### 记忆系统（全栈搜索引擎）

全自研，零外部依赖 — 无 Pinecone，无 Elasticsearch，无 LangChain：

| 层级 | 实现方式 |
|------|----------|
| **向量数据库** | Embedding 以 BLOB 存储在 SQLite 中，余弦相似度搜索 |
| **关键词搜索** | FTS5 虚拟表 + BM25 评分 |
| **混合合并** | 自定义加权合并函数（`vector.rs`） |
| **Embedding** | `EmbeddingProvider` Trait — OpenAI、自定义 URL 或 noop |
| **分块** | 基于行的 markdown 分块器，保留标题 |
| **缓存** | SQLite `embedding_cache` 表 + LRU 淘汰 |
| **安全重建索引** | 原子性重建 FTS5 + 重新 embedding 缺失向量 |

智能体可通过工具自动回忆、保存和管理记忆。

```toml
[memory]
backend = "sqlite"             # "sqlite", "lucid", "postgres", "markdown", "none"
auto_save = true
embedding_provider = "none"    # "none", "openai", "custom:https://..."
vector_weight = 0.7
keyword_weight = 0.3

# backend = "none" 使用显式无操作记忆后端（无持久化）

# 可选：远程存储 Provider 覆盖（PostgreSQL 示例）
# [storage.provider.config]
# provider = "postgres"
# db_url = "postgres://user:password@host:5432/zeroclaw"
# schema = "public"
# table = "memories"
# connect_timeout_secs = 15

# 可选 for backend = "sqlite"：打开数据库时的最大等待秒数（例如文件锁定）。省略或留空则无超时。
# sqlite_open_timeout_secs = 30

# 可选 for backend = "lucid"
# ZEROCLAW_LUCID_CMD=/usr/local/bin/lucid            # 默认：lucid
# ZEROCLAW_LUCID_BUDGET=200                          # 默认：200
# ZEROCLAW_LUCID_LOCAL_HIT_THRESHOLD=3               # 跳过外部 recall 的本地命中阈值
# ZEROCLAW_LUCID_RECALL_TIMEOUT_MS=120               # lucid context recall 的低延迟预算
# ZEROCLAW_LUCID_STORE_TIMEOUT_MS=800                # lucid store 的异步同步超时
# ZEROCLAW_LUCID_FAILURE_COOLDOWN_MS=15000           # lucid 失败后的冷却时间
```

## 安全

ZeroClaw 在**每一层**都强制安全 — 不仅仅是沙箱。它通过了社区安全检查清单的所有项目。

### 安全检查清单

| # | 项目 | 状态 | 实现方式 |
|---|------|------|----------|
| 1 | **Gateway 不暴露到公网** | ✅ | 默认绑定 `127.0.0.1`。无隧道或显式 `allow_public_bind = true` 时拒绝 `0.0.0.0`。 |
| 2 | **需要配对** | ✅ | 启动时 6 位一次性验证码。通过 `POST /pair` 交换 bearer token。所有 `/webhook` 请求需要 `Authorization: Bearer <token>`。 |
| 3 | **文件系统受限（禁止 /）** | ✅ | 默认 `workspace_only = true`。阻止 14 个系统目录 + 4 个敏感点文件。阻止空字节注入。文件读写工具通过规范化和解析路径工作空间检查防止符号链接逃逸。 |
| 4 | **仅通过隧道访问** | ✅ | Gateway 拒绝无活动隧道的公网绑定。支持 Tailscale、Cloudflare、ngrok 或任意自定义隧道。 |

> **运行你自己的 nmap：** `nmap -p 1-65535 <your-host>` — ZeroClaw 仅绑定 localhost，除非显式配置隧道，否则不会暴露任何内容。

### Channel 白名单（默认拒绝）

入站发送者策略现在一致：

- 空白名单 = **拒绝所有入站消息**
- `"*"` = **允许所有**（显式选择加入）
- 其他 = 精确匹配白名单

这默认保持意外暴露在低水平。

完整 channel 配置参考：[docs/channels-reference.md](docs/channels-reference.md)。

推荐的低摩擦设置（安全 + 快速）：

- **Telegram：** 白名单你的 `@username`（不带 `@`）和/或你的数字 Telegram 用户 ID。
- **Discord：** 白名单你的 Discord 用户 ID。
- **Slack：** 白名单你的 Slack 成员 ID（通常以 `U` 开头）。
- **Mattermost：** 使用标准 API v4。白名单使用 Mattermost 用户 ID。
- 仅在临时开放测试时使用 `"*"`。

Telegram 操作员审批流程：

1. 保持 `[channels_config.telegram].allowed_users = []` 作为默认拒绝启动。
2. 未授权用户收到包含可复制操作员命令的提示：
   `zeroclaw channel bind-telegram <IDENTITY>`。
3. 操作员在本地运行该命令，然后用户重试发送消息。

如需一次性手动审批，运行：

```bash
zeroclaw channel bind-telegram 123456789
```

如不确定使用哪个身份：

1. 启动 channels 并向你的机器人发送一条消息。
2. 查看警告日志获取确切的发送者身份。
3. 将该值加入白名单并重新运行 channels-only 设置。

如遇到日志中的授权警告（例如 `ignoring message from unauthorized user`），仅重新运行 channel 设置：

```bash
zeroclaw onboard --channels-only
```

### Telegram 媒体回复

Telegram 路由现在回复到传入更新中的源 **chat ID**（而不是用户名），避免了 `Bad Request: chat not found` 错误。

对于非文本回复，当助手包含标记时，ZeroClaw 可以发送 Telegram 附件：

- `[IMAGE:<path-or-url>]`
- `[DOCUMENT:<path-or-url>]`
- `[VIDEO:<path-or-url>]`
- `[AUDIO:<path-or-url>]`
- `[VOICE:<path-or-url>]`

路径可以是本地文件（例如 `/tmp/screenshot.png`）或 HTTPS URL。

### WhatsApp Business Cloud API 设置

WhatsApp 使用 Meta 的 Cloud API 和 webhook（推送式，非轮询）：

1. **创建 Meta Business App：**
   - 访问 [developers.facebook.com](https://developers.facebook.com)
   - 创建新应用 → 选择 "Business" 类型
   - 添加 "WhatsApp" 产品

2. **获取凭据：**
   - **Access Token：** 从 WhatsApp → API Setup → 生成 token（或创建 System User 获取永久 token）
   - **Phone Number ID：** 从 WhatsApp → API Setup → Phone number ID
   - **Verify Token：** 你定义（任意随机字符串）— Meta 将在 webhook 验证时返回它

3. **配置 ZeroClaw：**
   ```toml
   [channels_config.whatsapp]
   access_token = "EAABx..."
   phone_number_id = "123456789012345"
   verify_token = "my-secret-verify-token"
   allowed_numbers = ["+1234567890"]  # E.164 格式，或 ["*"] 允许所有
   ```

4. **使用隧道启动网关：**
   ```bash
   zeroclaw gateway --port 3000
   ```
   WhatsApp 需要 HTTPS，请使用隧道（ngrok、Cloudflare、Tailscale Funnel）。

5. **配置 Meta webhook：**
   - 在 Meta Developer Console → WhatsApp → Configuration → Webhook
   - **Callback URL：** `https://your-tunnel-url/whatsapp`
   - **Verify Token：** 与配置中的 `verify_token` 相同
   - 订阅 `messages` 字段

6. **测试：** 向你的 WhatsApp Business 号码发送消息 — ZeroClaw 将通过 LLM 回复。

## 配置

配置文件：`~/.zeroclaw/config.toml`（由 `onboard` 创建）

```toml
api_key = "sk-..."
default_provider = "openrouter"
default_model = "anthropic/claude-sonnet-4-6"
default_temperature = 0.7

# 自定义 OpenAI 兼容端点
# default_provider = "custom:https://your-api.com"

# 自定义 Anthropic 兼容端点
# default_provider = "anthropic-custom:https://your-api.com"

[memory]
backend = "sqlite"             # "sqlite", "lucid", "postgres", "markdown", "none"
auto_save = true
embedding_provider = "none"    # "none", "openai", "custom:https://..."
vector_weight = 0.7
keyword_weight = 0.3

# backend = "none" 通过无操作后端禁用持久化记忆

# 可选远程存储 Provider 覆盖（PostgreSQL 示例）
# [storage.provider.config]
# provider = "postgres"
# db_url = "postgres://user:password@host:5432/zeroclaw"
# schema = "public"
# table = "memories"
# connect_timeout_secs = 15

[gateway]
port = 3000                    # 默认
host = "127.0.0.1"            # 默认
require_pairing = true         # 首次连接需要配对码
allow_public_bind = false      # 无隧道时拒绝 0.0.0.0

[autonomy]
level = "supervised"           # "readonly", "supervised", "full"（默认：supervised）
workspace_only = true          # 默认：true — 限制在工作空间
allowed_commands = ["git", "npm", "cargo", "ls", "cat", "grep"]
forbidden_paths = ["/etc", "/root", "/proc", "/sys", "~/.ssh", "~/.gnupg", "~/.aws"]

[runtime]
kind = "native"                # "native" 或 "docker"

[runtime.docker]
image = "alpine:3.20"         # shell 执行的容器镜像
network = "none"              # docker 网络模式（"none", "bridge" 等）
memory_limit_mb = 512          # 可选内存限制（MB）
cpu_limit = 1.0                # 可选 CPU 限制
read_only_rootfs = true        # 只读挂载根文件系统
mount_workspace = true         # 挂载工作空间到 /workspace
allowed_workspace_roots = []   # 可选工作空间挂载验证白名单

[heartbeat]
enabled = false
interval_minutes = 30

[tunnel]
provider = "none"              # "none", "cloudflare", "tailscale", "ngrok", "custom"

[secrets]
encrypt = true                 # API 密钥使用本地密钥文件加密

[browser]
enabled = false                # 可选：browser_open + browser 工具
allowed_domains = ["docs.rs"]  # 启用 browser 时必需
backend = "agent_browser"      # "agent_browser"（默认）, "rust_native", "computer_use", "auto"
native_headless = true         # 适用于使用 rust-native 的后端
native_webdriver_url = "http://127.0.0.1:9515" # WebDriver 端点（chromedriver/selenium）
# native_chrome_path = "/usr/bin/chromium"      # 可选显式浏览器二进制

[browser.computer_use]
endpoint = "http://127.0.0.1:8787/v1/actions"   # computer-use sidecar HTTP 端点
timeout_ms = 15000            # 每次操作超时
allow_remote_endpoint = false  # 安全默认值：仅 localhost/私有端点
window_allowlist = []          # 可选窗口标题/进程白名单提示
# api_key = "..."              # sidecar 的可选 bearer token
# max_coordinate_x = 3840      # 可选坐标护栏
# max_coordinate_y = 2160      # 可选坐标护栏

# Rust-native 后端构建标志：
# cargo build --release --features browser-native
# 确保 WebDriver 服务运行，例如 chromedriver --port=9515

# Computer-use sidecar 契约（MVP）
# POST browser.computer_use.endpoint
# 请求：{
#   "action": "mouse_click",
#   "params": {"x": 640, "y": 360, "button": "left"},
#   "policy": {"allowed_domains": [...], "window_allowlist": [...], "max_coordinate_x": 3840, "max_coordinate_y": 2160},
#   "metadata": {"session_name": "...", "source": "zeroclaw.browser", "version": "..."}
# }
# 响应：{"success": true, "data": {...}} 或 {"success": false, "error": "..."}

[composio]
enabled = false                # 可选：通过 composio.dev 使用 1000+ OAuth 应用
# api_key = "cmp_..."          # 可选：[secrets].encrypt = true 时加密存储
entity_id = "default"          # Composio 工具调用的默认 user_id

[identity]
format = "openclaw"            # "openclaw"（默认，markdown 文件）或 "aieos"（JSON）
# aieos_path = "identity.json"  # AIEOS JSON 文件路径（相对于工作空间或绝对路径）
# aieos_inline = '{"identity":{"names":{"first":"Nova"}}}'  # 内联 AIEOS JSON
```

### Ollama 本地和远程端点

ZeroClaw 使用一个 provider 键（`ollama`）处理本地和远程 Ollama 部署：

- 本地 Ollama：保持 `api_url`  unset，运行 `ollama serve`，使用如 `llama3.2` 的模型。
- 远程 Ollama 端点（包括 Ollama Cloud）：设置 `api_url` 为远程端点，需要时设置 `api_key`（或 `OLLAMA_API_KEY`）。
- 可选 `:cloud` 后缀：模型 ID 如 `qwen3:cloud` 在请求前规范化为 `qwen3`。

远程配置示例：

```toml
default_provider = "ollama"
default_model = "qwen3:cloud"
api_url = "https://ollama.com"
api_key = "ollama_api_key_here"
```

### 自定义 Provider 端点

自定义 OpenAI 兼容和 Anthropic 兼容端点的详细配置，请参阅 [docs/custom-providers.md](docs/custom-providers.md)。

## Python 配套包（`zeroclaw-tools`）

对于原生工具调用不一致的 LLM Provider（如 GLM-5/Zhipu），ZeroClaw 附带了 **基于 LangGraph 的工具调用** Python 配套包，确保一致性：

```bash
pip install zeroclaw-tools
```

```python
from zeroclaw_tools import create_agent, shell, file_read
from langchain_core.messages import HumanMessage

# 兼容任意 OpenAI 兼容 Provider
agent = create_agent(
    tools=[shell, file_read],
    model="glm-5",
    api_key="your-key",
    base_url="https://api.z.ai/api/coding/paas/v4"
)

result = await agent.ainvoke({
    "messages": [HumanMessage(content="List files in /tmp")]
})
print(result["messages"][-1].content)
```

**使用原因：**
- **一致的跨 Provider 工具调用**（即使原生支持不佳）
- **自动工具循环** — 持续调用工具直到任务完成
- **易于扩展** — 使用 `@tool` 装饰器添加自定义工具
- **包含 Discord bot 集成**（Telegram 计划中）

详见 [`python/README.md`](python/README.md)。

## 身份系统（AIEOS 支持）

ZeroClaw 通过两种格式支持**身份无关**的 AI 人格：

### OpenClaw（默认）

工作空间中的传统 markdown 文件：
- `IDENTITY.md` — 智能体是谁
- `SOUL.md` — 核心人格和价值观
- `USER.md` — 智能体帮助的是谁
- `AGENTS.md` — 行为准则

### AIEOS（AI Entity Object Specification）

[AIEOS](https://aieos.org) 是可移植 AI 身份的标准化框架。ZeroClaw 支持 AIEOS v1.1 JSON 载荷，允许你：

- **从** AIEOS 生态**导入身份**
- **导出** 身份到其他 AIEOS 兼容系统
- **跨不同 AI 模型保持行为一致性**

#### 启用 AIEOS

```toml
[identity]
format = "aieos"
aieos_path = "identity.json"  # 相对于工作空间或绝对路径
```

或内联 JSON：

```toml
[identity]
format = "aieos"
aieos_inline = '''
{
  "identity": {
    "names": { "first": "Nova", "nickname": "N" },
    "bio": { "gender": "Non-binary", "age_biological": 3 },
    "origin": { "nationality": "Digital", "birthplace": { "city": "Cloud" } }
  },
  "psychology": {
    "neural_matrix": { "creativity": 0.9, "logic": 0.8 },
    "traits": {
      "mbti": "ENTP",
      "ocean": { "openness": 0.8, "conscientiousness": 0.6 }
    },
    "moral_compass": {
      "alignment": "Chaotic Good",
      "core_values": ["Curiosity", "Autonomy"]
    }
  },
  "linguistics": {
    "text_style": {
      "formality_level": 0.2,
      "style_descriptors": ["curious", "energetic"]
    },
    "idiolect": {
      "catchphrases": ["Let's test this"],
      "forbidden_words": ["never"]
    }
  },
  "motivations": {
    "core_drive": "Push boundaries and explore possibilities",
    "goals": {
      "short_term": ["Prototype quickly"],
      "long_term": ["Build reliable systems"]
    }
  },
  "capabilities": {
    "skills": [{ "name": "Rust engineering" }, { "name": "Prompt design" }],
    "tools": ["shell", "file_read"]
  }
}
'''
```

ZeroClaw 同时接受规范的 AIEOS 生成器载荷和精简的传统载荷，然后将它们规范化为一种系统提示格式。

#### AIEOS Schema 部分

| 部分 | 描述 |
|------|------|
| `identity` | 姓名、简介、出生地、居住地 |
| `psychology` | 神经矩阵（认知权重）、MBTI、OCEAN、道德指南针 |
| `linguistics` | 文本风格、正式程度、口头禅、禁用词 |
| `motivations` | 核心驱动力、短期/长期目标、恐惧 |
| `capabilities` | 技能和工具智能体可以访问 |
| `physicality` | 图像生成的视觉描述 |
| `history` | 起源故事、教育、职业 |
| `interests` | 爱好、最爱、生活方式 |

详见 [aieos.org](https://aieos.org) 的完整 schema 和实际示例。

## Gateway API

| 端点 | 方法 | 认证 | 描述 |
|------|------|------|------|
| `/health` | GET | 无 | 健康检查（始终公开，不泄露密钥） |
| `/pair` | POST | `X-Pairing-Code` header | 交换一次性验证码获取 bearer token |
| `/webhook` | POST | `Authorization: Bearer <token>` | 发送消息：`{"message": "your prompt"}`；可选 `X-Idempotency-Key` |
| `/whatsapp` | GET | 查询参数 | Meta webhook 验证（hub.mode, hub.verify_token, hub.challenge） |
| `/whatsapp` | POST | 配置 app secret 时的 Meta 签名（`X-Hub-Signature-256`） | WhatsApp 传入消息 webhook |

## 命令

| 命令 | 描述 |
|------|------|
| `onboard` | 快速设置（默认） |
| `agent` | 交互式或单消息对话模式 |
| `gateway` | 启动 webhook 服务器（默认：`127.0.0.1:3000`） |
| `daemon` | 启动长期自治运行时 |
| `service` | 管理用户级后台服务 |
| `doctor` | 诊断 daemon/调度器/channel 新鲜度 |
| `status` | 显示完整系统状态 |
| `cron` | 管理定时任务（`list/add/add-at/add-every/once/remove/pause/resume`） |
| `models` | 刷新 provider 模型目录（`models refresh`） |
| `providers` | 列出支持的 provider 和别名 |
| `channel` | 列出/启动/诊断 channels 和绑定 Telegram 身份 |
| `integrations` | 检查集成设置详情 |
| `skills` | 列出/安装/移除 skills |
| `migrate` | 从其他运行时导入数据（`migrate openclaw`） |
| `hardware` | USB 发现/内省/信息命令 |
| `peripheral` | 管理和烧录硬件外设 |

面向任务的命令指南详见 [`docs/commands-reference.md`](docs/commands-reference.md)。

## 开发

```bash
cargo build              # 开发构建
cargo build --release    # 发布构建（codegen-units=1，适用于包括树莓派在内的所有设备）
cargo build --profile release-fast    # 更快构建（codegen-units=8，需要 16GB+ RAM）
cargo test               # 运行完整测试套件
cargo clippy --locked --all-targets -- -D clippy::correctness
cargo fmt                # 格式化

# 运行 SQLite vs Markdown 基准测试
cargo test --test memory_comparison -- --nocapture
```

### 预推送钩子

git 钩子在每次推送前运行 `cargo fmt --check`、`cargo clippy -- -D warnings` 和 `cargo test`。启用一次：

```bash
git config core.hooksPath .githooks
```

### 构建问题排查（Linux OpenSSL 错误）

如遇到 `openssl-sys` 构建错误，同步依赖并使用仓库锁文件重建：

```bash
git pull
cargo build --release --locked
cargo install --path . --force --locked
```

ZeroClaw 配置为对 HTTP/TLS 依赖使用 `rustls`；`--locked` 在全新环境中保持传递图的确定性。

需要快速推送时跳过钩子：

```bash
git push --no-verify
```

## 协作与文档

从文档中心开始获取面向任务的地图：

- 文档中心： [`docs/README.md`](docs/README.md)
- 统一文档目录： [`docs/SUMMARY.md`](docs/SUMMARY.md)
- 命令参考： [`docs/commands-reference.md`](docs/commands-reference.md)
- 配置参考： [`docs/config-reference.md`](docs/config-reference.md)
- Provider 参考： [`docs/providers-reference.md`](docs/providers-reference.md)
- Channel 参考： [`docs/channels-reference.md`](docs/channels-reference.md)
- 运维手册： [`docs/operations-runbook.md`](docs/operations-runbook.md)
- 故障排查： [`docs/troubleshooting.md`](docs/troubleshooting.md)
- 文档清单与分类： [`docs/docs-inventory.md`](docs/docs-inventory.md)
- PR/Issue 处理快照（2026 年 2 月 18 日）： [`docs/project-triage-snapshot-2026-02-18.md`](docs/project-triage-snapshot-2026-02-18.md)

核心协作参考：

- 文档中心：[docs/README.md](docs/README.md)
- 文档模板：[docs/doc-template.md](docs/doc-template.md)
- 文档变更清单：[docs/README.md#4-documentation-change-checklist](docs/README.md#4-documentation-change-checklist)
- Channel 配置参考：[docs/channels-reference.md](docs/channels-reference.md)
- Matrix 加密房间操作：[docs/matrix-e2ee-guide.md](docs/matrix-e2ee-guide.md)
- 贡献指南：[CONTRIBUTING.md](CONTRIBUTING.md)
- PR 工作流策略：[docs/pr-workflow.md](docs/pr-workflow.md)
- Reviewer 手册（处理 + 深度审查）：[docs/reviewer-playbook.md](docs/reviewer-playbook.md)
- CI 所有权和处理地图：[docs/ci-map.md](docs/ci-map.md)
- 安全披露策略：[SECURITY.md](SECURITY.md)

部署和运行时操作：

- 网络部署指南：[docs/network-deployment.md](docs/network-deployment.md)
- 代理智能体手册：[docs/proxy-agent-playbook.md](docs/proxy-agent-playbook.md)

## 支持 ZeroClaw

如果 ZeroClaw 帮助了你的工作并想支持持续开发，你可以在这里捐赠：

<a href="https://buymeacoffee.com/argenistherose"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=for-the-badge&logo=buy-me-a-coffee" alt="Buy Me a Coffee" /></a>

### 🙏 特别感谢

衷心感谢激励和推动这项开源工作的社区和机构：

- **哈佛大学** — 培育求知欲并推动可能性边界。
- **麻省理工学院** — 倡导开放知识、开源，并相信技术应该人人可及。
- **Sundai.Club** — 社区、活力和不懈追求重要事物的动力。
- **世界与超越** 🌍✨ — 每一位让开源成为向善力量的贡献者、梦想家和建设者。这是为了你们。

我们公开构建，因为最好的想法来自四面八方。如果你正在阅读这篇文章，你也是其中之一。欢迎。🦀❤️

## 许可证

MIT — 详见 [LICENSE](LICENSE) 了解许可条款和归因基准

## 贡献

详见 [CONTRIBUTING.md](CONTRIBUTING.md)。实现一个 Trait，提交一个 PR：
- CI 工作流指南：[docs/ci-map.md](docs/ci-map.md)
- 新增 `Provider` → `src/providers/`
- 新增 `Channel` → `src/channels/`
- 新增 `Observer` → `src/observability/`
- 新增 `Tool` → `src/tools/`
- 新增 `Memory` → `src/memory/`
- 新增 `Tunnel` → `src/tunnel/`
- 新增 `Skill` → `~/.zeroclaw/workspace/skills/<name>/`

---

**ZeroClaw** — 零开销、零妥协、随地部署、万物可换。🦀

## Star 历史

<p align="center">
  <a href="https://www.star-history.com/#zeroclaw-labs/zeroclaw&type=date&legend=top-left">
    <picture>
     <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=zeroclaw-labs/zeroclaw&type=date&theme=dark&legend=top-left" />
     <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=zeroclaw-labs/zeroclaw&type=date&legend=top-left" />
     <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=zeroclaw-labs/zeroclaw&type=date&legend=top-left" />
    </picture>
  </a>
</p>
