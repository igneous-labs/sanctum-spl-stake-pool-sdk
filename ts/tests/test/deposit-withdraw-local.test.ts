import * as kit from "@solana/kit";
import { describe, it, assert } from "vitest";
import {
  readTestFixturesAccPk,
  readTestFixturesJsonFile,
  readTestFixturesKeypair,
} from "./utils";
import {
  depositSolIxFromStakePool,
  depositStakeIxFromStakePool,
  deserStakePool,
  getStakePool,
  quoteDepositSol,
  quoteDepositStake,
  quoteWithdrawSol,
  quoteWithdrawStake,
  withdrawSolIxFromStakePool,
  withdrawStakeIxFromStakePool,
} from "@sanctumso/spl-stake-pool";

/**
 * Requires a local validator running with test fixtures.
 *
 * Tests in this suite must be ran sequentially because they all
 * send transactons that modify the state of the following same accounts:
 *
 * - picosol-manager-fee
 * - signer-picosol-token
 * - referral-picosol-token
 */
describe("picosol-quote-sim-local", async () => {
  // For some reason the first sequential test always take a long time
  // ~10s to complete, regardless of which user action it is (deposit/withdraw sol/stake)

  it.sequential("deposit-sol", async () => {
    const keypair = await readTestFixturesKeypair("signer");
    const referralToken = readTestFixturesAccPk("referral-picosol-token");
    const signerToken = readTestFixturesAccPk("signer-picosol-token");

    let rpcClient = kit.createSolanaRpc("http://localhost:8899");
    const rpcClientSubscriptions = kit.createSolanaRpcSubscriptions(
      "ws://localhost:8900"
    );

    // Fetching the stake pool state for testing to enable multiple tests running on same validator
    const accountJson = readTestFixturesJsonFile("picosol-stake-pool");
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

    const quote = quoteDepositSol(stakePoolHandlePre, 1000000n);

    // Deposit SOL instruction
    const { value: blockhash } = await rpcClient.getLatestBlockhash().send();

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
      latestStakePool.poolTokenSupply - stakePoolPre.poolTokenSupply,
      quote.outAmount + quote.managerFee + quote.referralFee
    );
    assert.strictEqual(
      latestStakePool.totalLamports - stakePoolPre.totalLamports,
      quote.inAmount
    );
  });

  it.sequential("deposit-stake", async () => {
    const keypair = await readTestFixturesKeypair("signer");
    const referralToken = readTestFixturesAccPk("referral-picosol-token");
    const signerToken = readTestFixturesAccPk("signer-picosol-token");

    let rpcClient = kit.createSolanaRpc("http://localhost:8899");
    const rpcClientSubscriptions = kit.createSolanaRpcSubscriptions(
      "ws://localhost:8900"
    );

    // Fetching the stake pool state for testing to enable multiple tests running on same validator
    const accountJson = readTestFixturesJsonFile("picosol-stake-pool");
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
        depositStake: readTestFixturesAccPk("deposit-stake"),
        validatorVote: readTestFixturesAccPk("picosol-vote-account"),
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

  it.sequential("withdraw-sol", async () => {
    const keypair = await readTestFixturesKeypair("signer");

    const signerToken = readTestFixturesAccPk("signer-picosol-token");

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
    const accountJson = readTestFixturesJsonFile("picosol-stake-pool");
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

  it.sequential("withdraw-stake", async () => {
    const keypair = await readTestFixturesKeypair("signer");

    const signerToken = readTestFixturesAccPk("signer-picosol-token");

    let rpcClient = kit.createSolanaRpc("http://localhost:8899");
    const rpcClientSubscriptions = kit.createSolanaRpcSubscriptions(
      "ws://localhost:8900"
    );

    // Fetching the stake pool state for testing to enable multiple tests running on same validator
    const accountJson = readTestFixturesJsonFile("picosol-stake-pool");
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

    const stakeToReceive = readTestFixturesAccPk("uninitialized-stake");
    let ix = withdrawStakeIxFromStakePool(
      {
        program: "SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY",
        stakePool: accountJson.pubkey,
        stakeToSplit: readTestFixturesAccPk("picosol-validator-stake"),
        stakeToReceive,
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
      .getAccountInfo(stakeToReceive, {
        encoding: "base64",
      })
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
