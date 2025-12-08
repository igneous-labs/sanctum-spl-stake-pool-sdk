import { describe, it, assert } from "vitest";
import {
  fetchStakePool,
  readTestFixturesAccPk,
  readTestFixturesJsonFile,
} from "./utils";
import {
  deserStakePool,
  getStakePool,
  initSyncEmbed,
  updateStakePoolBalanceIxFromStakePool,
} from "@sanctumso/spl-stake-pool";
import {
  appendTransactionMessageInstructions,
  blockhash,
  compileTransaction,
  createSolanaRpc,
  createTransactionMessage,
  getBase64EncodedWireTransaction,
  getBase64Encoder,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  type Address,
  type IInstruction,
} from "@solana/kit";

initSyncEmbed();

describe("update-stake-pool-balance", async () => {
  it("update-stake-pool-balance-sim-mainnet", async () => {
    const rpcClient = createSolanaRpc("https://api.mainnet-beta.solana.com");
    const poolPk = readTestFixturesAccPk("jupsol-stake-pool");

    const stakePoolHandle = await fetchStakePool(rpcClient, poolPk);

    let ix = updateStakePoolBalanceIxFromStakePool(
      {
        program: "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn",
        stakePool: readTestFixturesAccPk("jupsol-stake-pool"),
      },
      stakePoolHandle
    ) as unknown as IInstruction;

    const simulatedTx = pipe(
      createTransactionMessage({
        version: 0,
      }),
      (txm) => appendTransactionMessageInstructions([ix], txm),
      (txm) =>
        setTransactionMessageFeePayer(
          getStakePool(stakePoolHandle).manager as Address<string>,
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
    assert.strictEqual(simulation.value.err, null, `${simulation.value.logs}`);
  });
});
