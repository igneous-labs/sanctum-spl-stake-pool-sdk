Typescript tests for the ts sdk.

## Setup

`bun install`

## Run

Before running the tests, make sure the `ts/sdk` rust crate has been rebuilt:

```sh
cd ../sdk
make
```

Start the local validator with

```sh
docker compose -f ../../docker-compose-local-validator.yml up
```

Then run the test script with:

```sh
`bun ./index.ts`
```

Teardown local validator with

```sh
docker compose -f ../../docker-compose-local-validator.yml down
```
