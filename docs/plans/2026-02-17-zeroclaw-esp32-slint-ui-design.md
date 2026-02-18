# ZeroClaw + Slint UI + ESP32 实施计划

**创建日期**: 2026-02-17  
**项目**: ZeroClaw ESP32 图形界面固件  
**目标**: 在 ESP32 上实现 Slint UI，为 ZeroClaw AI 助手提供本地图形界面

## 1. 背景与目标

### 1.1 项目背景

ZeroClaw 是一个用 Rust 编写的轻量级 AI 助手运行时，设计用于在资源受限的硬件上运行。当前 ZeroClaw 通过串口 (JSON-over-serial) 与 ESP32 通信，实现 GPIO 控制等硬件功能。本项目旨在扩展这一架构，在 ESP32 上运行完整的 Slint 图形界面，提供：

- 本地聊天界面
- 实时状态显示
- 触摸交互支持

### 1.2 技术选型

| 组件 | 选择 | 理由 |
|------|------|------|
| UI 框架 | Slint | 官方支持 ESP32，内存占用 < 300KB |
| 显示屏 | ST7789 | Rust 驱动成熟，320x240 分辨率 |
| 芯片 | ESP32-S3 | 双核 240MHz，可选 PSRAM |
| 通信 | WiFi + HTTP | 与 ZeroClaw Gateway 无缝集成 |

## 2. 系统架构

### 2.1 整体架构

```
┌─────────────────────────────────────────────────────────────────┐
│                      ZeroClaw ESP32 UI Firmware                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                     Slint UI Layer                        │   │
│  │  ┌────────────┐  ┌─────────────┐  ┌──────────────────┐   │   │
│  │  │ StatusBar  │  │ MessageList │  │    InputBar     │   │   │
│  │  └────────────┘  └─────────────┘  └──────────────────┘   │   │
│  └──────────────────────────────────────────────────────────┘   │
│                              │                                   │
│                              ▼                                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                     App Logic Layer                       │   │
│  │  ┌────────────┐  ┌─────────────┐  ┌──────────────────┐   │   │
│  │  │  Protocol  │  │  WiFi Mgr   │  │  ZeroClaw Client │   │   │
│  │  └────────────┘  └─────────────┘  └──────────────────┘   │   │
│  └──────────────────────────────────────────────────────────┘   │
│                              │                                   │
│                              ▼                                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                     HAL Layer                              │   │
│  │  ┌────────────┐  ┌─────────────┐  ┌──────────────────┐   │   │
│  │  │  esp-hal   │  │  mipidsi    │  │   display-i/f   │   │   │
│  │  └────────────┘  └─────────────┘  └──────────────────┘   │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
                               │
                               │ WiFi / HTTP
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                    ZeroClaw Gateway (Host)                       │
│                         127.0.0.1:8080                          │
│                                                                  │
│  /health  /pair  /webhook  /v1/chat  /v1/tools  ...           │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 模块说明

#### 2.2.1 Slint UI (ui/main.slint)

声明式界面定义，包含：
- **MainWindow**: 根容器，320x240 像素
- **StatusBar**: 连接状态和应用标题
- **MessageList**: 聊天消息列表
- **InputBar**: 文本输入和发送按钮

#### 2.2.2 协议层 (src/protocol.rs)

```rust
// 消息结构
struct ChatMessage {
    role: MessageRole,      // user | assistant | system
    content: String<512>,   // 消息内容
    timestamp: String<32>,  // 时间戳
}

// UI 状态
struct UiStatus {
    connected: bool,
    wifi_ssid: Option<String<32>>,
    zeroclaw_connected: bool,
}
```

#### 2.2.3 WiFi 管理 (src/wifi.rs)

- 异步 WiFi 连接
- 自动重连逻辑
- 信号强度监控

#### 2.2.4 ZeroClaw 客户端 (src/client.rs)

- HTTP POST 到 `/webhook` 端点
- Bearer Token 认证
- 健康检查 `/health`

## 3. 硬件规格

### 3.1 推荐硬件配置

| 组件 | 规格 | 备注 |
|------|------|------|
| MCU | ESP32-S3 | Xtensa LX7 双核 240MHz |
| RAM | 512KB + 8MB PSRAM | 推荐，可选 |
| Flash | 4MB+ | 固件约 1.5MB |
| 显示屏 | 2.8" TFT 320x240 | ST7789 控制器 |
| 触摸 | XPT2046 / FT6X36 | 可选 |

### 3.2 引脚配置

| 功能 | GPIO | 说明 |
|------|------|------|
| SPI SCK | GPIO6 | 时钟 |
| SPI MOSI | GPIO7 | 数据输出 |
| SPI MISO | GPIO8 | 数据输入 (可选) |
| SPI CS | GPIO10 | 片选 |
| DC | GPIO4 | 数据/命令 |
| RST | GPIO3 | 复位 |
| Backlight | GPIO5 | 背光控制 |

## 4. 实施计划

### 阶段 1: 基础框架 (已完成 ✅)

- [x] 项目结构创建
- [x] Cargo.toml 依赖配置
- [x] Slint UI 定义
- [x] 基础显示驱动集成
- [x] 协议模块
- [x] WiFi 管理模块
- [x] HTTP 客户端模块

### 阶段 2: 集成与测试 (待完成)

- [ ] 更新 main.rs 集成所有模块
- [ ] 添加消息队列
- [ ] 实现 UI 回调连接
- [ ] 本地测试 (模拟器)
- [ ] 硬件测试

### 阶段 3: 优化与 polish (待完成)

- [ ] 内存优化
- [ ] 错误处理增强
- [ ] 电源管理
- [ ] 文档完善

## 5. 构建与部署

### 5.1 构建命令

```bash
# 进入固件目录
cd firmware/zeroclaw-esp32-ui

# 构建发布版本
cargo build --release

# 烧录到设备
cargo espflash flash --release --monitor
```

### 5.2 功能开关

```bash
# 使用 ILI9341 显示屏
cargo build --features display-ili9341

# 启用 WiFi
cargo build --features wifi

# 启用触摸
cargo build --features touch-xpt2046
```

## 6. 已知限制

1. **内存限制**: ESP32-C3 (400KB RAM) 仅支持简化 UI
2. **显示分辨率**: 480x320 需要 PSRAM
3. **网络依赖**: 需要 WiFi 连接才能与 ZeroClaw 通信

## 7. 扩展方向

- 离线模式: 本地小型模型 (需额外存储)
- 蓝牙: BLE 配网和通信
- 电池: 电源管理和低功耗模式

## 8. 参考资源

- [Slint ESP32 官方文档](https://slint.dev/esp32)
- [ESP-IDF Rust 书籍](https://esp-rs.github.io/book/)
- [mipidsi 显示屏驱动](https://crates.io/crates/mipidsi)
- [ZeroClaw 硬件设计文档](../hardware-peripherals-design.md)
