import { describe, it, assert } from "vitest";
import { readTestFixturesJsonFile } from "./utils";
import {
  deserStakePool,
  getStakePool,
  updateStakePoolBalanceIxFromStakePool,
} from "@sanctumso/spl-stake-pool";
import * as kit from "@solana/kit";

describe("update-stake-pool-balance", async () => {
  it("update-stake-pool-balance-sim-mainnet", async () => {
    const accountJson = readTestFixturesJsonFile("jupSolStakePool");
    const accountData = Buffer.from(accountJson.account.data[0], "base64");
    const bytes = new Uint8Array(accountData);
    const stakePoolHandle = deserStakePool(bytes);
    const stakePool = getStakePool(stakePoolHandle);

    let ix = updateStakePoolBalanceIxFromStakePool(
      {
        program: "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn",
        stakePool: "8VpRhuxa7sUUepdY3kQiTmX9rS5vx4WgaXiAnXq4KCtr",
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
