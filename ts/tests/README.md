Typescript tests for the ts sdk.

## Setup

`pnpm install`

## Run

Before running the tests, make sure the `ts/sdk` rust crate has been rebuilt:

```sh
cd ../sdk
make
```

Start the local validator with

```sh
pnpm start:infra
```

Then run the test script with:

```sh
pnpm test
```

Teardown local validator with

```sh
pnpm stop:infra
```
