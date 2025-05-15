import { describe, it, assert } from "vitest";
import { readTestFixturesJsonFile } from "./utils";
import * as kit from "@solana/kit";
import {
  deserStakePool,
  deserValidatorList,
  getStakePool,
  updateValidatorListBalanceIxFromStakePool,
} from "@sanctumso/spl-stake-pool";

describe("update-validator-list-balance", async () => {
  it("update-validator-list-balance-sim-mainnet", async () => {
    const accountJson = readTestFixturesJsonFile("jupSolStakePool");
    const accountData = Buffer.from(accountJson.account.data[0], "base64");
    const bytes = new Uint8Array(accountData);
    const stakePoolHandle = deserStakePool(bytes);
    const stakePool = getStakePool(stakePoolHandle);

    const validatorListJson = readTestFixturesJsonFile("validatorList");
    const validatorListData = Buffer.from(
      validatorListJson.account.data[0],
      "base64"
    );
    const validatorListBytes = new Uint8Array(validatorListData);
    const validatorListHandle = deserValidatorList(validatorListBytes);

    let ix = updateValidatorListBalanceIxFromStakePool(
      {
        program: "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn",
        stakePool: "8VpRhuxa7sUUepdY3kQiTmX9rS5vx4WgaXiAnXq4KCtr",
      },
      stakePoolHandle,
      validatorListHandle,
      {
        startIndex: 0,
        noMerge: false,
        count: 3,
      }
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
