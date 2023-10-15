#!/bin/bash

servers=("localhost:8080" "localhost:8081" "localhost:8082" "localhost:8083")
keys=("key1" "key2" "key3" "key4", "key5", "key6", "key7", "key8")

server="${servers[$RANDOM % ${#servers[@]}]}"

for key in "${keys[@]}"; do
  response=$(curl -s -H "Content-Type: application/json" -X PUT -d "Hello World!" "$server/cache/$key/30")
done

for server in "${servers[@]}"; do
  for key in "${keys[@]}"; do
    response=$(curl -s "$server/cache/$key")
    if [ "$response" != "Hello World!" ]; then
      echo "[$server] Test failed: Expected 'Hello World!', got '$response'"
    else
      echo "[$server] Test passed"
    fi
  done
done

