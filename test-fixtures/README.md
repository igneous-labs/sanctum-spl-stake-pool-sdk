# test-fixtures

Any account prefixed with `signer-` has authority set to `signer.json`.

For example, `signer-picosol-token-deposit-sol.json` is a picoSOL spl token account with owner set to `signer.json`.

This can be achieved from existing accounts by editing their raw bytes, re-encoding it to base64 and updating the test-fixtures json account file.

## Programs Notes

All 3 deploys of SPL currently use the exact same binary, `programs/stake-pool.so`

## Accounts Notes

- `deposit-stake`

  - activation epoch changed to 0
  - stake reduced to 100 SOL

  So that its fully active for the test

- picosol stake pool data collected at epoch 787:

  - fee settings (changes made to make sure we cover different test cases):
    - epoch fee unchanged at 25/1000
    - stake deposit fee changed from 0/100 to 1/10000
    - stake withdrawal fee unchanged at 1/1000
    - sol deposit fee changed from 0/100 to 1/5000
  - vsa

    - activation epoch changed to 0
    - stake reduced to 1000 SOL

    So that its fully active for the test

  - picosol mint supply and recorded mint supply changed to 750 to be more realistic following vsa changes

  - changed last update epoch of all involved accounts (pool, validator list) to 1
