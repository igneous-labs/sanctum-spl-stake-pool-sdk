# test-fixtures

Any account prefixed with `signer-` has authority set to `signer.json`.

For example, `signer-picosol-token-deposit-sol.json` is a picoSOL spl token account with owner set to `signer.json`.

This can be achieved from existing accounts by editing their raw bytes, re-encoding it to base64 and updating the test-fixtures json account file.

## Stake Pools

- picosol stake pool data collected at epoch 787
