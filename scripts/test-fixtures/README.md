# test-fixtures scripts

Scripts for mutating and modifying test fixture accounts.

## Stake Pool Updating Hax

After updating the non-program accounts of a stake pool (eg stake accounts, pool token mint), you can run the local validator then run the update cranks on the stake pool and then obtain the updated stake pool/validator list account fixture by pulling from local.

```sh
$ solana config get
Config File: ~/.config/solana/cli/config.yml
RPC URL: http://localhost:8899
WebSocket URL: ws://localhost:8900/ (computed)
Keypair Path: ~/.config/solana/id.json
Commitment: confirmed

# start local validator with updated fixture
$ cd ../ts/tests
$ pnpm start:infra

$ solana airdrop 1
# Using our CLI at
# https://github.com/igneous-labs/sanctum-spl-stake-pool-cli
# This will update validator stake records and pool mint supply
$ splsp update -c force-all <stake-pool-addr>

$ solana account --output json -o stake-pool.json <stake-pool-addr>

$ pnpm stop:infra
```
