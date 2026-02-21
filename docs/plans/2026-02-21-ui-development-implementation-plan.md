# ZeroClaw UI 开发实施计划

> 生成时间: 2026-02-21
> 目标平台: Desktop Demo (800x600)
> 开发周期: 1周
> 技术栈: Slint UI + Rust

---

## 一、目标概述

在 1 周时间内完成 ZeroClaw Desktop Demo 的核心功能开发：

| 功能 | 优先级 | 状态 |
|------|--------|------|
| 聊天功能 | P0 | 待开发 |
| 记忆管理 | P0 | 待开发 |
| 模型选择 | P1 | 待开发 |
| 技能管理 | P2 | 待开发 |

---

## 二、项目架构

### 2.1 目录结构

```
zeroclaw-ui-demo/
├── ui/
│   ├── main.slint              # 主窗口入口
│   ├── globals.slint           # 全局样式系统
│   ├── structs.slint           # 数据结构定义
│   └── components/
│       ├── status_bar.slint    # 状态栏
│       ├── message_list.slint  # 消息列表
│       ├── message_bubble.slint # 消息气泡
│       ├── input_bar.slint     # 输入栏
│       ├── quick_action_bar.slint # 快捷操作栏
│       └── overlays/
│           ├── menu_overlay.slint    # 设置菜单
│           ├── memory_overlay.slint  # 记忆管理
│           ├── models_overlay.slint  # 模型选择
│           └── skills_overlay.slint  # 技能管理
├── src/
│   ├── main.rs                 # 入口
│   ├── api/
│   │   ├── mod.rs              # API 客户端导出
│   │   ├── client.rs           # HTTP 客户端
│   │   └── models.rs           # 请求/响应结构
│   └── state/
│       ├── mod.rs              # 状态管理导出
│       └── app_state.rs        # 应用状态
├── build.rs                    # Slint 构建脚本
├── Cargo.toml                  # 依赖配置
└── README.md                   # 使用说明
```

### 2.2 模块依赖关系

```
main.rs
    ├── api/client.rs ──────► HTTP 请求
    │       └── api/models.rs
    ├── state/app_state.rs ──► 全局状态
    │       └── api/models.rs
    └── ui/*.slint ──────────► 界面组件
            └── globals.slint
            └── structs.slint
```

---

## 三、样式系统设计

### 3.1 globals.slint

```slint
// 全局样式定义
export global Colors {
    // 背景 (Dark OLED Theme)
    pure-property <color> bg-primary: #0F172A;      // slate-900
    pure-property <color> bg-secondary: #1E293B;    // slate-800
    pure-property <color> bg-tertiary: #334155;     // slate-700
    
    // 强调色
    pure-property <color> accent-primary: #22C55E;   // green-500
    pure-property <color> accent-secondary: #D4AF37; // gold
    pure-property <color> accent-danger: #EF4444;    // red-500
    pure-property <color> accent-warning: #F59E0B;   // amber-500
    
    // 文字
    pure-property <color> text-primary: #F8FAFC;     // slate-50
    pure-property <color> text-secondary: #94A3B8;   // slate-400
    pure-property <color> text-muted: #64748B;       // slate-500
    
    // 边框
    pure-property <color> border-primary: #334155;
    pure-property <color> border-secondary: #475569;
}

export global Typography {
    pure-property <length> h1: 24px;
    pure-property <length> h2: 20px;
    pure-property <length> h3: 18px;
    pure-property <length> body: 16px;
    pure-property <length> small: 14px;
    pure-property <length> caption: 12px;
}

export global Spacing {
    pure-property <length> xs: 4px;
    pure-property <length> sm: 8px;
    pure-property <length> md: 12px;
    pure-property <length> lg: 16px;
    pure-property <length> xl: 20px;
    pure-property <length> xxl: 24px;
}

export global Radius {
    pure-property <length> sm: 4px;
    pure-property <length> md: 8px;
    pure-property <length> lg: 12px;
    pure-property <length> xl: 16px;
    pure-property <length> full: 1000px;
}
```

### 3.2 structs.slint

```slint
// 消息结构
export struct Message {
    id: string,
    role: string,        // "user" | "assistant" | "system"
    content: string,
    timestamp: string,
    status: string,      // "sending" | "sent" | "error"
}

// 记忆条目
export struct MemoryEntry {
    id: string,
    key: string,
    content: string,
    category: string,    // "core" | "daily" | "conversation"
    timestamp: string,
}

// 模型信息
export struct ModelInfo {
    id: string,
    name: string,
    provider: string,
    is-active: bool,
}

// 技能信息
export struct SkillInfo {
    name: string,
    description: string,
    enabled: bool,
    icon: string,
}

// 系统状态
export struct SystemStatus {
    connected: bool,
    current-model: string,
    server-url: string,
    wifi-signal: int,
}
```

---

## 四、API 客户端设计

### 4.1 api/client.rs

```rust
use anyhow::Result;
use reqwest::blocking::Client;
use serde_json::json;

use super::models::*;

pub struct ApiClient {
    base_url: String,
    token: Option<String>,
    client: Client,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            token: None,
            client: Client::new(),
        }
    }
    
    pub fn set_token(&mut self, token: &str) {
        self.token = Some(token.to_string());
    }
    
    // ========== 聊天 API ==========
    
    pub fn send_message(&self, message: &str, model: Option<&str>) -> Result<ChatResponse> {
        let url = format!("{}/v1/chat", self.base_url);
        let body = json!({
            "message": message,
            "model": model,
        });
        
        let response = self.client
            .post(&url)
            .bearer_auth(self.token.as_deref().unwrap_or(""))
            .json(&body)
            .send()?
            .error_for_status()?;
        
        Ok(response.json()?)
    }
    
    // ========== 记忆 API ==========
    
    pub fn list_memories(&self, query: Option<&str>, limit: usize) -> Result<Vec<MemoryEntry>> {
        let mut url = format!("{}/v1/memories?limit={}", self.base_url, limit);
        if let Some(q) = query {
            url.push_str(&format!("&query={}", urlencoding::encode(q)));
        }
        
        let response = self.client
            .get(&url)
            .bearer_auth(self.token.as_deref().unwrap_or(""))
            .send()?
            .error_for_status()?;
        
        Ok(response.json()?)
    }
    
    pub fn delete_memory(&self, key: &str) -> Result<()> {
        let url = format!("{}/v1/memories/{}", self.base_url, key);
        
        self.client
            .delete(&url)
            .bearer_auth(self.token.as_deref().unwrap_or(""))
            .send()?
            .error_for_status()?;
        
        Ok(())
    }
    
    // ========== 模型 API ==========
    
    pub fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let url = format!("{}/v1/models", self.base_url);
        
        let response = self.client
            .get(&url)
            .bearer_auth(self.token.as_deref().unwrap_or(""))
            .send()?
            .error_for_status()?;
        
        Ok(response.json()?)
    }
    
    // ========== 工具/技能 API ==========
    
    pub fn list_tools(&self) -> Result<Vec<SkillInfo>> {
        let url = format!("{}/v1/tools", self.base_url);
        
        let response = self.client
            .get(&url)
            .bearer_auth(self.token.as_deref().unwrap_or(""))
            .send()?
            .error_for_status()?;
        
        Ok(response.json()?)
    }
    
    pub fn toggle_tool(&self, name: &str, enabled: bool) -> Result<()> {
        let url = format!("{}/v1/tools/{}/toggle", self.base_url, name);
        
        self.client
            .post(&url)
            .bearer_auth(self.token.as_deref().unwrap_or(""))
            .json(&json!({ "enabled": enabled }))
            .send()?
            .error_for_status()?;
        
        Ok(())
    }
    
    // ========== 系统 API ==========
    
    pub fn health_check(&self) -> Result<HealthResponse> {
        let url = format!("{}/health", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()?
            .error_for_status()?;
        
        Ok(response.json()?)
    }
}
```

### 4.2 api/models.rs

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub response: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub key: String,
    pub content: String,
    pub category: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInfo {
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub paired: bool,
}
```

---

## 五、状态管理设计

### 5.1 state/app_state.rs

```rust
use std::rc::Rc;
use slint::VecModel;

use crate::api::models::*;

/// 应用全局状态
pub struct AppState {
    // 连接状态
    pub server_url: String,
    pub is_connected: bool,
    
    // 当前选择
    pub current_model: String,
    
    // 数据缓存
    pub messages: Rc<VecModel<Message>>,
    pub memories: Rc<VecModel<MemoryEntry>>,
    pub models: Rc<VecModel<ModelInfo>>,
    pub skills: Rc<VecModel<SkillInfo>>,
    
    // UI 状态
    pub show_menu: bool,
    pub show_memory: bool,
    pub show_models: bool,
    pub show_skills: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            server_url: "http://127.0.0.1:8080".to_string(),
            is_connected: false,
            current_model: String::new(),
            messages: Rc::new(VecModel::default()),
            memories: Rc::new(VecModel::default()),
            models: Rc::new(VecModel::default()),
            skills: Rc::new(VecModel::default()),
            show_menu: false,
            show_memory: false,
            show_models: false,
            show_skills: false,
        }
    }
}
```

---

## 六、每日任务清单

### Day 1：基础架构

#### 上午 (4h)
- [ ] 创建 `ui/` 目录结构
- [ ] 拆分 `ui.slint` 为多个组件文件
- [ ] 创建 `globals.slint` 样式系统
- [ ] 创建 `structs.slint` 数据结构

#### 下午 (4h)
- [ ] 创建 `src/api/` 模块
- [ ] 实现 `ApiClient` 基础结构
- [ ] 实现 `send_message` 方法
- [ ] 测试聊天 API 连接

**验收：** 能发送消息并收到响应

---

### Day 2：聊天功能完善

#### 上午 (4h)
- [ ] 重构 `MessageList` 组件
- [ ] 实现 `MessageBubble` 组件（用户/助手区分）
- [ ] 添加消息状态显示（发送中、已送达、错误）
- [ ] 消息时间戳格式化

#### 下午 (4h)
- [ ] 创建 `AppState` 状态管理
- [ ] 绑定消息列表到真实数据
- [ ] 实现消息发送流程
- [ ] 错误处理和用户提示

**验收：** 完整的聊天流程，包括错误提示

---

### Day 3：记忆管理 - 列表

#### 上午 (4h)
- [ ] 实现 `list_memories` API 方法
- [ ] 重构 `MemoryOverlay` 组件
- [ ] 绑定记忆列表到真实数据
- [ ] 记忆条目样式优化

#### 下午 (4h)
- [ ] 实现记忆搜索功能
- [ ] 搜索框 UI
- [ ] 搜索结果过滤
- [ ] 空状态提示

**验收：** 能查看和搜索记忆

---

### Day 4：记忆管理 - 操作

#### 上午 (4h)
- [ ] 实现记忆删除功能
- [ ] 删除确认对话框
- [ ] 删除后列表更新
- [ ] 错误处理

#### 下午 (4h)
- [ ] 从聊天保存记忆功能
- [ ] 长按消息菜单
- [ ] 保存到记忆 API 调用
- [ ] 成功提示

**验收：** 能删除记忆和从聊天保存

---

### Day 5：模型选择

#### 上午 (4h)
- [ ] 实现 `list_models` API 方法
- [ ] 重构 `ModelsOverlay` 组件
- [ ] 绑定模型列表到真实数据
- [ ] 模型选择 UI

#### 下午 (4h)
- [ ] 实现模型切换功能
- [ ] 当前模型高亮显示
- [ ] 模型选择后更新聊天
- [ ] 配置持久化（可选）

**验收：** 能切换模型并在聊天中生效

---

### Day 6：技能管理

#### 上午 (4h)
- [ ] 实现 `list_tools` 和 `toggle_tool` API
- [ ] 重构 `SkillsOverlay` 组件
- [ ] 绑定技能列表到真实数据
- [ ] Toggle 开关组件

#### 下午 (4h)
- [ ] 技能开关功能
- [ ] 状态持久化
- [ ] 错误处理
- [ ] UI 细节优化

**验收：** 能启用/禁用技能

---

### Day 7：测试与文档

#### 上午 (4h)
- [ ] 全功能测试
- [ ] 网络错误处理测试
- [ ] 边界情况测试
- [ ] Bug 修复

#### 下午 (4h)
- [ ] 编写 README.md
- [ ] 添加使用截图
- [ ] 代码注释完善
- [ ] 最终验收

**验收：** 所有功能正常，文档完整

---

## 七、验收标准

### 7.1 功能验收

| 功能 | 验收标准 | 通过标准 |
|------|----------|----------|
| 聊天 | 发送消息 → 收到回复 → 正确显示 | ✅ |
| 消息状态 | 显示发送中/已送达/错误 | ✅ |
| 记忆列表 | 显示所有记忆条目 | ✅ |
| 记忆搜索 | 关键词匹配正确 | ✅ |
| 记忆删除 | 删除成功，列表更新 | ✅ |
| 模型列表 | 显示所有可用模型 | ✅ |
| 模型切换 | 切换后聊天使用新模型 | ✅ |
| 技能列表 | 显示所有技能状态 | ✅ |
| 技能开关 | 切换成功，状态保存 | ✅ |

### 7.2 性能验收

| 指标 | 目标 |
|------|------|
| 启动时间 | < 2s |
| 消息发送延迟 | < 500ms |
| API 响应时间 | < 200ms |
| 内存占用 | < 100MB |

### 7.3 UI 验收

- [ ] 颜色符合设计系统
- [ ] 字体清晰可读（≥12px）
- [ ] 间距一致
- [ ] 暗色主题一致性
- [ ] 错误提示友好

---

## 八、风险与应对

| 风险 | 可能性 | 应对策略 |
|------|--------|----------|
| API 接口变更 | 中 | 封装 API 层隔离变化 |
| 网络不稳定 | 高 | 添加重试机制和错误提示 |
| 时间不足 | 中 | 优先完成 P0 功能 |

---

## 九、依赖配置

### Cargo.toml

```toml
[package]
name = "zeroclaw-ui-demo"
version = "0.1.0"
edition = "2021"

[build-dependencies]
slint-build = "1.10"

[dependencies]
slint = "1.10"
reqwest = { version = "0.11", features = ["blocking", "json", "rustls-tls"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
chrono = "0.4"
urlencoding = "2.1"
```

### build.rs

```rust
fn main() {
    slint_build::compile("ui/main.slint").unwrap();
}
```

---

## 十、后续扩展

完成本次 1 周计划后，可考虑：

1. **ESP32 适配** - 将组件移植到嵌入式平台
2. **WebSocket 实时通信** - 替代 HTTP 轮询
3. **离线模式** - 本地缓存和队列
4. **多语言支持** - i18n 国际化
5. **主题切换** - 亮色/暗色模式

---

*本计划基于三个设计文档整合而成，聚焦 1 周快速交付核心功能。*
