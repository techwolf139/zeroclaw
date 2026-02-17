# ZeroClaw API 验证文档

## 已部署的 API 端点

### 1. 基础端点

| 端点 | 方法 | 认证 | 描述 |
|------|------|------|------|
| `/health` | GET | ❌ | 健康检查 |
| `/pair` | POST | ❌ | 配对认证 |

### 2. Webhook 端点

| 端点 | 方法 | 认证 | 描述 |
|------|------|------|------|
| `/webhook` | POST | Bearer | 发送消息到 AI |
| `/whatsapp` | GET | Query | WhatsApp 验证 |
| `/whatsapp` | POST | ❌ | WhatsApp 消息 |

### 3. V1 API - 服务信息

| 端点 | 方法 | 认证 | 描述 |
|------|------|------|------|
| `/v1/stats` | GET | ❌ | 服务统计信息 |
| `/v1/models` | GET | ❌ | 可用模型列表 |

### 4. V1 API - 内存管理

| 端点 | 方法 | 认证 | 描述 |
|------|------|------|------|
| `/v1/memories` | GET | ❌ | 列出记忆 |
| `/v1/memories` | POST | ❌ | 创建记忆 |
| `/v1/memories/:key` | GET | ❌ | 获取记忆 |
| `/v1/memories/:key` | DELETE | ❌ | 删除记忆 |

**参数：**
- GET `/v1/memories?query=search&limit=10`
- POST `/v1/memories` - Body: `{"key": "...", "content": "...", "category": "conversation"}`

### 5. V1 API - AI 对话 ⭐ NEW

| 端点 | 方法 | 认证 | 描述 |
|------|------|------|------|
| `/v1/chat` | POST | Bearer | 与 AI 对话 |

**请求体：**
```json
{
  "message": "Hello AI",
  "model": "claude-sonnet-4-20250514",
  "temperature": 0.7
}
```

**响应：**
```json
{
  "response": "AI 回复内容",
  "model": "claude-sonnet-4-20250514"
}
```

### 6. V1 API - 工具执行 ⭐ NEW

| 端点 | 方法 | 认证 | 描述 |
|------|------|------|------|
| `/v1/tools/execute` | POST | Bearer | 执行工具 |

**支持的工具：**

| 工具 | 参数 | 描述 |
|------|------|------|
| `shell` / `bash` | `command` | 执行 Shell 命令 |
| `file_read` / `read` | `path` | 读取文件 |
| `file_write` / `write` | `path`, `content` | 写入文件 |
| `memory_store` | `key`, `content`, `category` | 存储记忆 |
| `memory_recall` / `recall` | `query`, `limit` | 搜索记忆 |
| `memory_forget` / `forget` | `key` | 删除记忆 |

**请求体：**
```json
{
  "tool": "shell",
  "params": {
    "command": "ls -la"
  }
}
```

### 7. V1 API - 渠道管理 ⭐ NEW

| 端点 | 方法 | 认证 | 描述 |
|------|------|------|------|
| `/v1/channels` | GET | ❌ | 列出可用渠道 |
| `/v1/channels/:name/send` | POST | Bearer | 发送消息 |

**请求体 (发送消息)：**
```json
{
  "recipient": "user_id",
  "message": "Hello!"
}
```

---

## 测试命令

### 1. 健康检查
```bash
curl http://localhost:8080/health
```

### 2. 服务统计
```bash
curl http://localhost:8080/v1/stats
```

### 3. 模型列表
```bash
curl http://localhost:8080/v1/models
```

### 4. AI 对话
```bash
curl -X POST http://localhost:8080/v1/chat \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Hello, what can you do?",
    "temperature": 0.7
  }'
```

### 5. 执行 Shell 命令
```bash
curl -X POST http://localhost:8080/v1/tools/execute \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "tool": "shell",
    "params": {
      "command": "pwd"
    }
  }'
```

### 6. 存储记忆
```bash
curl -X POST http://localhost:8080/v1/memories \
  -H "Content-Type: application/json" \
  -d '{
    "key": "test_memory",
    "content": "This is a test memory",
    "category": "conversation"
  }'
```

### 7. 列出渠道
```bash
curl http://localhost:8080/v1/channels
```

---

## 验证清单

- [x] `/health` - 健康检查
- [x] `/pair` - 配对认证
- [x] `/webhook` - Webhook
- [x] `/v1/stats` - 服务统计
- [x] `/v1/models` - 模型列表
- [x] `/v1/chat` - AI 对话
- [x] `/v1/memories` - 内存 CRUD
- [x] `/v1/tools/execute` - 工具执行
- [x] `/v1/channels` - 渠道列表
- [x] `/v1/channels/:name/send` - 发送消息

## 总结

✅ **所有核心功能接口已成功部署为外部 API**

共实现 **15 个 API 端点**，覆盖：
1. ✅ 基础服务（健康检查、配对）
2. ✅ Webhook（接收消息）
3. ✅ 服务信息（统计、模型）
4. ✅ AI 对话（/v1/chat）
5. ✅ 内存管理（CRUD）
6. ✅ 工具执行（Shell、文件、记忆）
7. ✅ 渠道管理（列表、发送）
