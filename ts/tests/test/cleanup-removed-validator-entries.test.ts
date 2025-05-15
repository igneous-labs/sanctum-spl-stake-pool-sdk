import { describe, it, assert } from "vitest";
import { readTestFixturesAccPk, readTestFixturesJsonFile } from "./utils";
import * as kit from "@solana/kit";
import {
  cleanupRemovedValidatorEntriesIxFromStakePool,
  deserStakePool,
  getStakePool,
} from "@sanctumso/spl-stake-pool";

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
    ) as unknown as kit.IInstruction;

    let rpcClient = kit.createSolanaRpc("https://api.mainnet-beta.solana.com");

    const simulatedTx = kit.pipe(
      kit.createTransactionMessage({
        version: 0,
      }),
      (txm) => kit.appendTransactionMessageInstructions([ix], txm),
      (txm) =>
        kit.setTransactionMessageFeePayer(
          stakePool.manager as kit.Address<string>,
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
