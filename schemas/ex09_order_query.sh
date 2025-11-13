#!/bin/bash
# Ex09 - Federation: query order with product lines (via router)
# Router: http://localhost:5000/graphql
# Orders subgraph directly: http://localhost:3002/graphql

curl -X POST http://localhost:5000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query ($orderId: ID!) { order(id: $orderId) { id status totalAmount lines { quantity product { id code description ... on DangerousProduct { maxTemperature } ... on ExpiringProduct { expirationDate } } } } }",
    "variables": {
      "orderId": "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa"
    }
  }' | jq
