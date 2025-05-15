import { describe, it, assert } from "vitest";
import { readTestFixturesAccPk, readTestFixturesKeypair } from "./utils";
import * as kit from "@solana/kit";
import { initializeIx } from "@sanctumso/spl-stake-pool";

describe("initialize", async () => {
  // Requires a local validator running with test fixtures.
  // This requires the backpackSOL fixtures to be present and it initializes the pool
  // with the following accounts already existing onchain:
  // - uninitialized stake-pool
  // - uninitialized validator-list
  // - initialized mint
  // - initialized reserve stake
  // - initialized manager fee destination
  it("initialize-sim-local", async () => {
    const keypair = await readTestFixturesKeypair("signer");

    let ix = initializeIx(
      {
        program: "SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY",
        stakePool: readTestFixturesAccPk("bpsol-stake-pool"),
        manager: keypair.address,
        managerFee: readTestFixturesAccPk("bpsol-manager-fee"),
        staker: keypair.address,
        validatorList: readTestFixturesAccPk("bpsol-validator-list"),
        reserve: readTestFixturesAccPk("bpsol-reserve"),
        poolMint: readTestFixturesAccPk("bpsol-mint"),
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
