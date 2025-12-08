import { describe, it, assert } from "vitest";
import { fetchStakePool, readTestFixturesAccPk } from "./utils";
import {
  getStakePool,
  decreaseAdditionalValidatorStakeIxFromStakePool,
  initSyncEmbed,
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

initSyncEmbed();

describe("decrease-additional-validator-stake", async () => {
  it("decrease-additional-validator-stake-sim-mainnet", async () => {
    const rpcClient = createSolanaRpc("https://api.mainnet-beta.solana.com");
    const poolPk = readTestFixturesAccPk("jupsol-stake-pool");

    const stakePoolHandle = await fetchStakePool(rpcClient, poolPk);
    const stakePool = getStakePool(stakePoolHandle);

    let ix = decreaseAdditionalValidatorStakeIxFromStakePool(
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
    //console.log(simulation.value.logs);
    //assert.strictEqual(simulation.value.err, null);
  });
});
