import { describe, it, assert } from "vitest";
import { readTestFixturesJsonFile, readTestFixturesKeypair } from "./utils";
import * as kit from "@solana/kit";
import {
  deserStakePool,
  getStakePool,
  quoteWithdrawSol,
  withdrawSolIxFromStakePool,
} from "@sanctumso/spl-stake-pool";

describe("withdraw-sol", async () => {
  // Requires a local validator running with test fixtures
  it("withdraw-sol-quote-sim-local", async () => {
    const keypair = await readTestFixturesKeypair("signer");

    let signerToken = kit.address(
      "D3aouXxyPdDrnb8Q2KybMDdEyBXJaP3r2mHzN3EmZWj9"
    );

    let rpcClient = kit.createSolanaRpc("http://localhost:8899");
    const rpcClientSubscriptions = kit.createSolanaRpcSubscriptions(
      "ws://localhost:8900"
    );

    const signerInfo = await rpcClient
      .getAccountInfo(keypair.address, {
        encoding: "base64",
      })
      .send();

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

    let quote = quoteWithdrawSol(stakePoolHandle, 1000000n);

    let ix = withdrawSolIxFromStakePool(
      {
        program: "SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY",
        stakePool: accountJson.pubkey,
        userTransferAuth: keypair.address,
        poolTokensFrom: signerToken,
        lamportsTo: keypair.address,
      },
      stakePoolHandle,
      {
        poolTokensIn: 1000000n,
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

    const signerInfoAfter = await rpcClient
      .getAccountInfo(keypair.address, {
        encoding: "base64",
      })
      .send();

    assert.strictEqual(
      signerTokenBalanceBefore - signerTokenBalanceAfter,
      quote.inAmount
    );
    assert.strictEqual(
      managerFeeTokenBalanceAfter - managerFeeTokenBalanceBefore,
      quote.managerFee
    );
    assert.strictEqual(
      signerInfo.value!.lamports + quote.outAmount - 5000n,
      signerInfoAfter.value!.lamports
    );
    assert.strictEqual(
      stakePool.poolTokenSupply - newStakePool.poolTokenSupply,
      quote.inAmount - quote.managerFee
    );
    assert.strictEqual(
      stakePool.totalLamports - newStakePool.totalLamports,
      quote.outAmount
    );
  });
});
