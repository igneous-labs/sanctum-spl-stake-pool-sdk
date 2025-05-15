import { describe, it, assert } from "vitest";
import { readTestFixturesJsonFile, readTestFixturesKeypair } from "./utils";
import * as kit from "@solana/kit";
import {
  depositStakeIxFromStakePool,
  deserStakePool,
  getStakePool,
  quoteDepositStake,
} from "@sanctumso/spl-stake-pool";

describe("deposit-stake", async () => {
  // Requires a local validator running with test fixtures
  it("deposit-stake-quote-sim-local", async () => {
    const keypair = await readTestFixturesKeypair("signer");
    let referralToken = kit.address(
      "5YSa7x36xXZC3fRRbh3tEGAeE3penVphkAhpwDi6GCJM"
    );
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

    let quote = quoteDepositStake(stakePoolHandle, {
      staked: 117349565740842n,
      unstaked: 2282880n,
    });

    const signerTokenBalanceBefore = BigInt(
      (await rpcClient.getTokenAccountBalance(signerToken).send()).value.amount
    );
    const referralTokenBalanceBefore = BigInt(
      (await rpcClient.getTokenAccountBalance(referralToken).send()).value
        .amount
    );
    const managerFeeTokenBalanceBefore = BigInt(
      (
        await rpcClient
          .getTokenAccountBalance(kit.address(stakePool.managerFeeAccount))
          .send()
      ).value.amount
    );

    let ix = depositStakeIxFromStakePool(
      {
        program: "SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY",
        stakePool: accountJson.pubkey,
        depositStake: "9WE9mV97A7oKZjwLf32c9r9darZMMM93fkbmPd9m8wWY",
        validatorVote: "3xjfK9C9YNcta8MvK1US4sQ3bc6DEjoJoR3qLExGf9xE",
        poolTokensTo: signerToken,
        referralPoolTokens: referralToken,
      },
      stakePoolHandle
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
    const referralTokenBalanceAfter = BigInt(
      (await rpcClient.getTokenAccountBalance(referralToken).send()).value
        .amount
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

    assert.strictEqual(
      signerTokenBalanceAfter - signerTokenBalanceBefore,
      quote.tokensOut
    );
    assert.strictEqual(
      managerFeeTokenBalanceAfter - managerFeeTokenBalanceBefore,
      quote.managerFee
    );
    assert.strictEqual(
      referralTokenBalanceAfter - referralTokenBalanceBefore,
      quote.referralFee
    );

    assert.strictEqual(
      newStakePool.poolTokenSupply - stakePool.poolTokenSupply,
      quote.tokensOut + quote.managerFee + quote.referralFee
    );
    assert.strictEqual(
      newStakePool.totalLamports - stakePool.totalLamports,
      117349568023722n
    );
  });
});
