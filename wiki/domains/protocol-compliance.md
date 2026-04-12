---
domain: protocol-compliance
category: format-constrained
verification_type: execution | constraint
dataset_scale: ~medium (RFCs + test vectors)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Protocol Compliance

## Overview

Protocol compliance tasks require generating messages or data that conform to network and communication protocol specifications: HTTP requests/responses, SMTP email envelopes, DNS query/response packets, TLS handshake messages, and others. Each protocol is defined by RFCs with precise syntax and semantics.

Verification is exact: protocol parsers either accept or reject the message. This domain exercises precise format knowledge, understanding of protocol state machines, and attention to the many details (headers, encoding, field ordering) that make real-world protocols work.

## Verification Mechanism

**Type: Execution (protocol parser) + Constraint satisfaction**

**Approach 1 — Parser acceptance:**
Feed the model's output to a protocol parser and check if it accepts:

```python
from http.client import HTTPResponse
from io import BytesIO
from email import message_from_string
import dns.message

def verify_http_response(model_output: str) -> float:
    try:
        # Parse as HTTP response
        response = HTTPResponse(BytesIO(model_output.encode()))
        response.begin()
        status = response.status
        headers = dict(response.getheaders())
        body = response.read()
        return 1.0  # Successfully parsed
    except Exception:
        return 0.0

def verify_smtp(model_output: str) -> float:
    try:
        msg = message_from_string(model_output)
        required = ['From', 'To', 'Subject', 'Date', 'Message-ID']
        has_required = all(msg[h] is not None for h in required)
        return 1.0 if has_required else 0.5
    except Exception:
        return 0.0

def verify_dns_query(model_output: bytes) -> float:
    try:
        msg = dns.message.from_wire(model_output)
        return 1.0
    except Exception:
        return 0.0
```

**Approach 2 — Field-level verification:**
Parse the message and check specific fields match requirements:

```python
def verify_http_request(model_output: str, requirements: dict) -> float:
    lines = model_output.strip().split('\r\n')
    method, path, version = lines[0].split(' ')
    headers = parse_headers(lines[1:])
    
    checks = {
        'method': method == requirements.get('method', 'GET'),
        'path': path == requirements.get('path', '/'),
        'version': version in ['HTTP/1.0', 'HTTP/1.1', 'HTTP/2'],
        'host_header': 'Host' in headers,
        'content_type': headers.get('Content-Type') == requirements.get('content_type'),
    }
    return sum(checks.values()) / len(checks)
```

**Verification confidence: HIGH.** Protocol parsers are well-tested (they run the internet). If a parser accepts the message, it is protocol-compliant. Field-level checks add behavioral verification.

## Dataset Sources

- **RFC documents:** Every internet protocol is specified in RFCs. RFCs include examples and test vectors.
  - HTTP: RFC 7230-7235 (HTTP/1.1), RFC 7540 (HTTP/2)
  - SMTP: RFC 5321
  - DNS: RFC 1035
  - TLS: RFC 8446
  - WebSocket: RFC 6455
  - IMAP: RFC 3501
- **Wireshark packet captures:** Real-world protocol traffic (anonymized captures available on Wireshark wiki).
- **Protocol test suites:** HTTP compliance test suites, SMTP test tools, DNS test datasets.
- **httpbin.org:** HTTP testing service documenting expected request/response formats.
- **Synthetic generation:** Generate protocol messages from templates with randomized valid parameters.
- **Curl examples:** The curl documentation provides hundreds of example HTTP requests.

## Task Format

**Input:**
```
Construct a valid HTTP/1.1 POST request to https://api.example.com/users with:
- Content-Type: application/json
- Authorization: Bearer token123
- Body: {"name": "Alice", "email": "alice@example.com"}
Include all required headers.
```

**Output:**
```
POST /users HTTP/1.1
Host: api.example.com
Content-Type: application/json
Authorization: Bearer token123
Content-Length: 45
Connection: close

{"name": "Alice", "email": "alice@example.com"}
```

**Input (SMTP):**
```
Construct a valid SMTP email message from sender@example.com to recipient@test.org 
with subject "Meeting Tomorrow" and a plain text body "See you at 3pm."
Include all required RFC 5321/5322 headers.
```

**Input (DNS):**
```
Describe the wire format of a DNS query for the A record of www.example.com.
Provide the query as a hex dump with field annotations.
```

## Difficulty Curriculum

1. **Medium:** Simple HTTP GET/POST requests, basic email headers, DNS query structure.
2. **Hard:** HTTP with complex headers (content negotiation, caching, range requests), multipart MIME email with attachments, DNS responses with multiple records.
3. **Very hard:** TLS ClientHello message construction, HTTP/2 frame format, WebSocket upgrade handshake, SMTP with DKIM signatures, protocol state machine reasoning ("given this sequence of messages, what is the valid next message?").

Note: No "easy" tier because protocol compliance inherently requires knowing specific format details.

## Limitations & Risks

- **Binary protocols:** Many protocols use binary formats (DNS wire format, TLS) that are awkward for text-based LLMs. Must use hex representation.
- **Version proliferation:** HTTP/1.0, HTTP/1.1, HTTP/2, HTTP/3 all have different rules. Must specify version.
- **Partial specification:** Real protocol messages often have many optional fields. Multiple valid messages exist for any given task. Verification must accept valid alternatives.
- **Narrow transfer:** Protocol compliance is specialized knowledge. Skills may not transfer broadly outside of network engineering tasks.
- **Security implications:** Training to generate protocol messages could theoretically help with crafting malicious requests, though this knowledge is publicly available in RFCs.

## Connections

- [[config-generation]] — Both involve generating standards-compliant structured output.
- [[data-formatting]] — Protocol messages are structured data with strict format rules.
- [[calendar-ical]] — iCalendar is an application-level protocol (RFC 5545).
- [[encryption-decryption]] — TLS protocol involves cryptographic parameters.
