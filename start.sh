#!/bin/zsh

if ! [ -f "target/release/ai_chat" ]; then
  if ! cargo build --release; then
    echo "Could not build binary! Press any key to exit..." >&2
    read
    exit
  fi
fi

if ! [ -f "openai.sk" ]; then
  echo -n "Enter OpenAI secret key: "
  read -s OPENAI_SK
else
  OPENAI_SK="$(cat openai.sk)"
fi

export OPENAI_SK
target/release/ai_chat
