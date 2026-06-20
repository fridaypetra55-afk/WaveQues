# WaveQuest 🌊🎓

> Gamified learn-to-earn on Stellar — complete quizzes, maintain streaks, earn verifiable on-chain achievements, and receive XLM micro-rewards powered by Soroban smart contracts.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Built on Stellar](https://img.shields.io/badge/built%20on-Stellar-6366F1)](https://stellar.org)
[![Soroban](https://img.shields.io/badge/smart%20contracts-Soroban-22D3EE)](https://soroban.stellar.org)
[![Rust](https://img.shields.io/badge/contracts-Rust%20100%25-CE422B)](https://www.rust-lang.org)
[![CI](https://img.shields.io/github/actions/workflow/status/Wave-Quest/WaveQues/ci.yml?label=CI)](https://github.com/Wave-Quest/WaveQues/actions)

---

## Overview

Education in emerging markets is undermined by one persistent problem: learners have no verifiable, portable proof of what they know. Certificates are paper. Grades are local. Neither travels.

WaveQuest fixes this at the protocol layer. Every quiz completion is hashed and recorded on a Soroban smart contract — immutable, publicly auditable, and permanently linked to the learner's Stellar wallet. Complete a module: the contract pays XLM directly to your wallet, no intermediary, no delay. Maintain a daily streak: bonus multiplier applied on-chain. Earn enough achievements: an NFT-style on-chain credential minted to your account.

The platform is designed for accessibility first — mobile-first Next.js frontend, lightweight NestJS API, and a Soroban contract that is gas-efficient enough to run on Stellar's sub-cent fees. A learner in Lagos with a Freighter wallet and a data connection can participate on equal footing with anyone anywhere.

---

## Architecture

```
┌────────────────────────────────────────────────────────────────────┐
│                    Browser / Freighter Wallet                      │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                  Frontend  (Next.js 15)                      │  │
│  │                                                              │  │
│  │  /              Landing — stats, leaderboard preview        │  │
│  │  /learn         Module browser + quiz interface             │  │
│  │  /learn/[id]    Single quiz — questions, timer, submit      │  │
│  │  /achievements  On-chain credential gallery                 │  │
│  │  /leaderboard   Global XLM earnings rankings                │  │
│  │  /profile       My streaks, completions, earnings           │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                          │ REST API                                │
└──────────────────────────┼────────────────────────────────────────┘
                           │
           ┌───────────────▼──────────────────────────┐
           │         Backend  (NestJS / TypeScript)    │
           │                                           │
           │  POST /auth/challenge                     │
           │  POST /auth/verify                        │
           │  GET  /modules                            │
           │  POST /modules/:id/submit                 │
           │  GET  /leaderboard                        │
           │  GET  /profile/:address                   │
           └───────────┬──────────────────────────────┘
                       │
          ┌────────────▼────────────────────────────────┐
          │       Soroban Contract  (contracts/)         │
          │                                              │
          │  initialize(admin, treasury, reward_bps)     │
          │  approve_answer(oracle, module_id, hash)     │
          │  claim_reward(learner, module_id)            │
          │  record_streak(learner)                      │
          │  mint_achievement(learner, achievement_id)   │
          │  get_completion(learner, module_id)          │
          │  get_streak(learner)                         │
          │  get_balance(fund_id)                        │
          └────────────┬────────────────────────────────┘
                       │
                Stellar Network
                (Testnet / Mainnet)
```

### Stack

| Layer | Technology |
|---|---|
| Frontend | Next.js 15, TypeScript, Tailwind CSS |
| Backend | NestJS, TypeScript, Node.js 20 |
| Smart Contracts | Rust 100%, Soroban SDK `22.0.0` |
| Wallet | Freighter (`@stellar/freighter-api`) |
| Stellar SDK | `@stellar/stellar-sdk` |
| CI/CD | GitHub Actions (`.github/workflows/`) |
| API Testing | Postman (`postman/`) |
| Containers | Docker + Docker Compose |
| Network | Stellar Testnet / Mainnet |

---

## Core Protocol Mechanics

### Anti-Cheat Answer Hashing

Every quiz submission is protected by a learner-specific cryptographic hash:

```
answerHash = sha256( answer + learnerAddress + moduleId )
```

The oracle backend computes this hash server-side and calls `approve_answer()` on the contract. The same answer produces a different hash for every wallet — sharing answers is cryptographically useless. Approved hashes are deleted from contract storage after the first `claim_reward()` call, preventing replay attacks.

### Streak System

```
daily_streak    → +10% XLM bonus on reward
weekly_streak   → +25% XLM bonus on reward
streak reset    → 00:00 UTC if no completion that calendar day
```

`record_streak(learner)` is called atomically with `claim_reward()` — both succeed or both fail.

### Achievement Credentials

On-chain achievement NFTs (non-transferable) are minted via `mint_achievement()` when a learner hits defined milestones:

| Achievement | Condition |
|---|---|
| First Steps | Complete 1 module |
| Stellar Scholar | Complete 10 modules |
| Streak Master | 7-day streak |
| XLM Earner | Earn 10 XLM total |
| Quest Champion | Top 10 leaderboard |

---

## Quick Start

### Prerequisites

- Node.js 20+
- Rust stable + `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-stellar-cli)
- [Freighter wallet extension](https://freighter.app)
- Docker + Docker Compose

### 1. Clone and configure

```bash
git clone https://github.com/Wave-Quest/WaveQues.git
cd WaveQues
cp .env.example .env.local
```

### 2. Start the stack

```bash
docker compose up -d
```

### 3. Install dependencies

```bash
# Frontend
cd frontend && npm install

# Backend
cd ../backend && npm install
```

### 4. Build and test the contracts

```bash
cd contracts
rustup target add wasm32-unknown-unknown
cargo test
cargo build --target wasm32-unknown-unknown --release
```

### 5. Deploy to Stellar testnet

```bash
# Fund deployer
stellar keys generate deployer --network testnet
stellar keys fund deployer --network testnet

# Deploy
stellar contract deploy \
  --wasm contracts/target/wasm32-unknown-unknown/release/wavequest.wasm \
  --source deployer \
  --network testnet

# Initialize
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin <DEPLOYER_ADDRESS> \
  --oracle <ORACLE_ADDRESS> \
  --reward_bps 100

# Copy CONTRACT_ID to .env.local
```

### 6. Run in development

```bash
# Terminal 1 — Frontend (http://localhost:3000)
cd frontend && npm run dev

# Terminal 2 — Backend (http://localhost:3001)
cd backend && npm run start:dev
```

---

## Environment Variables

| Variable | Description | Required |
|---|---|---|
| `CONTRACT_ID` | Deployed Soroban WaveQuest contract ID | Yes |
| `NEXT_PUBLIC_CONTRACT_ID` | Contract ID for frontend | Yes |
| `NEXT_PUBLIC_STELLAR_NETWORK` | `testnet` or `mainnet` | Yes |
| `NEXT_PUBLIC_HORIZON_URL` | Horizon API URL | Yes |
| `NEXT_PUBLIC_RPC_URL` | Soroban RPC endpoint | Yes |
| `ORACLE_SECRET_KEY` | Stellar secret key for oracle answer approval | Yes |
| `JWT_SECRET` | JWT signing key for backend auth | Yes |
| `DATABASE_URL` | PostgreSQL connection string | Yes |
| `PORT` | Backend port (default: `3001`) | No |

---

## Contract API

| Function | Arguments | Auth | Description |
|---|---|---|---|
| `initialize` | `admin, oracle, reward_bps` | Admin | One-time contract setup |
| `approve_answer` | `module_id: String, answer_hash: BytesN<32>` | Oracle | Oracle registers approved answer hash for a module |
| `claim_reward` | `learner: Address, module_id: String` | Learner | Learner claims XLM reward after submitting correct answer hash; deletes used hash |
| `record_streak` | `learner: Address` | Learner | Update daily/weekly streak counter; applies bonus multiplier |
| `mint_achievement` | `learner: Address, achievement_id: u32` | Oracle | Mint non-transferable on-chain credential to learner |
| `get_completion` | `learner: Address, module_id: String` | Read | Returns whether learner has completed a module |
| `get_streak` | `learner: Address` | Read | Returns current streak count and last completion timestamp |
| `get_balance` | — | Read | Returns treasury XLM balance available for rewards |

---

## Backend API

| Method | Route | Auth | Description |
|---|---|---|---|
| `GET` | `/auth/challenge?address=G...` | None | Issue a one-time signing challenge |
| `POST` | `/auth/verify` | None | Verify Freighter signature → issue JWT |
| `GET` | `/modules` | JWT | List all available quiz modules |
| `POST` | `/modules/:id/submit` | JWT | Submit answer → oracle approves hash → returns signed XDR |
| `GET` | `/leaderboard` | None | Top 50 learners by XLM earned |
| `GET` | `/profile/:address` | None | Learner stats — completions, streak, earnings, achievements |

---

## Frontend Pages

| Route | Description |
|---|---|
| `/` | Landing — protocol stats, how it works, top earners preview |
| `/learn` | Module browser — filter by topic, difficulty, reward amount |
| `/learn/[id]` | Quiz — step-through questions, countdown timer, submit + sign |
| `/achievements` | On-chain credential gallery — all earned badges |
| `/leaderboard` | Global rankings by XLM earned, sortable by streak and completions |
| `/profile` | My stats — streak calendar, earnings chart, completion history |

---

## Project Structure

```
WaveQues/
├── .github/
│   └── workflows/              # GitHub Actions CI
├── backend/                    # NestJS API
│   └── src/
│       ├── auth/               # JWT + Freighter challenge-response
│       ├── modules/            # Quiz module management
│       ├── oracle/             # Answer hash approval + contract calls
│       └── leaderboard/        # Rankings service
├── contracts/                  # Soroban Rust contracts (100% Rust)
│   └── src/
│       ├── lib.rs              # All contract entry points
│       ├── streak.rs           # Streak and bonus logic
│       ├── achievements.rs     # Credential minting
│       └── test.rs             # Cargo test suite
├── frontend/                   # Next.js 15 app
│   ├── app/                    # App Router pages
│   ├── components/
│   │   ├── QuizCard.tsx
│   │   ├── AchievementBadge.tsx
│   │   ├── StreakBar.tsx
│   │   └── WalletConnect.tsx
│   └── lib/
│       ├── stellar.ts          # Freighter helpers
│       └── hash.ts             # Client-side sha256 preview
├── postman/                    # API test collection
├── docker-compose.yml
└── .env.example
```

---

## Contributing

Contributions are welcome. Please open an issue first to discuss what you would like to change, then submit a pull request against `main`.

**Quick rules:**
- One concern per PR — bug fixes and features in separate PRs
- For contract changes: run `cargo test` and include the full output in the PR description
- No TypeScript `any` types
- `cargo clippy --deny warnings` must pass before requesting review
- Follow Conventional Commits: `feat:`, `fix:`, `docs:`, `test:`, `chore:`

---

## Stellar Testnet Resources

- [Stellar Laboratory](https://laboratory.stellar.org) — inspect accounts, transactions, and contracts
- [Friendbot](https://friendbot.stellar.org) — fund a testnet account with free XLM
- [Soroban Testnet RPC](https://soroban-testnet.stellar.org) — RPC endpoint for contract calls
- [Stellar Expert (Testnet)](https://stellar.expert/explorer/testnet) — block explorer

---

## Roadmap

- [ ] Multi-language quiz content (Yoruba, Swahili, French)
- [ ] DAO governance for module curation and reward rates
- [ ] Institution dashboard — universities can issue verified credentials
- [ ] Mobile PWA with offline quiz mode
- [ ] Mainnet deployment

---

## License

MIT © [Wave-Quest](https://github.com/Wave-Quest)
