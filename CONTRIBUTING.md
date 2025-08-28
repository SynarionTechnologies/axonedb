# Contributing to AxoneDB

Thank you for considering a contribution! Please review [AGENTS.md](AGENTS.md) for detailed coding standards and project policies.

## Pull Request Checklist
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --workspace --all-features`
- [ ] `cargo bench` (baseline comparison)
- [ ] Documentation updated (`/docs` and Rustdoc)
- [ ] Feature flags behind TBD where appropriate

## Commit Convention
We use [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` new feature
- `fix:` bug fix
- `chore:` maintenance
- `docs:` documentation

## Reviews
At least one reviewer is required for all changes, and two for performance-critical code.
