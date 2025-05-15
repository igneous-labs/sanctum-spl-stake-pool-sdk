import { describe, it, assert } from "vitest";
import { readTestFixturesJsonFile, readTestFixturesKeypair } from "./utils";
import * as kit from "@solana/kit";
import {
  deserStakePool,
  getStakePool,
  quoteWithdrawStake,
  withdrawStakeIxFromStakePool,
} from "@sanctumso/spl-stake-pool";

describe("withdraw-stake", async () => {
  // Requires a local validator running with test fixtures
  it("withdraw-stake-quote-sim-local", async () => {
    const keypair = await readTestFixturesKeypair("signer");

    let signerToken = kit.address(
      "D3aouXxyPdDrnb8Q2KybMDdEyBXJaP3r2mHzN3EmZWj9"
    );

    let rpcClient = kit.createSolanaRpc("http://localhost:8899");
    const rpcClientSubscriptions = kit.createSolanaRpcSubscriptions(
      "ws://localhost:8900"
    );

    // Fetching the stake pool state for testing to enable multiple tests running on same validator
    const accountJson = readTestFixturesJsonFile("pico-sol-stake-pool");
    const stakePoolInfo = await rpcClient
      .getAccountInfo(kit.address(accountJson.pubkey), {
        encoding: "base64",
      })
      .send();
    const stakePoolData = new Uint8Array(
      kit.getBase64Encoder().encode(stakePoolInfo.value!.data[0])
    );
    const stakePoolHandle = deserStakePool(stakePoolData);
    const stakePool = getStakePool(stakePoolHandle);

    const signerTokenBalanceBefore = BigInt(
      (await rpcClient.getTokenAccountBalance(signerToken).send()).value.amount
    );
    const managerFeeTokenBalanceBefore = BigInt(
      (
        await rpcClient
          .getTokenAccountBalance(kit.address(stakePool.managerFeeAccount))
          .send()
      ).value.amount
    );

    let quote = quoteWithdrawStake(stakePoolHandle, 10000000n);

    let ix = withdrawStakeIxFromStakePool(
      {
        program: "SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY",
        stakePool: accountJson.pubkey,
        stakeToSplit: "ALkqRsYmCtWE1hx6VQPqtwnxwA8ymB5h8FzBNrF32SSN",
        stakeToReceive: "3Vegk2SB2SSFpiWXwp4k82xGxALpqSdu8DatZRyDenxE",
        userStakeAuth: keypair.address,
        userTransferAuth: keypair.address,
        poolTokensFrom: signerToken,
      },
      stakePoolHandle,
      {
        poolTokensIn: 10000000n,
      }
    ) as unknown as kit.IInstruction;

    const { value: blockhash } = await rpcClient.getLatestBlockhash().send();

    const tx = kit.pipe(
      kit.createTransactionMessage({
        version: 0,
      }),
      (txm) => kit.appendTransactionMessageInstructions([ix], txm),
      (txm) => kit.setTransactionMessageFeePayerSigner(keypair, txm),
      (txm) => kit.setTransactionMessageLifetimeUsingBlockhash(blockhash, txm)
    );

    const signedTx = await kit.signTransactionMessageWithSigners(tx);
    const sendAndConfirmTx = kit.sendAndConfirmTransactionFactory({
      rpc: rpcClient,
      rpcSubscriptions: rpcClientSubscriptions,
    });
    await sendAndConfirmTx(signedTx, {
      commitment: "confirmed",
    });

    let stakeAccountInfoAfter = await rpcClient
      .getAccountInfo(
        kit.address("3Vegk2SB2SSFpiWXwp4k82xGxALpqSdu8DatZRyDenxE"),
        {
          encoding: "base64",
        }
      )
      .send();

    let encoded = kit
      .getBase64Encoder()
      .encode(stakeAccountInfoAfter.value!.data[0]);

    let delegationStake = kit.getU64Codec().decode(encoded.slice(156, 164));

    // Refetching stake pool to get updated state
    const changedStakePool = await rpcClient
      .getAccountInfo(kit.address(accountJson.pubkey), {
        encoding: "base64",
      })
      .send();
    const newStakePoolData = new Uint8Array(
      kit.getBase64Encoder().encode(changedStakePool.value!.data[0])
    );
    const newStakePoolHandle = deserStakePool(newStakePoolData);
    const newStakePool = getStakePool(newStakePoolHandle);

    const signerTokenBalanceAfter = BigInt(
      (await rpcClient.getTokenAccountBalance(signerToken).send()).value.amount
    );
    const managerFeeTokenBalanceAfter = BigInt(
      (
        await rpcClient
          .getTokenAccountBalance(kit.address(stakePool.managerFeeAccount))
          .send()
      ).value.amount
    );

    assert.strictEqual(delegationStake, quote.lamportsStaked);
    assert.strictEqual(
      signerTokenBalanceBefore - signerTokenBalanceAfter,
      quote.tokensIn
    );
    assert.strictEqual(
      managerFeeTokenBalanceAfter - managerFeeTokenBalanceBefore,
      quote.feeAmount
    );
    assert.strictEqual(
      stakePool.totalLamports - newStakePool.totalLamports,
      quote.lamportsStaked
    );
  });
});
