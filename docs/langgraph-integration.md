# LangGraph Integration Guide

This guide explains how to use the Python package `redclaw-tools` to get consistent tool-calling behavior across any OpenAI-compatible LLM provider.

## Background

Some LLM providers—especially certain models from China such as GLM-5 (Zhipu AI)—can behave inconsistently when using text-based tool invocation.
RedClaw’s Rust core uses structured tool calling in the OpenAI API format, but some models respond better with a different approach.

LangGraph provides a stateful graph execution engine that enforces consistent tool-calling behavior regardless of the underlying model’s native capabilities.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Your Application                        │
├─────────────────────────────────────────────────────────────┤
│                   redclaw-tools Agent                       │
│                                                              │
│   ┌─────────────────────────────────────────────────────┐   │
│   │              LangGraph StateGraph                    │   │
│   │                                                      │   │
│   │    ┌────────────┐         ┌────────────┐            │   │
│   │    │   Agent    │ ──────▶ │   Tools    │            │   │
│   │    │   Node     │ ◀────── │   Node     │            │   │
│   │    └────────────┘         └────────────┘            │   │
│   │         │                       │                    │   │
│   │         ▼                       ▼                    │   │
│   │    [Continue?]            [Execute Tool]             │   │
│   │         │                       │                    │   │
│   │    Yes │ No                Result│                    │   │
│   │         ▼                       ▼                    │   │
│   │      [END]              [Back to Agent]              │   │
│   │                                                      │   │
│   └─────────────────────────────────────────────────────┘   │
│                                                              │
├─────────────────────────────────────────────────────────────┤
│            OpenAI-Compatible LLM Provider                    │
│   (Z.AI, OpenRouter, Groq, DeepSeek, Ollama, etc.)          │
└─────────────────────────────────────────────────────────────┘
```

## Quick Start

### Install

```bash
pip install redclaw-tools
```

### Basic usage

```python
import asyncio
from redclaw_tools import create_agent, shell, file_read, file_write
from langchain_core.messages import HumanMessage

async def main():
    agent = create_agent(
        tools=[shell, file_read, file_write],
        model="glm-5",
        api_key="your-api-key",
        base_url="https://api.z.ai/api/coding/paas/v4"
    )

    result = await agent.ainvoke({
        "messages": [HumanMessage(content="Read /etc/hostname and tell me the machine name")]
    })

    print(result["messages"][-1].content)

asyncio.run(main())
```

## Available Tools

### Core tools

| Tool | Description |
|------|-------------|
| `shell` | Execute shell commands |
| `file_read` | Read file contents |
| `file_write` | Write file contents |

### Extended tools

| Tool | Description |
|------|-------------|
| `web_search` | Web search (requires `BRAVE_API_KEY`) |
| `http_request` | Make HTTP requests |
| `memory_store` | Store data in long-term memory |
| `memory_recall` | Recall stored data |

## Custom tools

Create your own tools using the `@tool` decorator:

```python
from redclaw_tools import tool, create_agent

@tool
def get_weather(city: str) -> str:
    """Get the current weather for a city."""
    # Your implementation
    return f"Weather in {city}: Sunny, 25°C"

@tool
def query_database(sql: str) -> str:
    """Execute a SQL query and return results."""
    # Your implementation
    return "Query returned 5 rows"

agent = create_agent(
    tools=[get_weather, query_database],
    model="glm-5",
    api_key="your-key"
)
```

## Provider configuration

### Z.AI / GLM-5

```python
agent = create_agent(
    model="glm-5",
    api_key="your-zhipu-key",
    base_url="https://api.z.ai/api/coding/paas/v4"
)
```

### OpenRouter

```python
agent = create_agent(
    model="anthropic/claude-sonnet-4-6",
    api_key="your-openrouter-key",
    base_url="https://openrouter.ai/api/v1"
)
```

### Groq

```python
agent = create_agent(
    model="llama-3.3-70b-versatile",
    api_key="your-groq-key",
    base_url="https://api.groq.com/openai/v1"
)
```

### Ollama (local)

```python
agent = create_agent(
    model="llama3.2",
    base_url="http://localhost:11434/v1"
)
```

## Discord Bot integration

```python
import os
from redclaw_tools.integrations import DiscordBot

bot = DiscordBot(
    token=os.environ["DISCORD_TOKEN"],
    guild_id=123456789,  # Your Discord server ID
    allowed_users=["123456789"],  # User IDs that can use the bot
    api_key=os.environ["API_KEY"],
    model="glm-5"
)

bot.run()
```

## Using via CLI

```bash
# Set environment variables
export API_KEY="your-key"
export BRAVE_API_KEY="your-brave-key"  # Optional, for web search

# Single message
redclaw-tools "What is the current date?"

# Interactive mode
redclaw-tools -i
```

## Comparison with Rust RedClaw

| Aspect | Rust RedClaw | redclaw-tools |
|--------|---------------|-----------------|
| **Performance** | Extremely fast (~10ms startup) | Python startup (~500ms) |
| **Memory** | <5 MB | ~50 MB |
| **Binary size** | ~3.4 MB | pip package |
| **Tool-call consistency** | Model-dependent | Enforced by LangGraph |
| **Extensibility** | Rust traits | Python decorators |
| **Ecosystem** | Rust crates | PyPI packages |

**When to use Rust RedClaw:**
- Production edge deployments
- Resource-constrained environments (Raspberry Pi, etc.)
- Maximum performance requirements

**When to use redclaw-tools:**
- Models with inconsistent native tool calling
- Python-centered development
- Rapid prototyping
- Integrations with the Python ML ecosystem

## Troubleshooting

### "API key required" error

Set the `API_KEY` environment variable or pass `api_key` into `create_agent()`.

### Tool calls are not executed

Make sure your model supports function calling. Some older models may not support tool calling.

### Rate limiting

Add a delay between calls or implement your own rate limiter:

```python
import asyncio

for message in messages:
    result = await agent.ainvoke({"messages": [message]})
    await asyncio.sleep(1)  # Rate limit
```

## Related Projects

- [rs-graph-llm](https://github.com/a-agmon/rs-graph-llm) - Rust LangGraph alternative
- [langchain-rust](https://github.com/Abraxas-365/langchain-rust) - LangChain for Rust
- [llm-chain](https://github.com/sobelio/llm-chain) - LLM chains in Rust
