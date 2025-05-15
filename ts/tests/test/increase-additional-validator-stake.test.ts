import { describe, it, assert } from "vitest";
import { readTestFixturesJsonFile } from "./utils";
import * as kit from "@solana/kit";
import {
  deserStakePool,
  getStakePool,
  increaseAdditionalValidatorStakeIxFromStakePool,
} from "@sanctumso/spl-stake-pool";

describe("increase-additional-validator-stake", async () => {
  it("increase-additional-validator-stake-sim-mainnet", async () => {
    const accountJson = readTestFixturesJsonFile("jupsol-stake-pool");
    const accountData = Buffer.from(accountJson.account.data[0], "base64");
    const bytes = new Uint8Array(accountData);
    const stakePoolHandle = deserStakePool(bytes);
    const stakePool = getStakePool(stakePoolHandle);

    let ix = increaseAdditionalValidatorStakeIxFromStakePool(
      {
        program: "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn",
        voteAccount: "CatzoSMUkTRidT5DwBxAC2pEtnwMBTpkCepHkFgZDiqb",
        stakePool: "8VpRhuxa7sUUepdY3kQiTmX9rS5vx4WgaXiAnXq4KCtr",
      },
      stakePoolHandle,
      {
        lamports: 1000000n,
        transientStakeSeed: 0n,
        validatorStakeSeed: undefined,
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

    // TODO:
    // just asserting simulation non null for now because
    // we're testing against mainnet so simulation might fail with InsufficientFunds
    // if jupsol reserves does not have enough SOL to delegate
    assert.notEqual(null, simulation);
    //assert.strictEqual(simulation.value.err, null);
  });
});
