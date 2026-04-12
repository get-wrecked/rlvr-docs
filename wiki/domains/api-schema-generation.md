---
domain: API Schema Generation
category: Code & Software
verification_type: execution
dataset_scale: ~40K+ OpenAPI specs from APIs.guru
difficulty_range: easy/medium/hard
modality: code
status: strong_hypothesis
---

# API Schema Generation

## Overview
Generate OpenAPI/Swagger specs, GraphQL schemas, or Protocol Buffer definitions from requirements. Verification via official schema validators and compiler-based checks.

## Verification Mechanism
1. Schema parses with official validator (swagger-parser, graphql-js, protoc)
2. Generated client/server stubs compile
3. Schema satisfies stated constraints (required endpoints, types, relationships)

## Dataset Sources
- APIs.guru (40K+ OpenAPI specs)
- GitHub public GraphQL schemas
- Google APIs protobuf definitions
- Buf registry for protobufs

## Task Format
**Input**: Natural language API requirements
**Output**: OpenAPI YAML, GraphQL SDL, or .proto file

## Difficulty Curriculum
1. Simple CRUD API schemas
2. Nested resources with relationships
3. Authentication/authorization schemes
4. Pagination, filtering, versioning patterns
5. Complex real-time APIs (WebSocket, streaming gRPC)

## Connections
[[config-generation]], [[structured-output-generation]], [[natural-language-to-api]]
