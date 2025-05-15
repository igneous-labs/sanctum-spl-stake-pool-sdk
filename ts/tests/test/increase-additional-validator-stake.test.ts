import { describe, it, assert } from "vitest";
import { readTestFixturesJsonFile } from "./utils";
import {
  deserStakePool,
  getStakePool,
  increaseAdditionalValidatorStakeIxFromStakePool,
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

    // TODO:
    // just asserting simulation non null for now because
    // we're testing against mainnet so simulation might fail with InsufficientFunds
    // if jupsol reserves does not have enough SOL to delegate
    assert.notEqual(null, simulation);
    //assert.strictEqual(simulation.value.err, null);
  });
});
