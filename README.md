# Bounty Program (Anchor)

A Solana Anchor program that implements a simple bounty system (create bounty, claim bounty, fund bounty, close bounty). This repository contains the on-chain program, IDL, TypeScript types, and tests.

## Contents
- **Program:** on-chain logic in Rust under this folder.
- **IDL:** [idl/bounty.json](../../idl/bounty.json)
- **TypeScript types:** [types/bounty_program.ts](../../types/bounty_program.ts)
- **Tests:** located in `tests/` at repository root (`tests/bounty_program.ts`).

## Overview

The bounty program manages task-based rewards using SPL token escrow. 
Each bounty is a unique PDA account, paired with a PDA-controlled token 
vault that holds escrowed rewards trustlessly.

### Instructions

| Instruction     | Description |
|-----------------|-------------|
| `create_bounty` | Deploys a bounty PDA + vault token account. Sets creator, claimant, mint, and reward. |
| `fund_bounty`   | CPI transfers SPL tokens from a funder's token account into the PDA vault. |
| `claim_bounty`  | Pre-assigned claimant collects all vault tokens via PDA-signed CPI. Accounts are closed. |
| `close_bounty`  | Creator cancels the bounty. Vault tokens are refunded via CPI. Accounts are closed. |

### PDA Derivation
```
Bounty PDA : [b"bounty", creator_pubkey, bounty_id_le_bytes]
Vault PDA  : [b"vault",  bounty_pda]
```

### Key Design Points

- The vault authority is the bounty PDA — only the program can authorize token movements
- `transfer_checked` is used for all SPL transfers (Token-2022 compatible)
- `has_one` constraints on vault and mint prevent account substitution attacks
- Account closure returns rent lamports to the creator

## Prerequisites

- Rust (stable) with the Solana-compatible toolchain installed (see `rust-toolchain.toml`).
- Solana CLI (recommended version compatible with the Anchor version used).
- Anchor CLI (for build/test/deploy).
- Node.js >= 18 and npm (for TypeScript tests and types).

Install common tools (example):

```bash
# Solana CLI
sh -c "(curl -sSfL https://release.solana.com/stable/install)"

# Anchor (requires npm + cargo)
npm install -g @coral-xyz/anchor-cli

# Project dependencies for tests
npm install
```

## Build

From the repository root run:

```bash
anchor build
```

This compiles the Rust program and generates the IDL under the `target/` and `idl/` paths.

## Test (local)

Run the Anchor test suite (the project includes `tests/bounty_program.ts`):

```bash
anchor test
```

Notes:
- `anchor test` spins up a local test validator, deploys the program, runs the TypeScript tests, and then tears down the local validator.
- If you need faster iteration, consider running unit tests or using LiteSVM as documented in the project docs.

## Deploy

Deploy to the configured cluster using Anchor's deploy command. For example, to deploy to devnet after updating `Anchor.toml`:

```bash
anchor deploy --provider.cluster devnet
```

If deploying to a custom cluster or using a specific keypair, set `ANCHOR_PROVIDER_URL` and `ANCHOR_WALLET` environment variables, or update `Anchor.toml`.

## Interacting with the Program (TypeScript)

The generated IDL lives at [idl/bounty.json](../../idl/bounty.json) and the typed client is available at [types/bounty.ts](../../types/bounty.ts). Typical interaction steps from a TypeScript client:

1. Load the IDL and program ID.
2. Create and sign transactions with a wallet provider.
3. Send instructions and await confirmations.

Example (pseudo-code):

```ts
import { Program, AnchorProvider, Wallet } from '@coral-xyz/anchor';
import idl from '../../idl/bounty.json';
import { Bounty } from '../../types/bounty';

const provider = AnchorProvider.local();
const program = new Program(idl as any, process.env.BOUNTY_PROGRAM_ID, provider) as Program<Bounty>;

// create a bounty
await program.rpc.createBounty(...);
```

Refer to the tests in `tests/bounty_program.ts` for concrete, working examples of instruction usage and account construction.

## File map (important locations)

- Program source: [programs/bounty_program/src](../bounty_program/src)
- IDL: [idl/bounty.json](../../idl/bounty.json)
- Types: [types/bounty.ts](../../types/bounty.ts)
- Tests: [tests/bounty_program.ts](../../tests/bounty_program.ts)

## Contributing

- Run `anchor build` before opening PRs that change the program.
- Add/extend tests in `tests/` to cover new behavior.
- Keep the IDL and TypeScript types in sync: `anchor build` regenerates the IDL; re-run any codegen step your project uses to refresh `types/`.

## Troubleshooting & Notes

- If you see mismatched IDL/program IDs, check `Anchor.toml` and `target/deploy/*.json` for the deployed program keypair.
- For CLI version mismatches, verify `anchor --version` and `solana --version` match the versions used in CI.
- If tests fail due to validator issues, try `solana-test-validator --reset` or `anchor test --skip-deploy` depending on the failure mode.

## References

- IDL file: [idl/bounty.json](../../idl/bounty.json)
- Types file: [types/bounty.ts](../../types/bounty.ts)
- Tests: [tests/bounty_program.ts](../../tests/bounty_program.ts)

---
Created to make onboarding and development easier. If you want, I can add CI snippets (GitHub Actions) to run `anchor test` on PRs.
