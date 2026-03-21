# Security

## Reporting Vulnerabilities

If you discover a security vulnerability, please report it privately via email to **contact@steven-geller.com**. Do not open a public issue.

I will acknowledge receipt within 48 hours and aim to release a fix within 7 days of confirmation.

## Security Measures

### Input Handling

- Text input is sanitized and truncated (max 20 words) before processing
- SVG output uses deterministic path generation with no user-controlled attributes
- Uploaded SVGs for decode are parsed with strict path-data extraction only
- No shell commands, SQL, or template interpolation on user input

### File Uploads

- The `/api/decode` endpoint accepts multipart file uploads
- Only SVG path data (`d` attribute) is extracted; all other content is discarded
- No uploaded files are written to disk or served back

### Access Logging

- `access.log` records timestamp, IP, and input text for operational visibility
- Log file is local only and not exposed via any endpoint

### OG Image Cache

- In-memory cache limited to 500 entries to prevent unbounded memory growth
- Cache keys are derived from user input text; no filesystem writes
