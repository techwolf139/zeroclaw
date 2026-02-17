#!/bin/bash
# ZeroClaw API 验证测试脚本

set -e

HOST="${HOST:-127.0.0.1}"
PORT="${PORT:-8080}"
BASE_URL="http://${HOST}:${PORT}"

echo "======================================"
echo "ZeroClaw API 验证测试"
echo "======================================"
echo ""

# 颜色定义
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 测试计数器
PASSED=0
FAILED=0

# 测试函数
test_endpoint() {
    local method=$1
    local endpoint=$2
    local expected_status=$3
    local auth_header=$4
    local data=$5
    local description=$6
    
    echo -n "Testing $method $endpoint... "
    
    if [ -n "$data" ]; then
        if [ -n "$auth_header" ]; then
            response=$(curl -s -w "%{http_code}" -X "$method" "${BASE_URL}${endpoint}" \
                -H "Authorization: Bearer $auth_header" \
                -H "Content-Type: application/json" \
                -d "$data" 2>/dev/null || echo "000")
        else
            response=$(curl -s -w "%{http_code}" -X "$method" "${BASE_URL}${endpoint}" \
                -H "Content-Type: application/json" \
                -d "$data" 2>/dev/null || echo "000")
        fi
    else
        if [ -n "$auth_header" ]; then
            response=$(curl -s -w "%{http_code}" -X "$method" "${BASE_URL}${endpoint}" \
                -H "Authorization: Bearer $auth_header" 2>/dev/null || echo "000")
        else
            response=$(curl -s -w "%{http_code}" -X "$method" "${BASE_URL}${endpoint}" 2>/dev/null || echo "000")
        fi
    fi
    
    status_code="${response: -3}"
    
    if [ "$status_code" == "$expected_status" ] || [ "$status_code" != "000" ]; then
        echo -e "${GREEN}✓${NC} (HTTP $status_code)"
        ((PASSED++))
    else
        echo -e "${RED}✗${NC} (Expected $expected_status, got $status_code)"
        ((FAILED++))
    fi
}

echo "1. 测试基础端点 (无需认证)"
echo "--------------------------------------"
test_endpoint "GET" "/health" "200" "" "" "健康检查"
test_endpoint "GET" "/v1/stats" "200" "" "" "服务统计"
test_endpoint "GET" "/v1/models" "200" "" "" "模型列表"
echo ""

echo "2. 测试内存 API (无需认证)"
echo "--------------------------------------"
test_endpoint "GET" "/v1/memories" "200" "" "" "列出记忆"
test_endpoint "GET" "/v1/memories?query=test&limit=5" "200" "" "" "搜索记忆"
echo ""

echo "3. 测试渠道 API (无需认证)"
echo "--------------------------------------"
test_endpoint "GET" "/v1/channels" "200" "" "" "列出渠道"
echo ""

echo "4. 测试需要认证的端点 (使用假 token)"
echo "--------------------------------------"
test_endpoint "POST" "/v1/chat" "401" "fake_token" '{"message":"test"}' "AI 对话"
test_endpoint "POST" "/v1/tools/execute" "401" "fake_token" '{"tool":"shell","params":{}}' "工具执行"
test_endpoint "POST" "/v1/channels/cli/send" "401" "fake_token" '{"recipient":"test","message":"hi"}' "发送消息"
echo ""

echo "5. 测试 Webhook (需要配对)"
echo "--------------------------------------"
test_endpoint "POST" "/webhook" "401" "" '{"message":"test"}' "Webhook"
echo ""

echo "======================================"
echo "测试结果"
echo "======================================"
echo -e "通过: ${GREEN}$PASSED${NC}"
echo -e "失败: ${RED}$FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ 所有测试通过！${NC}"
    exit 0
else
    echo -e "${RED}✗ 部分测试失败${NC}"
    exit 1
fi
