# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do NOT** open a public issue
2. Email the maintainer directly or use [GitHub's private vulnerability reporting](https://github.com/AriajSarkar/gitignore-gen/security/advisories/new)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### Response Timeline

- **Initial response**: Within 48 hours
- **Status update**: Within 7 days
- **Fix release**: Depends on severity

### After Reporting

- We will acknowledge receipt of your report
- We will investigate and determine the impact
- We will work on a fix and coordinate disclosure
- You will be credited in the security advisory (unless you prefer anonymity)

## Security Best Practices

This tool:
- Only reads local file system to detect project types
- Makes HTTPS requests only to `raw.githubusercontent.com` for templates
- Does not execute any downloaded content
- Does not require elevated privileges
