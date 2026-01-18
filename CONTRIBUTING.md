# Contributing to AXON

First off, thanks for taking the time to contribute!

## The "Deca-Stack" Rule
AXON is a polyglot system. Before contributing, ensure you understand the role of the language you are touching:
- **Rust**: Core logic, UI, Safety.
- **Go**: Networking, I/O.
- **Python**: AI Logic.

## Pull Request Process
1.  Ensure any install or build dependencies are removed before the end of the layer when doing a build.
2.  Update the `README.md` with details of changes to the interface.
3.  You may merge the Pull Request in once you have the sign-off of two other developers (or the Lead Architect).

## Coding Standards
- **Rust**: Run `cargo fmt` and `cargo clippy`.
- **Go**: Run `go fmt ./...`.
