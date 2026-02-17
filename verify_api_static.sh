#!/bin/bash
# API 端点静态验证脚本

echo "======================================"
echo "ZeroClaw API 静态验证"
echo "======================================"
echo ""

# 定义颜色
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# 检查文件
check_file="/Users/mac/git/zeroclaw/.worktrees/service-api/src/gateway/mod.rs"

# 期望的路由
routes=(
    "/health:GET:handle_health"
    "/pair:POST:handle_pair"
    "/webhook:POST:handle_webhook"
    "/whatsapp:GET:handle_whatsapp_verify"
    "/whatsapp:POST:handle_whatsapp_message"
    "/v1/stats:GET:handle_v1_stats"
    "/v1/models:GET:handle_v1_models"
    "/v1/memories:GET:handle_v1_memories_list"
    "/v1/memories:POST:handle_v1_memories_create"
    "/v1/memories/:key:GET:handle_v1_memory_get"
    "/v1/memories/:key:DELETE:handle_v1_memory_delete"
    "/v1/chat:POST:handle_v1_chat"
    "/v1/tools/execute:POST:handle_v1_tools_execute"
    "/v1/channels:GET:handle_v1_channels_list"
    "/v1/channels/:name/send:POST:handle_v1_channels_send"
)

# 期望的处理器
handlers=(
    "handle_health"
    "handle_pair"
    "handle_webhook"
    "handle_whatsapp_verify"
    "handle_whatsapp_message"
    "handle_v1_stats"
    "handle_v1_models"
    "handle_v1_memories_list"
    "handle_v1_memories_create"
    "handle_v1_memory_get"
    "handle_v1_memory_delete"
    "handle_v1_chat"
    "handle_v1_tools_execute"
    "handle_v1_channels_list"
    "handle_v1_channels_send"
)

echo "1. 验证路由定义"
echo "--------------------------------------"
routes_found=0
for route in "${routes[@]}"; do
    IFS=':' read -r path method handler <<< "$route"
    if grep -q "\.route(\"$path\", $method($handler))" "$check_file"; then
        echo -e "${GREEN}✓${NC} $method $path -> $handler"
        ((routes_found++))
    else
        echo -e "${RED}✗${NC} $method $path -> $handler (未找到)"
    fi
done
echo ""

echo "2. 验证处理器函数"
echo "--------------------------------------"
handlers_found=0
for handler in "${handlers[@]}"; do
    if grep -q "async fn $handler(" "$check_file"; then
        echo -e "${GREEN}✓${NC} fn $handler"
        ((handlers_found++))
    else
        echo -e "${RED}✗${NC} fn $handler (未找到)"
    fi
done
echo ""

echo "3. 验证结构体定义"
echo "--------------------------------------"
structs=("ChatRequest" "ChatResponse" "ToolExecuteRequest" "ToolExecuteResponse" "ChannelSendRequest" "ChannelSendResponse" "MemoryQuery" "MemoryCreateRequest" "MemoryResponse")
structs_found=0
for struct in "${structs[@]}"; do
    if grep -q "struct $struct" "$check_file"; then
        echo -e "${GREEN}✓${NC} struct $struct"
        ((structs_found++))
    else
        echo -e "${RED}✗${NC} struct $struct (未找到)"
    fi
done
echo ""

echo "4. 验证辅助函数"
echo "--------------------------------------"
helpers=("execute_shell" "execute_file_read" "execute_file_write" "default_category")
helpers_found=0
for helper in "${helpers[@]}"; do
    if grep -q "async fn $helper" "$check_file" || grep -q "fn $helper" "$check_file"; then
        echo -e "${GREEN}✓${NC} fn $helper"
        ((helpers_found++))
    else
        echo -e "${RED}✗${NC} fn $helper (未找到)"
    fi
done
echo ""

echo "======================================"
echo "验证结果"
echo "======================================"
total_routes=${#routes[@]}
total_handlers=${#handlers[@]}
total_structs=${#structs[@]}
total_helpers=${#helpers[@]}

echo "路由: $routes_found / $total_routes"
echo "处理器: $handlers_found / $total_handlers"
echo "结构体: $structs_found / $total_structs"
echo "辅助函数: $helpers_found / $total_helpers"
echo ""

if [ $routes_found -eq $total_routes ] && [ $handlers_found -eq $total_handlers ] && [ $structs_found -eq $total_structs ]; then
    echo -e "${GREEN}✓ 所有组件已正确部署！${NC}"
    exit 0
else
    echo -e "${YELLOW}! 部分组件缺失${NC}"
    exit 1
fi
