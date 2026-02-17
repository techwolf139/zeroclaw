# ESP32 UI Slint Interface Design

**Date**: 2026-02-17  
**Status**: Approved

## Overview

Hybrid chat interface for ZeroClaw ESP32 UI - combines chat view with settings menu overlay.

## Screen Layout

### Main Chat View (320x240)

```
┌─────────────────────────────┐
│ 🦀 ZeroClaw  [WiFi] ● ●   │  <- Status bar (28px)
├─────────────────────────────┤
│                             │
│  🤖 Hello! How can I      │  <- AI message
│     help you today?        │
│                             │
│  Hi! Can you help me with  │  <- User message
│  coding?                    │
│                             │
├─────────────────────────────┤
│ [Input...............] [Send]│  <- Input bar (32px)
└─────────────────────────────┘
```

### Settings Menu Overlay

```
┌─────────────────────────────┐
│ ⚙️ Settings          [X]  │
├─────────────────────────────┤
│ WiFi                       │
│   🔄 Connect / Disconnect  │
│   📡 Scan Networks         │
├─────────────────────────────┤
│ Connection                  │
│   🌐 Server: 192.168.1.x  │
│   🔗 Status: Connected     │
├─────────────────────────────┤
│ Display                     │
│   ☀ Brightness [====    ] │
├─────────────────────────────┤
│ About                       │
│   v0.1.0 | ZeroClaw ESP32  │
└─────────────────────────────┘
```

## Components

### StatusBar
- Brand text "ZeroClaw" (red)
- WiFi icon + signal strength (1-3 bars)
- Connection status LED (cyan/red)
- Tap WiFi icon → open WiFi submenu

### MessageList
- Scrollable ListView
- User messages: right-aligned, blue bubble (#0f3460)
- AI messages: left-aligned, dark bubble (#1a1a2e)
- Max visible: ~6 messages on screen

### InputBar
- TextInput field (single line)
- Send button (red, rounded)
- Tap or Enter to send

### MenuOverlay
- Modal overlay on chat view
- Semi-transparent background
- Tap outside or X to close

## Color Palette

| Element | Hex |
|---------|-----|
| Background | #1a1a2e |
| Status Bar | #16213e |
| User Bubble | #0f3460 |
| Send Button | #e94560 |
| Connected LED | #00d9ff |
| Disconnected LED | #ff4757 |
| Text | #eaeaea |
| Secondary Text | #666666 |

## State Machine

```
┌──────────────┐     tap_settings     ┌──────────────┐
│  ChatView    │ ──────────────────►  │   MenuView   │
│              │ ◄────────────────────  │              │
└──────────────┘     tap_close        └──────────────┘
        │
        │ send_message
        ▼
   [Send to Server]
```

## Implementation Notes

### ESP32 Memory Constraints
- No emoji (use text icons)
- Small fonts: 10-12px
- Max 20 messages in memory
- Flat design (no shadows)

### Server Communication
- HTTP POST to `/v1/chat`
- JSON body: `{"message": "...", "model": "...", "temperature": 0.7}`
- Response: `{"response": "...", "model": "..."}`

### WiFi Configuration
- Stored in NVS flash
- Auto-connect on startup
- Signal strength: RSSI to bars mapping
