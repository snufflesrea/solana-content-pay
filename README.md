# Solana Content Pay

**Pay-per-content with USDC on Solana, powered by Rust and x402.**

Solana Content Pay is a payment-gated digital content store where users and AI agents can purchase access to digital products using USDC on Solana. Built with Rust and Axum, the project explores how blockchain payments and the x402 protocol can enable automated, pay-per-request access to digital content.

Instead of subscriptions or traditional API keys, customers pay for the content they want and receive access after the payment is verified and settled.

> **Status:** Work in Progress 🚧

## Overview

Digital creators often rely on subscriptions, centralized payment processors, or manual payment verification to sell digital content. Solana Content Pay explores an alternative: programmable, on-demand payments using USDC.

The initial version focuses on selling digital products such as:

- Solana security checklists
- Rust and Anchor security cheatsheets
- Educational guides and mini-courses
- Downloadable digital resources

The long-term goal is to enable AI agents to discover available content, evaluate prices, pay autonomously within predefined spending limits, and retrieve purchased resources programmatically.

## How It Works

```text
Customer / AI Agent
        |
        v
   Browse Products
        |
        v
 Request Protected Content
        |
        v
  Rust / Axum Server
        |
        v
  HTTP 402 Payment Required
        |
        v
   Pay with USDC
     on Solana
        |
        v
 Payment Verification
   and Settlement
        |
        v
   Content Unlocked
```

The x402 protocol enables HTTP resources to require payment before returning the requested content.

## Features

### Planned MVP

- [ ] Browse a catalogue of digital products
- [ ] Rust-based API using Axum
- [ ] Payment-gated content endpoints
- [ ] HTTP 402 payment requirements through x402
- [ ] USDC payments on Solana
- [ ] Payment verification and settlement
- [ ] Secure content delivery after successful payment
- [ ] Simple web storefront
- [ ] Transaction receipt and Solana Explorer link
- [ ] Automated tests for payment and access-control failures

### Future Enhancements

- [ ] AI agent that discovers and purchases content
- [ ] Spending limits for autonomous agents
- [ ] Multiple products with different prices
- [ ] Short-lived access tokens for protected downloads
- [ ] Machine-readable product catalogue for agent discovery
- [ ] Security testing for payment manipulation and replay attacks

## Technology Stack

| Technology | Purpose |
|---|---|
| Rust | Backend implementation |
| Axum | HTTP API and routing |
| Tokio | Asynchronous runtime |
| Serde | Serialization and JSON responses |
| Solana | Blockchain payment settlement |
| USDC | Payment currency |
| x402 | HTTP-native payment protocol |
| HTML, CSS, JavaScript | Lightweight storefront |
| Solana Devnet | Development and testing |

The project will build on Solana's Rust x402 template rather than reimplementing the payment protocol from scratch.

## Getting Started

### Prerequisites

Install the following tools:

- Rust and Cargo
- Node.js and npm
- Git
- A Solana wallet for development
- Access to a compatible x402 facilitator and Solana RPC endpoint

### 1. Clone the repository

```bash
git clone https://github.com/snufflesrea/solana-content-pay.git
cd solana-content-pay
```

### 2. Set up the official Rust x402 template

If you are starting from the official Solana template, generate the project first:

```bash
npx create-solana-dapp my-content-store --template x402-solana-rust
```

Select the Axum implementation when prompted, then follow the generated project's setup instructions.

> If you already cloned the generated template, do not generate a second project. Continue using the existing repository and adapt its protected endpoint to serve your products.

### 3. Configure environment variables

Create a local `.env` file using the names and format required by the selected template.

Typical configuration includes:

```dotenv
# Example placeholders — use the exact variable names
# expected by your chosen template.

SOLANA_RPC_URL=https://api.devnet.solana.com
NETWORK=devnet
PAYMENT_RECEIVER=YOUR_DEVNET_WALLET_ADDRESS
DEFAULT_PRICE=0.20
```

**Important:** These names are illustrative, not guaranteed to match the template's actual configuration. Check its environment example and source code before running it.

Never commit private keys, seed phrases, facilitator secrets, or production credentials to Git.

### 4. Run the application

Follow the commands in the generated template's README. For a standard Cargo-based Axum project, the server may be started with:

```bash
cargo run
```

Use the actual workspace directory and run command specified by the template.

### 5. Test a protected endpoint

The intended flow is:

1. Request a protected content endpoint without payment.
2. Receive an HTTP 402 response with payment requirements.
3. Have a compatible client produce the required payment authorization.
4. Submit the payment payload to the server.
5. Verify and settle the payment through the configured x402 flow.
6. Receive the requested content after successful payment.

Start on Devnet and use test funds. Do not use real funds until the payment flow and security checks have been thoroughly tested.

## API Design

The following endpoints describe the intended interface. They are planned endpoints and may change during implementation.

| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/api/products` | List available digital products |
| `GET` | `/api/products/:id` | Retrieve product metadata |
| `GET` | `/api/content/:id` | Request protected content; payment required |
| `GET` | `/health` | Check server health |

### Example product catalogue

```json
[
  {
    "id": "solana-audit-checklist",
    "name": "Solana Audit Checklist",
    "description": "A practical checklist for reviewing Solana programs.",
    "price": "0.20",
    "currency": "USDC"
  },
  {
    "id": "anchor-security-guide",
    "name": "Anchor Security Guide",
    "description": "Common security pitfalls in Anchor programs.",
    "price": "0.50",
    "currency": "USDC"
  }
]
```

### Example purchase flow

A client requests:

```http
GET /api/content/solana-audit-checklist
```

Without valid payment, the server responds:

```http
HTTP/1.1 402 Payment Required
```

The x402 middleware or handler supplies the protocol-compliant payment requirements. A compatible client then completes the payment flow and retries the request with the appropriate payment payload.

After successful verification and settlement, the server returns the protected content.

The actual payment headers and payload format must follow the x402 version implemented by the selected template.

### CLI Command
Check health : curl http://localhost:3000/api/health
Products list : curl http://localhost:3000/api/products
To buy the products: curl http://localhost:3000/api/product/:product_id

## Security Considerations

Payment-gated content introduces several trust boundaries. This project aims to test and document them explicitly.

### Payment validation

The server must validate the payment against the intended purchase, including:

- Correct Solana network
- Correct USDC mint
- Correct recipient
- Correct payment amount
- Valid payment authorization and transaction structure
- Payment validity and expiry
- Replay protection and duplicate-settlement handling
- Correct association between payment and product

### Content access control

- Never return protected content before successful payment authorization.
- Do not trust a transaction signature or client-supplied success flag alone.
- Do not allow payment for a cheaper product to unlock a more expensive product.
- Keep protected files outside publicly served static directories.
- Use cryptographically secure, short-lived access tokens when needed.
- Keep private keys and facilitator credentials out of the frontend.
- Apply request limits and sensible error handling.

### Security test scenarios

| Scenario | Expected result |
|---|---|
| No payment supplied | Payment required |
| Payment amount is insufficient | Reject access |
| Incorrect token mint | Reject access |
| Incorrect recipient | Reject access |
| Payment for a different product | Reject access |
| Expired or invalid payment | Reject access |
| Reused payment | Reject duplicate redemption |
| Valid payment for requested product | Grant access |

These scenarios are test goals; they should not be considered verified until automated tests demonstrate the expected behavior.

## Why x402?

x402 enables HTTP resources to request payment using the HTTP 402 status code and a standardized payment flow.

For this project, it offers several benefits:

- **Pay per item:** Customers pay for individual resources rather than subscribing.
- **Machine-readable payments:** Compatible software clients can process payment requirements.
- **Agent compatibility:** AI agents can discover prices and purchase resources programmatically.
- **On-chain settlement:** USDC payments can settle on Solana.
- **Reusable payment infrastructure:** The project can build on an existing protocol implementation.

## Roadmap

### Phase 1 — Functional MVP
- [x] Set up the official Rust/Axum template
- [x] Create a small product catalogue
- [ ] Add one protected content endpoint
- [ ] Configure Devnet USDC payments
- [ ] Verify payment and unlock content

### Phase 2 — Storefront
- [ ] Add product cards and prices
- [ ] Integrate a compatible wallet/client
- [ ] Display payment status and transaction receipts
- [ ] Deploy a public demo

### Phase 3 — Security and Reliability
- [ ] Add payment-validation tests
- [ ] Test replay and product-substitution scenarios
- [ ] Add rate limiting and error handling
- [ ] Document trust assumptions and known limitations

### Phase 4 — Agentic Commerce
- [ ] Expose a machine-readable product catalogue
- [ ] Build an agent-compatible purchase client
- [ ] Add configurable spending limits
- [ ] Demonstrate autonomous content discovery and purchase

## Resources

- [Solana Documentation](https://solana.com/docs)
- [Solana x402 Rust Template](https://solana.com/developers/templates/x402-solana-rust)
- [Solana Agentic Payments](https://solana.com/docs/payments/agentic-payments)
- [x402 Protocol](https://www.x402.org/)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)

## Disclaimer

This project is for educational and portfolio purposes. It is not audited or intended to handle production funds in its initial form. Payment verification, content authorization, wallet security, and deployment configuration must be reviewed before mainnet use.

## License

MIT
