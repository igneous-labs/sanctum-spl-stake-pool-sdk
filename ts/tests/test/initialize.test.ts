import { describe, it, assert } from "vitest";
import { readTestFixturesKeypair } from "./utils";
import * as kit from "@solana/kit";
import { initializeIx } from "@sanctumso/spl-stake-pool";

describe("initialize", async () => {
  // Requires a local validator running with test fixtures
  it("initialize-sim-local", async () => {
    const keypair = await readTestFixturesKeypair("signer");

    let ix = initializeIx(
      {
        program: "SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY",
        stakePool: "ETVc1GBAiKzv2gNaA3Hfq4hsS1Mzh1NwQSxRFst7k8vz",
        manager: keypair.address,
        managerFee: "oGNCWtCuDs48gDjGCFwDkoFH1ZWLwehaYYcAoe6fCLD",
        staker: keypair.address,
        validatorList: "A2fm8gqbBHDcirKM2Ciqo7h9dg9FJeJZqpLGGJdDhBJq",
        reserve: "C6nDiFyQH8vbVyfGhgpCfzWbHixf5Kq3MUN5vFCdJ4qP",
        poolMint: "BPSoLzmLQn47EP5aa7jmFngRL8KC3TWAeAwXwZD8ip3P",
        poolTokenProgram: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
      },
      {
        fee: {
          denominator: 100n,
          numerator: 1n,
        },
        withdrawalFee: {
          denominator: 100n,
          numerator: 1n,
        },
        depositFee: {
          denominator: 100n,
          numerator: 1n,
        },
        referralFee: 0,
        maxValidators: 10,
      }
    ) as unknown as kit.IInstruction;

    let rpcClient = kit.createSolanaRpc("http://localhost:8899");

    const simulatedTx = kit.pipe(
      kit.createTransactionMessage({
        version: 0,
      }),
      (txm) => kit.appendTransactionMessageInstructions([ix], txm),
      (txm) =>
        kit.setTransactionMessageFeePayer(
          keypair.address as kit.Address<string>,
          txm
        ),
      (txm) =>
        kit.setTransactionMessageLifetimeUsingBlockhash(
          {
            blockhash: kit.blockhash("11111111111111111111111111111111"),
            lastValidBlockHeight: 0n,
          },
          txm
        ),
      kit.compileTransaction
    );

    const simulation = await rpcClient
      .simulateTransaction(kit.getBase64EncodedWireTransaction(simulatedTx), {
        encoding: "base64",
        sigVerify: false,
        replaceRecentBlockhash: true,
      })
      .send();

    assert.strictEqual(simulation.value.err, null);
  });
});
