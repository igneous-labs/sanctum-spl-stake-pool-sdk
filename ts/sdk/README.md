# sanctum-spl-stake-pool-ts

Typescript + WASM SDK for the SPL stake pool program.

## Example Usage

```ts
import initSdk from "@sanctumso/spl-stake-pool";

// The SDK needs to be initialized once globally before it can be used (idempotent).
// For nodejs environments, use
// `import { initSyncEmbed } from "@sanctumso/spl-stake-pool"; initSyncEmbed();`
// instead
await initSdk();

// TODO
```

## Build

### Prerequisites

- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/)
- `make` (optional, you can just run the `wasm-pack` commands manually)
