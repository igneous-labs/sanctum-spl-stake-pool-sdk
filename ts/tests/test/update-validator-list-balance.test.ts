import { describe, it, assert } from "vitest";
import { readTestFixturesAccPk, readTestFixturesJsonFile } from "./utils";
import {
  deserStakePool,
  deserValidatorList,
  getStakePool,
  updateValidatorListBalanceIxFromStakePool,
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

describe("update-validator-list-balance", async () => {
  it("update-validator-list-balance-sim-mainnet", async () => {
    const accountJson = readTestFixturesJsonFile("jupsol-stake-pool");
    const accountData = Buffer.from(accountJson.account.data[0], "base64");
    const bytes = new Uint8Array(accountData);
    const stakePoolHandle = deserStakePool(bytes);
    const stakePool = getStakePool(stakePoolHandle);

    const validatorListJson = readTestFixturesJsonFile("validator-list");
    const validatorListData = Buffer.from(
      validatorListJson.account.data[0],
      "base64"
    );
    const validatorListBytes = new Uint8Array(validatorListData);
    const validatorListHandle = deserValidatorList(validatorListBytes);

    let ix = updateValidatorListBalanceIxFromStakePool(
      {
        program: "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn",
        stakePool: readTestFixturesAccPk("jupsol-stake-pool"),
      },
      stakePoolHandle,
      validatorListHandle,
      {
        startIndex: 0,
        noMerge: false,
        count: 3,
      }
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
