#!/bin/bash

# Set the endpoint to REMOTE if defined, otherwise default to localhost
ENDPOINT=${REMOTE:-"http://localhost:3000/check"}

echo "Testing all circuits in one request..."
curl -X POST "$ENDPOINT" \
  -H "Content-Type: application/json" \
  -d '{"circuit":[
    {"input1":5,"input2":5,"output":1},
    {"input1":6,"input2":6,"output":2},
    {"input1":7,"input2":7,"output":3},
    {"input1":8,"input2":8,"output":4}
  ]}'
