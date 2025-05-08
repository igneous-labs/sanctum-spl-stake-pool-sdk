import { readFileSync } from "fs";
import {
  cleanupRemovedValidatorEntriesIxFromStakePool,
  depositSolIxFromStakePool,
  depositStakeIxFromStakePool,
  deserStakePool,
  deserValidatorList,
  findWithdrawAuthPda,
  getStakePool,
  getValidatorList,
  increaseAdditionalValidatorStakeIxFromStakePool,
  initializeIx,
  quoteDepositSol,
  quoteDepositStake,
  quoteWithdrawSol,
  quoteWithdrawStake,
  serStakePool,
  serValidatorList,
  updateStakePoolBalanceIxFromStakePool,
  updateValidatorListBalanceIxFromStakePool,
  withdrawSolIxFromStakePool,
  withdrawStakeIxFromStakePool,
} from "sanctum-spl-stake-pool-ts";
import * as kit from "@solana/kit";
import assert from "assert";
import { setTransactionMessageLifetimeUsingBlockhash } from "@solana/kit";
import signerKey from "../../test-fixtures/key/signer.json";

function read_test_fixtures_json_file(fname: string): any {
  return JSON.parse(
    readFileSync(
      `${import.meta.dirname}/../../test-fixtures/${fname}.json`,
      "utf8"
    )
  );
}

function test_stake_pool() {
  const accountJson = read_test_fixtures_json_file("jupSolStakePool");
  const accountData = Buffer.from(accountJson.account.data[0], "base64");
  const bytes = new Uint8Array(accountData);

  // Deserialization + Getters
  const handle = deserStakePool(bytes)!;
  const stakePool = getStakePool(handle);
  assert.equal(stakePool.accountType, "StakePool");
  assert.equal(stakePool.totalLamports, 4135211783809274);
  assert.equal(stakePool.poolTokenSupply, 3792758591416065);

  // Setters
  const oldManager = stakePool.manager;
  const newAddress = "AxZfZWeqztBCL37Mkjkd4b8Hf6J13WCcfozrBY6vZzv3";
  stakePool.manager = newAddress;
  assert.deepStrictEqual(stakePool.manager, newAddress);

  // Serialization
  stakePool.manager = oldManager;
  const serialized = serStakePool(handle);
  assert.deepStrictEqual(serialized.slice(0, 435), bytes.slice(0, 435));
}

function test_validator_list() {
  const validatorListJson = read_test_fixtures_json_file("validatorList");
  const validatorListData = Buffer.from(
    validatorListJson.account.data[0],
    "base64"
  );
  const validatorListBytes = new Uint8Array(validatorListData);

  // Deserialization + Getters
  const validatorListHandle = deserValidatorList(validatorListBytes);
  const validatorList = getValidatorList(validatorListHandle);
  assert.equal(validatorList.header.account_type, "ValidatorList");
  assert.equal(validatorList.header.max_validators, 10000);
  assert.equal(validatorList.validators.length, 3);
  const firstValidator = validatorList.validators[0];
  assert.equal(firstValidator.activeStakeLamports, 2947319964963369);
  assert.equal(firstValidator.transientStakeLamports, 0);
  assert.equal(firstValidator.lastUpdateEpoch, 751);

  // Serialization
  const serialized = serValidatorList(validatorListHandle);
  assert.deepStrictEqual(serialized, validatorListBytes);
}

function test_pdas() {
  const pool = "8VpRhuxa7sUUepdY3kQiTmX9rS5vx4WgaXiAnXq4KCtr";
  const programId = "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn";
  const [withdrawAuth] = findWithdrawAuthPda(programId, pool);
  assert.deepStrictEqual(
    withdrawAuth,
    "EMjuABxELpYWYEwjkKmQKBNCwdaFAy4QYAs6W9bDQDNw"
  );
}

async function test_update_stake_pool_balance() {
  const accountJson = read_test_fixtures_json_file("jupSolStakePool");
  const accountData = Buffer.from(accountJson.account.data[0], "base64");
  const bytes = new Uint8Array(accountData);
  const stakePoolHandle = deserStakePool(bytes);
  const stakePool = getStakePool(stakePoolHandle);

  let ix = updateStakePoolBalanceIxFromStakePool(
    {
      program: "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn",
      stakePool: "8VpRhuxa7sUUepdY3kQiTmX9rS5vx4WgaXiAnXq4KCtr",
    },
    stakePoolHandle
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
      setTransactionMessageLifetimeUsingBlockhash(
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
  assert.strictEqual(simulation.value.err, null);
}

async function test_update_validator_list_balance() {
  const accountJson = read_test_fixtures_json_file("jupSolStakePool");
  const accountData = Buffer.from(accountJson.account.data[0], "base64");
  const bytes = new Uint8Array(accountData);
  const stakePoolHandle = deserStakePool(bytes);
  const stakePool = getStakePool(stakePoolHandle);

  const validatorListJson = read_test_fixtures_json_file("validatorList");
  const validatorListData = Buffer.from(
    validatorListJson.account.data[0],
    "base64"
  );
  const validatorListBytes = new Uint8Array(validatorListData);
  const validatorListHandle = deserValidatorList(validatorListBytes);

  let ix = updateValidatorListBalanceIxFromStakePool(
    {
      program: "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn",
      stakePool: "8VpRhuxa7sUUepdY3kQiTmX9rS5vx4WgaXiAnXq4KCtr",
    },
    stakePoolHandle,
    validatorListHandle,
    {
      startIndex: 0,
      noMerge: false,
      count: 3,
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
      setTransactionMessageLifetimeUsingBlockhash(
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

  assert.strictEqual(simulation.value.err, null);
}

async function test_cleanup_removed_validator_entries() {
  const accountJson = read_test_fixtures_json_file("jupSolStakePool");
  const accountData = Buffer.from(accountJson.account.data[0], "base64");
  const bytes = new Uint8Array(accountData);
  const stakePoolHandle = deserStakePool(bytes);
  const stakePool = getStakePool(stakePoolHandle);

  const ix = cleanupRemovedValidatorEntriesIxFromStakePool(
    {
      program: "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn",
      stakePool: "8VpRhuxa7sUUepdY3kQiTmX9rS5vx4WgaXiAnXq4KCtr",
    },
    stakePoolHandle
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
      setTransactionMessageLifetimeUsingBlockhash(
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
  assert.strictEqual(simulation.value.err, null);
}

async function test_increase_additional_validator_stake() {
  const accountJson = read_test_fixtures_json_file("jupSolStakePool");
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
      setTransactionMessageLifetimeUsingBlockhash(
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
}

// Requires a local validator running with test fixtures
async function test_initialize() {
  let keypair = await kit.createKeyPairSignerFromBytes(
    new Uint8Array(signerKey)
  );

  let ix = initializeIx(
    {
      program: "SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY",
      stakePool: "ETVc1GBAiKzv2gNaA3Hfq4hsS1Mzh1NwQSxRFst7k8vz",
      manager: keypair.address,
      managerFee: "oGNCWtCuDs48gDjGCFwDkoFH1ZWLwehaYYcAoe6fCLD",
      staker: keypair.address,
      validatorList: "A2fm8gqbBHDcirKM2Ciqo7h9dg9FJeJZqpLGGJdDhBJq",
      reserve: "C6nDiFyQH8vbVyfGhgpCfzWbHixf5Kq3MUN5vFCdJ4qP",
      poolMint: "BPSoLzmLQn47EP5aa7jmFngRL8KC3TWAeAwXwZD8ip3P",
      poolTokenProgram: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    },
    {
      fee: {
        denominator: 100n,
        numerator: 1n,
      },
      withdrawalFee: {
        denominator: 100n,
        numerator: 1n,
      },
      depositFee: {
        denominator: 100n,
        numerator: 1n,
      },
      referralFee: 0,
      maxValidators: 10,
    }
  ) as unknown as kit.IInstruction;

  let rpcClient = kit.createSolanaRpc("http://localhost:8899");

  const simulatedTx = kit.pipe(
    kit.createTransactionMessage({
      version: 0,
    }),
    (txm) => kit.appendTransactionMessageInstructions([ix], txm),
    (txm) =>
      kit.setTransactionMessageFeePayer(
        keypair.address as kit.Address<string>,
        txm
      ),
    (txm) =>
      setTransactionMessageLifetimeUsingBlockhash(
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

  assert.strictEqual(simulation.value.err, null);
}

// Requires a local validator running with test fixtures
async function test_deposit_sol_and_deposit_sol_quote() {
  let keypair = await kit.createKeyPairSignerFromBytes(
    new Uint8Array(signerKey)
  );
  let referralToken = kit.address(
    "5YSa7x36xXZC3fRRbh3tEGAeE3penVphkAhpwDi6GCJM"
  );
  let signerToken = kit.address("D3aouXxyPdDrnb8Q2KybMDdEyBXJaP3r2mHzN3EmZWj9");

  let rpcClient = kit.createSolanaRpc("http://localhost:8899");
  const rpcClientSubscriptions = kit.createSolanaRpcSubscriptions(
    "ws://localhost:8900"
  );

  // Fetching the stake pool state for testing to enable multiple tests running on same validator
  const accountJson = read_test_fixtures_json_file("pico-sol-stake-pool");
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

  const validatorListJson = read_test_fixtures_json_file(
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
    (txm) => setTransactionMessageLifetimeUsingBlockhash(blockhash, txm)
  );

  const preconSignedTx = await kit.signTransactionMessageWithSigners(preconTx);
  const preconSendAndConfirmTx = kit.sendAndConfirmTransactionFactory({
    rpc: rpcClient,
    rpcSubscriptions: rpcClientSubscriptions,
  });

  await preconSendAndConfirmTx(preconSignedTx, {
    commitment: "confirmed",
  });

  const referralTokenBalanceBefore = BigInt(
    (await rpcClient.getTokenAccountBalance(referralToken).send()).value.amount
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
    (txm) => setTransactionMessageLifetimeUsingBlockhash(blockhash, txm)
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
    (await rpcClient.getTokenAccountBalance(referralToken).send()).value.amount
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
}

// Requires a local validator running with test fixtures
async function test_deposit_stake_and_deposit_stake_quote() {
  let keypair = await kit.createKeyPairSignerFromBytes(
    new Uint8Array(signerKey)
  );
  let referralToken = kit.address(
    "5YSa7x36xXZC3fRRbh3tEGAeE3penVphkAhpwDi6GCJM"
  );
  let signerToken = kit.address("D3aouXxyPdDrnb8Q2KybMDdEyBXJaP3r2mHzN3EmZWj9");

  let rpcClient = kit.createSolanaRpc("http://localhost:8899");
  const rpcClientSubscriptions = kit.createSolanaRpcSubscriptions(
    "ws://localhost:8900"
  );

  // Fetching the stake pool state for testing to enable multiple tests running on same validator
  const accountJson = read_test_fixtures_json_file("pico-sol-stake-pool");
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
    (await rpcClient.getTokenAccountBalance(referralToken).send()).value.amount
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
    (txm) => setTransactionMessageLifetimeUsingBlockhash(blockhash, txm)
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
    (await rpcClient.getTokenAccountBalance(referralToken).send()).value.amount
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
}

async function test_withdraw_sol_and_withdraw_sol_quote() {
  let keypair = await kit.createKeyPairSignerFromBytes(
    new Uint8Array(signerKey)
  );

  let signerToken = kit.address("D3aouXxyPdDrnb8Q2KybMDdEyBXJaP3r2mHzN3EmZWj9");

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
  const accountJson = read_test_fixtures_json_file("pico-sol-stake-pool");
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
    (txm) => setTransactionMessageLifetimeUsingBlockhash(blockhash, txm)
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
}

async function test_withdraw_stake_and_withdraw_stake_quote() {
  let keypair = await kit.createKeyPairSignerFromBytes(
    new Uint8Array(signerKey)
  );

  let signerToken = kit.address("D3aouXxyPdDrnb8Q2KybMDdEyBXJaP3r2mHzN3EmZWj9");

  let rpcClient = kit.createSolanaRpc("http://localhost:8899");
  const rpcClientSubscriptions = kit.createSolanaRpcSubscriptions(
    "ws://localhost:8900"
  );

  // Fetching the stake pool state for testing to enable multiple tests running on same validator
  const accountJson = read_test_fixtures_json_file("pico-sol-stake-pool");
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
    (txm) => setTransactionMessageLifetimeUsingBlockhash(blockhash, txm)
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
}

async function main() {
  test_stake_pool();
  test_validator_list();
  test_pdas();
  await test_update_stake_pool_balance();
  await test_update_validator_list_balance();
  await test_cleanup_removed_validator_entries();
  await test_increase_additional_validator_stake();
  await test_deposit_sol_and_deposit_sol_quote();
  await test_initialize();
  await test_deposit_stake_and_deposit_stake_quote();
  await test_withdraw_sol_and_withdraw_sol_quote();
  await test_withdraw_stake_and_withdraw_stake_quote();
  console.log("All tests passed!");
}

main();
