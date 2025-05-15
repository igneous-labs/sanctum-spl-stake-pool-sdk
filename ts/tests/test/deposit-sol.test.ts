import { describe, it, assert } from "vitest";
import { readTestFixturesJsonFile, readTestFixturesKeypair } from "./utils";
import * as kit from "@solana/kit";
import {
  depositSolIxFromStakePool,
  deserStakePool,
  deserValidatorList,
  getStakePool,
  quoteDepositSol,
  updateStakePoolBalanceIxFromStakePool,
  updateValidatorListBalanceIxFromStakePool,
} from "@sanctumso/spl-stake-pool";

describe("deposit-sol", async () => {
  // Requires a local validator running with test fixtures
  it("deposit-sol-quote-sim-local", async () => {
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
    const stakePoolInfoPre = await rpcClient
      .getAccountInfo(kit.address(accountJson.pubkey), {
        encoding: "base64",
      })
      .send();
    const stakePoolDataPre = new Uint8Array(
      kit.getBase64Encoder().encode(stakePoolInfoPre.value!.data[0])
    );
    const stakePoolHandlePre = deserStakePool(stakePoolDataPre);
    const stakePoolPre = getStakePool(stakePoolHandlePre);

    const validatorListJson = readTestFixturesJsonFile(
      "pico-sol-validator-list"
    );
    const validatorListData = Buffer.from(
      validatorListJson.account.data[0],
      "base64"
    );
    const validatorListBytes = new Uint8Array(validatorListData);
    const validatorListHandle = deserValidatorList(validatorListBytes);

    // Precondition instructions for correct quote

    let uvlPreconIx = updateValidatorListBalanceIxFromStakePool(
      {
        program: "SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY",
        stakePool: accountJson.pubkey,
      },
      stakePoolHandlePre,
      validatorListHandle,
      {
        startIndex: 0,
        noMerge: false,
        count: 1,
      }
    ) as unknown as kit.IInstruction;

    let uspbPreconIx = updateStakePoolBalanceIxFromStakePool(
      {
        program: "SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY",
        stakePool: accountJson.pubkey,
      },
      stakePoolHandlePre
    ) as unknown as kit.IInstruction;

    const { value: blockhash } = await rpcClient.getLatestBlockhash().send();
    const preconTx = kit.pipe(
      kit.createTransactionMessage({
        version: 0,
      }),
      (txm) =>
        kit.appendTransactionMessageInstructions(
          [uvlPreconIx, uspbPreconIx],
          txm
        ),
      (txm) => kit.setTransactionMessageFeePayerSigner(keypair, txm),
      (txm) => kit.setTransactionMessageLifetimeUsingBlockhash(blockhash, txm)
    );

    const preconSignedTx = await kit.signTransactionMessageWithSigners(
      preconTx
    );
    const preconSendAndConfirmTx = kit.sendAndConfirmTransactionFactory({
      rpc: rpcClient,
      rpcSubscriptions: rpcClientSubscriptions,
    });

    await preconSendAndConfirmTx(preconSignedTx, {
      commitment: "confirmed",
    });

    const referralTokenBalanceBefore = BigInt(
      (await rpcClient.getTokenAccountBalance(referralToken).send()).value
        .amount
    );
    const signerTokenBalanceBefore = BigInt(
      (await rpcClient.getTokenAccountBalance(signerToken).send()).value.amount
    );
    const managerFeeTokenBalanceBefore = BigInt(
      (
        await rpcClient
          .getTokenAccountBalance(kit.address(stakePoolPre.managerFeeAccount))
          .send()
      ).value.amount
    );

    // New stake pool data post precondition instructions

    const stakePoolHandleInfoPost = await rpcClient
      .getAccountInfo(kit.address(accountJson.pubkey), {
        encoding: "base64",
      })
      .send();
    const stakePoolDataPost = new Uint8Array(
      kit.getBase64Encoder().encode(stakePoolHandleInfoPost.value!.data[0])
    );
    const stakePoolHandlePost = deserStakePool(stakePoolDataPost);
    const stakePoolPost = getStakePool(stakePoolHandlePost);

    const quote = quoteDepositSol(stakePoolHandlePost, 1000000n);

    // Deposit SOL instruction

    let ix = depositSolIxFromStakePool(
      {
        program: "SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY",
        stakePool: accountJson.pubkey,
        referrerFee: referralToken,
        fromUserLamports: keypair.address,
        destUserPool: signerToken,
      },
      stakePoolHandlePre,
      {
        depositLamports: 1000000n,
      }
    ) as unknown as kit.IInstruction;

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

    const referralTokenBalanceAfter = BigInt(
      (await rpcClient.getTokenAccountBalance(referralToken).send()).value
        .amount
    );
    const signerTokenBalanceAfter = BigInt(
      (await rpcClient.getTokenAccountBalance(signerToken).send()).value.amount
    );
    const managerFeeTokenBalanceAfter = BigInt(
      (
        await rpcClient
          .getTokenAccountBalance(kit.address(stakePoolPre.managerFeeAccount))
          .send()
      ).value.amount
    );

    // Refetching stake pool to get updated state
    const latestStakePoolInfo = await rpcClient
      .getAccountInfo(kit.address(accountJson.pubkey), {
        encoding: "base64",
      })
      .send();
    const latestStakePoolData = new Uint8Array(
      kit.getBase64Encoder().encode(latestStakePoolInfo.value!.data[0])
    );
    const latestStakePoolHandle = deserStakePool(latestStakePoolData);
    const latestStakePool = getStakePool(latestStakePoolHandle);

    assert.strictEqual(
      referralTokenBalanceAfter - referralTokenBalanceBefore,
      quote.referralFee
    );
    assert.strictEqual(
      signerTokenBalanceAfter - signerTokenBalanceBefore,
      quote.outAmount
    );
    assert.strictEqual(
      managerFeeTokenBalanceAfter - managerFeeTokenBalanceBefore,
      quote.managerFee
    );
    assert.strictEqual(
      latestStakePool.poolTokenSupply - stakePoolPost.poolTokenSupply,
      quote.outAmount + quote.managerFee + quote.referralFee
    );
    assert.strictEqual(
      latestStakePool.totalLamports - stakePoolPost.totalLamports,
      quote.inAmount
    );
  });
});
