# ZeroClaw 项目结构文档

**生成日期**: 2025年
**项目版本**: 0.1.0
**作者**: theonlyhennygod

---

## 项目概述

**ZeroClaw** 是一个用 100% Rust 编写的高性能 AI 助手框架，主打"零开销、零妥协"的设计理念。

### 核心特点
- **极致轻量**: 运行时 < 5MB RAM，二进制文件约 3.4MB
- **快速启动**: 在 0.8GHz 边缘设备上 < 10ms 启动
- **硬件友好**: 可在 $10 的低端硬件上运行（比 OpenClaw 少 99% 内存，便宜 98%）
- **完全可插拔**: 所有核心系统基于 Trait 架构，支持零代码更换实现
- **安全优先**: 默认启用沙箱、配对验证、白名单和加密存储

### 项目定位
ZeroClaw 是为需要低资源消耗、高安全性、可部署在任何环境的 AI 助手应用场景而设计的。支持从边缘设备到云端的广泛部署场景。

---

## 技术栈

### 核心语言与工具
| 组件 | 技术 | 说明 |
|------|------|------|
| 编程语言 | **Rust** (Edition 2021) | 100% Rust 实现 |
| 异步运行时 | **Tokio** | 多线程异步运行时 |
| CLI 框架 | **clap** | 派生宏驱动的命令行解析 |
| HTTP 客户端 | **reqwest** | 基于 rustls-tls 的 HTTP 客户端 |
| 序列化 | **serde** + **serde_json** | JSON 序列化/反序列化 |
| 配置管理 | **toml** + **directories** | TOML 配置文件 + 标准目录 |

### 关键依赖库
| 功能 | 库 | 用途 |
|------|-----|------|
| Web 服务器 | **axum** + **tower** | Gateway HTTP 服务和 WebSocket |
| 数据库 | **rusqlite** | SQLite 嵌入式数据库（向量+FTS5） |
| 加密 | **chacha20poly1305** | 密钥加密存储（AEAD） |
| 认证 | **ring** + **hmac** + **sha2** | JWT 和 webhook 签名验证 |
| WebSocket | **tokio-tungstenite** | Discord 等实时通道 |
| 邮件 | **lettre** + **mail-parser** | SMTP 邮件收发 |
| 可观测性 | **opentelemetry** + **prometheus** | 追踪和指标收集 |
| 协议缓冲 | **prost** | Feishu WebSocket 帧编解码 |

### 嵌入式/硬件支持
| 功能 | 库 | 说明 |
|------|-----|------|
| USB 设备 | **nusb** | USB 设备枚举 |
| 串口通信 | **tokio-serial** | 外设串口通信 |
| 调试器 | **probe-rs** | STM32/Nucleo 内存读取 |
| GPIO (Linux) | **rppal** | 树莓派 GPIO 控制 |
| 沙箱 (Linux) | **landlock** | Linux Landlock LSM |

### 可选功能特性
- `hardware` - 启用硬件发现功能
- `browser-native` - Rust 原生浏览器自动化（fantoccini）
- `sandbox-landlock` - Linux Landlock 沙箱
- `probe` - STM32 调试支持
- `rag-pdf` - PDF 文档 RAG 支持

---

## 目录结构

```
~/git/zeroclaw/
├── Cargo.toml              # 项目配置与依赖
├── Cargo.lock              # 依赖锁定文件
├── README.md               # 项目主文档（多语言版本）
├── LICENSE                 # MIT 许可证
├── NOTICE                  # 贡献者声明
├── CHANGELOG.md            # 变更日志
├── CONTRIBUTING.md         # 贡献指南
├── SECURITY.md             # 安全披露政策
│
├── src/                    # 主源代码目录（约 161 个 .rs 文件，~90K 行）
│   ├── main.rs             # CLI 入口点
│   ├── lib.rs              # 库入口点
│   ├── util.rs             # 通用工具函数
│   ├── identity.rs         # 身份系统（AIEOS/OpenClaw）
│   │
│   ├── agent/              # AI 代理核心
│   ├── approval/           # 操作审批系统
│   ├── auth/               # 认证管理（OAuth/Token/API Key）
│   ├── channels/           # 通信渠道（15+ 种消息平台）
│   ├── config/             # 配置管理
│   ├── cost/               # 成本追踪
│   ├── cron/               # 定时任务调度
│   ├── daemon/             # 守护进程管理
│   ├── doctor/             # 系统诊断
│   ├── gateway/            # HTTP Webhook 网关
│   ├── hardware/           # 硬件发现与管理
│   ├── health/             # 健康检查
│   ├── heartbeat/          # 心跳任务引擎
│   ├── integrations/       # 集成注册表
│   ├── memory/             # 内存/记忆系统
│   ├── observability/      # 可观测性（日志/指标/追踪）
│   ├── onboard/            # 初始化引导向导
│   ├── peripherals/        # 外设管理（Arduino/STM32/ESP32）
│   ├── providers/          # AI 提供商（28+ 内置）
│   ├── rag/                # RAG 检索增强生成
│   ├── runtime/            # 运行时适配器（Native/Docker/WASM）
│   ├── security/           # 安全策略与沙箱
│   ├── service/            # 系统服务管理
│   ├── skillforge/         # 技能开发框架
│   ├── skills/             # 技能系统
│   ├── tools/              # 工具集（30+ 个工具）
│   └── tunnel/             # 隧道服务（ngrok/Cloudflare/Tailscale）
│
├── crates/                 # 工作区子 crate
│   └── robot-kit/          # 机器人硬件套件库
│
├── firmware/               # 嵌入式固件
│   ├── zeroclaw-arduino/   # Arduino 固件（.ino）
│   ├── zeroclaw-esp32/     # ESP32 固件（Rust/ESP-IDF）
│   ├── zeroclaw-esp32-ui/  # ESP32 + Slint UI
│   ├── zeroclaw-nucleo/    # STM32 Nucleo 固件
│   └── zeroclaw-uno-q-bridge/  # Arduino UNO-Q 桥接
│
├── docs/                   # 文档目录
│   ├── README.md           # 文档中心入口
│   ├── SUMMARY.md          # 统一目录
│   ├── architecture.svg    # 架构图
│   ├── commands-reference.md    # 命令参考
│   ├── config-reference.md      # 配置参考
│   ├── providers-reference.md   # 提供商参考
│   ├── channels-reference.md    # 通道参考
│   ├── operations-runbook.md    # 运维手册
│   ├── troubleshooting.md       # 故障排查
│   ├── security/           # 安全文档
│   ├── hardware/           # 硬件文档
│   ├── contributing/       # 贡献文档
│   ├── getting-started/    # 入门指南
│   ├── operations/         # 运维文档
│   ├── reference/          # 参考文档
│   ├── project/            # 项目状态
│   └── datasheets/         # 硬件数据手册
│
├── python/                 # Python 配套包
│   ├── zeroclaw_tools/     # Python 工具库
│   │   ├── tools/          # Python 工具实现
│   │   ├── integrations/   # Discord Bot 等集成
│   │   └── agent.py        # LangGraph 代理
│   └── tests/              # Python 测试
│
├── examples/               # Rust 示例代码
│   ├── custom_channel.rs   # 自定义通道示例
│   ├── custom_memory.rs    # 自定义内存后端示例
│   ├── custom_provider.rs  # 自定义提供商示例
│   └── custom_tool.rs      # 自定义工具示例
│
├── tests/                  # 集成测试
│   ├── agent_e2e.rs        # 端到端代理测试
│   ├── memory_comparison.rs # 内存基准测试
│   └── ...
│
├── benches/                # 性能基准测试
│   └── agent_benchmarks.rs
│
├── scripts/                # 脚本工具
│   ├── bootstrap.sh        # 一键引导脚本
│   ├── install.sh          # 安装脚本
│   └── ci/                 # CI/CD 脚本
│
├── dev/                    # 开发工具
│   ├── docker-compose.yml  # 开发环境
│   ├── config.template.toml # 配置模板
│   └── sandbox/            # 沙箱配置
│
├── fuzz/                   # 模糊测试
│   └── fuzz_targets/
│
└── test_helpers/           # 测试辅助工具
```

---

## 主要模块说明

### 1. `agent/` - AI 代理核心
**文件**: `src/agent/mod.rs`, `agent.rs`, `loop_.rs`, `dispatcher.rs`, `classifier.rs`, `prompt.rs`, `memory_loader.rs`

实现 ZeroClaw 的核心 AI 代理逻辑：
- **Agent**: 代理状态和执行上下文
- **loop_**: 主消息处理循环
- **dispatcher**: 工具调用分派器
- **classifier**: 意图分类器
- **prompt**: 系统提示词生成
- **memory_loader**: 记忆加载器

### 2. `channels/` - 通信渠道层
**文件**: `src/channels/mod.rs`, `traits.rs`, 以及各渠道实现

支持 15+ 种消息平台：
| 渠道 | 文件 | 协议 |
|------|------|------|
| CLI | `cli.rs` | 本地终端 |
| Telegram | `telegram.rs` | HTTP API + Webhook |
| Discord | `discord.rs` | Gateway WebSocket |
| Slack | `slack.rs` | Events API |
| WhatsApp | `whatsapp.rs` | Meta Cloud API |
| Matrix | `matrix.rs` | Client-Server API |
| Signal | `signal.rs` | Signal API |
| iMessage | `imessage.rs` | macOS 桥接 |
| Email | `email_channel.rs` | SMTP/IMAP |
| IRC | `irc.rs` | IRC 协议 |
| Lark/Feishu | `lark.rs` | 飞书 API + WebSocket |
| DingTalk | `dingtalk.rs` | 钉钉 API |
| QQ | `qq.rs` | QQ Bot API |
| Mattermost | `mattermost.rs` | Mattermost API |

**核心 Trait**: `Channel` - 定义发送/接收消息的通用接口

### 3. `memory/` - 记忆系统
**文件**: `src/memory/mod.rs`, `sqlite.rs`, `vector.rs`, `embeddings.rs`, `chunker.rs`, `fts.rs` 等

ZeroClaw 实现了一套**零外部依赖**的完整搜索栈：

| 层级 | 实现 | 说明 |
|------|------|------|
| 向量数据库 | SQLite BLOB + 余弦相似度 | 无需 Pinecone/Weaviate |
| 关键词搜索 | FTS5 虚拟表 | BM25 评分 |
| 混合检索 | 自定义加权合并 | vector_weight + keyword_weight |
| 分块 | 基于 Markdown 标题 | 保留文档结构 |
| 缓存 | SQLite LRU 缓存 | embedding_cache 表 |

**支持的 Backend**:
- `sqlite` (默认) - SQLite 混合搜索
- `lucid` - Lucid 外部记忆桥接
- `markdown` - Markdown 文件存储
- `none` - 显式无持久化

### 4. `tools/` - 工具集
**文件**: `src/tools/` 目录下 31 个工具实现

| 类别 | 工具 | 功能 |
|------|------|------|
| **文件** | `file_read.rs` | 读取文件内容 |
| | `file_write.rs` | 写入文件 |
| **Shell** | `shell.rs` | 执行 shell 命令 |
| **Git** | `git_operations.rs` | Git 操作 |
| **内存** | `memory_store.rs` | 存储记忆 |
| | `memory_recall.rs` | 回忆记忆 |
| | `memory_forget.rs` | 遗忘记忆 |
| **定时任务** | `cron_add.rs`, `cron_list.rs`, `cron_remove.rs`, `cron_run.rs`, `cron_update.rs` | Cron 任务管理 |
| **浏览器** | `browser.rs`, `browser_open.rs` | 浏览器自动化 |
| **网络** | `http_request.rs` | HTTP 请求 |
| | `web_search_tool.rs` | 网页搜索 |
| **图像** | `screenshot.rs`, `image_info.rs` | 截图和图像分析 |
| **硬件** | `hardware_board_info.rs`, `hardware_memory_map.rs`, `hardware_memory_read.rs` | 硬件信息读取 |
| **其他** | `pushover.rs` | Pushover 通知 |
| | `composio.rs` | Composio 集成 |
| | `delegate.rs` | 代理调用 |

**核心 Trait**: `Tool` - 定义工具的 schema 和执行接口

### 5. `providers/` - AI 提供商
**文件**: `src/providers/` 目录下 15+ 个提供商实现

支持 28+ 个内置提供商和别名：

| 提供商 | 文件 | 特点 |
|--------|------|------|
| OpenAI | `openai.rs` | GPT-4/GPT-3.5 |
| Anthropic | `anthropic.rs` | Claude 系列 |
| Gemini | `gemini.rs` | Google Gemini |
| Ollama | `ollama.rs` | 本地模型 |
| OpenRouter | `openrouter.rs` | 多模型路由 |
| Zhipu/GLM | `glm.rs` | 智谱 AI |
| Copilot | `copilot.rs` | GitHub Copilot |
| OpenAI Codex | `openai_codex.rs` | Codex CLI 兼容 |

**核心 Trait**: `Provider` - 定义聊天完成和流式响应接口

**自定义端点**: 支持 `custom:https://your-api.com` 和 `anthropic-custom:https://...`

### 6. `security/` - 安全系统
**文件**: `src/security/` 目录

| 模块 | 文件 | 功能 |
|------|------|------|
| 配对系统 | `pairing.rs` | 6 位一次性配对码 |
| 沙箱 | `bubblewrap.rs`, `firejail.rs`, `landlock.rs`, `docker.rs` | 多级沙箱 |
| 审计 | `audit.rs` | 操作审计日志 |
| 密钥 | `secrets.rs` | 加密密钥存储（AEAD）|
| 策略 | `policy.rs` | 安全策略定义 |
| 检测 | `detect.rs` | 威胁检测 |

**安全清单**:
- ✅ 网关默认绑定 127.0.0.1
- ✅ 强制配对验证
- ✅ 工作区隔离（禁止访问 /etc, /root 等）
- ✅ 隧道强制（拒绝公开绑定）
- ✅ 空白名单 = 拒绝所有入站

### 7. `hardware/` - 硬件支持
**文件**: `src/hardware/` 目录

- `discover.rs` - USB/串口设备发现
- `introspect.rs` - 硬件自检
- `registry.rs` - 硬件注册表

### 8. `peripherals/` - 外设管理
**文件**: `src/peripherals/` 目录

支持多种开发板和外设：
- Arduino（通过 avrdude 烧录）
- STM32 Nucleo（通过 probe-rs）
- Raspberry Pi GPIO
- Arduino UNO-Q 桥接器

### 9. `runtime/` - 运行时适配器
**文件**: `src/runtime/` 目录

| 运行时 | 文件 | 状态 |
|--------|------|------|
| Native | `native.rs` | ✅ 支持 |
| Docker | `docker.rs` | ✅ 支持（沙箱化）|
| WASM | `wasm.rs` | 🚧 计划 |

### 10. `cron/` - 定时任务
**文件**: `src/cron/` 目录

完整的 Cron 调度系统：
- `scheduler.rs` - 任务调度器
- `schedule.rs` - 调度表达式解析
- `store.rs` - 任务持久化
- `types.rs` - 类型定义

### 11. `observability/` - 可观测性
**文件**: `src/observability/` 目录

| 后端 | 文件 | 用途 |
|------|------|------|
| Noop | `noop.rs` | 无操作（默认）|
| Log | `log.rs` | 结构化日志 |
| Multi | `multi.rs` | 多路复用 |
| Prometheus | `prometheus.rs` | 指标收集 |
| OpenTelemetry | `otel.rs` | 分布式追踪 |

### 12. `tunnel/` - 隧道服务
**文件**: `src/tunnel/` 目录

支持多种隧道方案：
- Cloudflare Tunnel
- Tailscale Funnel
- ngrok
- 自定义隧道

### 13. `auth/` - 认证管理
**文件**: `src/auth/` 目录

支持多种认证方式：
- API Key
- OAuth 2.0（OpenAI Codex 设备码流）
- 订阅 Token（Anthropic）
- 多账号配置文件

### 14. `gateway/` - HTTP 网关
**文件**: `src/gateway/mod.rs`

基于 Axum 的 Web 服务器：
- `/health` - 健康检查
- `/pair` - 设备配对
- `/webhook` - 入站消息
- `/whatsapp` - WhatsApp Meta webhook

### 15. `firmware/` - 嵌入式固件

独立的嵌入式项目：

| 项目 | 路径 | 平台 | 说明 |
|------|------|------|------|
| zeroclaw-arduino | `firmware/zeroclaw-arduino/` | Arduino | C++ 固件 |
| zeroclaw-esp32 | `firmware/zeroclaw-esp32/` | ESP32 | Rust/ESP-IDF |
| zeroclaw-esp32-ui | `firmware/zeroclaw-esp32-ui/` | ESP32 | Slint UI |
| zeroclaw-nucleo | `firmware/zeroclaw-nucleo/` | STM32 | Rust/embedded-hal |
| zeroclaw-uno-q-bridge | `firmware/zeroclaw-uno-q-bridge/` | Arduino UNO-Q | 桥接固件 |

### 16. `python/` - Python 配套包

`zeroclaw-tools` 包为 LLM 提供商提供**LangGraph 驱动的工具调用**：

```python
from zeroclaw_tools import create_agent, shell, file_read
agent = create_agent(tools=[shell, file_read], model="glm-5")
```

特别适用于原生工具调用不稳定的提供商（如 GLM-5/Zhipu）。

---

## 架构概览

### Trait 驱动架构

ZeroClaw 的每个子系统都基于 Trait 定义，实现完全可插拔：

| 子系统 | Trait | 默认实现 |
|--------|-------|----------|
| AI 模型 | `Provider` | 28+ 内置提供商 |
| 通信渠道 | `Channel` | 15+ 内置渠道 |
| 记忆存储 | `Memory` | SQLite/Lucid/Markdown |
| 工具 | `Tool` | 30+ 内置工具 |
| 可观测性 | `Observer` | Log/Multi/Prometheus/OTel |
| 运行时 | `RuntimeAdapter` | Native/Docker |
| 安全策略 | `SecurityPolicy` | 配对/沙箱/白名单 |
| 隧道 | `Tunnel` | Cloudflare/Tailscale/ngrok |

### 配置驱动

所有子系统通过 `~/.zeroclaw/config.toml` 配置，无需代码更改即可更换实现：

```toml
default_provider = "openrouter"
default_model = "anthropic/claude-sonnet-4-6"

[memory]
backend = "sqlite"  # 可改为 "lucid" 或 "markdown"

[runtime]
kind = "native"     # 可改为 "docker"

[tunnel]
provider = "none"   # 可改为 "cloudflare" 等
```

---

## 文档索引

### 快速导航
| 需求 | 文档 |
|------|------|
| 快速开始 | `README.md`, `docs/getting-started/README.md` |
| 命令参考 | `docs/commands-reference.md` |
| 配置参考 | `docs/config-reference.md` |
| 提供商列表 | `docs/providers-reference.md` |
| 渠道设置 | `docs/channels-reference.md` |
| 运维手册 | `docs/operations-runbook.md` |
| 故障排查 | `docs/troubleshooting.md` |
| 安全设计 | `docs/security/README.md` |
| 硬件指南 | `docs/hardware/README.md` |
| 贡献指南 | `CONTRIBUTING.md` |

### 完整目录
详见 `docs/SUMMARY.md`

---

## 开发统计

| 指标 | 数值 |
|------|------|
| Rust 源文件 | ~161 个 |
| Rust 代码行数 | ~90,000 行 |
| 核心模块 | 34 个 |
| 内置工具 | 30+ 个 |
| 内置渠道 | 15+ 个 |
| 内置提供商 | 28+ 个 |
| 文档文件 | 40+ 个 |
| 嵌入式固件项目 | 5 个 |

---

## 许可证

MIT License - 详见 `LICENSE` 文件

---

*本文档由项目结构分析工具自动生成*
