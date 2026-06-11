# Governance

This project should accept community help through issues and pull requests.

## Maintainer Responsibilities

- Protect wallet and deployment secrets.
- Review security-sensitive changes carefully.
- Require evidence for contract, wallet, node, and relay claims.
- Keep public documentation accurate about what is proven and what is experimental.

## Contributor Permissions

External contributors should use forks and pull requests. Direct write access should be limited to trusted maintainers.

Recommended GitHub settings:

- Require pull requests for `main`.
- Require at least one approving review.
- Require status checks before merge.
- Disable force pushes.
- Enable secret scanning and push protection.
- Use CODEOWNERS for security-sensitive files.

## Scope Boundaries

DAO, Railway production deployment, wallet custody, and contract redeploys require explicit maintainer approval.
