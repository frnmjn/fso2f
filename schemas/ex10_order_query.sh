#!/bin/bash
# Ex10 - Feature Subgraph: query order with product lines + feature flag header
# Router: http://localhost:5000/graphql
# Header X-Feature-Flag: flag-customers abilita il subgrafo customers reale

curl -X POST http://localhost:5000/graphql \
  -H "Content-Type: application/json" \
  -H "X-Feature-Flag: flag-customers" \
  -d '{
    "query": "query ($orderId: ID!) { order(id: $orderId) { id status totalAmount customer { id name vat } lines { id quantity price discount product { id code description ... on DangerousProduct { maxTemperature } ... on ExpiringProduct { expirationDate } } } } }",
    "variables": {
      "orderId": "aaaa1111-aa11-4aa1-8aa1-aaaaaaaaaaaa"
    }
  }' | jq
