# Ruflo execution ledger

- Requested pin: `ruflo 3.57.2`, upstream commit `7be330bcdcbf109ee43e2243e0904587fa0c403d`
- Version probe: `npx --yes ruflo@3.57.2 --version` returned `ruflo v3.57.2`.
- Observed packaging defect: `npx --yes ruflo@3.57.2 swarm --help` failed because `@claude-flow/cli-core/output` was absent.
- Governed fallback: installed `ruflo v3.47.1`.
- Executed command: `ruflo swarm init --topology hierarchical --max-agents 1 --auto-scale false`.
- Result: exit 0; swarm `swarm-1792461200444-0d8wn2`; hierarchical topology; one agent; auto-scale disabled; message-bus protocol.
- Authority: coordination and provenance only. Ruflo generated no product source, made no policy decision, and had no merge or publication authority.
