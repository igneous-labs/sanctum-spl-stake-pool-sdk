import { describe, it, assert } from "vitest";
import { readTestFixturesAccPk, readTestFixturesJsonFile } from "./utils";
import {
  cleanupRemovedValidatorEntriesIxFromStakePool,
  deserStakePool,
  getStakePool,
} from "@sanctumso/spl-stake-pool";
import {
  appendTransactionMessageInstructions,
  blockhash,
  compileTransaction,
  createSolanaRpc,
  createTransactionMessage,
  getBase64EncodedWireTransaction,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  type Address,
  type IInstruction,
} from "@solana/kit";

describe("cleanup-removed-validator-entries", async () => {
  it("cleanup-removed-validator-entries-sim-mainnet", async () => {
    const accountJson = readTestFixturesJsonFile("jupsol-stake-pool");
    const accountData = Buffer.from(accountJson.account.data[0], "base64");
    const bytes = new Uint8Array(accountData);
    const stakePoolHandle = deserStakePool(bytes);
    const stakePool = getStakePool(stakePoolHandle);

    const ix = cleanupRemovedValidatorEntriesIxFromStakePool(
      {
        program: "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn",
        stakePool: readTestFixturesAccPk("jupsol-stake-pool"),
      },
      stakePoolHandle
    ) as unknown as IInstruction;

    let rpcClient = createSolanaRpc("https://api.mainnet-beta.solana.com");

    const simulatedTx = pipe(
      createTransactionMessage({
        version: 0,
      }),
      (txm) => appendTransactionMessageInstructions([ix], txm),
      (txm) =>
        setTransactionMessageFeePayer(
          stakePool.manager as Address<string>,
          txm
        ),
      (txm) =>
        setTransactionMessageLifetimeUsingBlockhash(
          {
            blockhash: blockhash("11111111111111111111111111111111"),
            lastValidBlockHeight: 0n,
          },
          txm
        ),
      compileTransaction
    );

    const simulation = await rpcClient
      .simulateTransaction(getBase64EncodedWireTransaction(simulatedTx), {
        encoding: "base64",
        sigVerify: false,
        replaceRecentBlockhash: true,
      })
      .send();
    assert.strictEqual(simulation.value.err, null);
  });
});
