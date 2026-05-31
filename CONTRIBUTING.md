# Contributing to FlowNet

Thanks for considering a contribution!

## Quick start

```bash
git clone https://github.com/LambiKYT/FlowNet.git
cd flownet
npm --prefix ui install
cargo tauri dev
```

## Workflow

1. **Discuss** — open an issue before writing code for large features.
2. **Fork & branch** — create a branch from `main`:
   - `feat/` for features (`feat/bpf-filter`)
   - `fix/` for bug fixes (`fix/packet-loss`)
   - `docs/` for documentation (`docs/api-guide`)
3. **Code** — keep commits atomic; follow `rustfmt` and `clippy`.
4. **Test** — run the full check suite:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace -- -D warnings
   cargo test
   cd ui && npm run build
   ```
5. **PR** — open a Pull Request against `main`, reference the issue, ensure CI passes.

## Guidelines

- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/).
- One feature or fix per PR.
- Packet capture requires elevated privileges; document any privileged operations.

## Code of Conduct

All contributors must follow [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).
