import { describe, it, assert } from "vitest";
import { randPubkey, readTestFixturesJsonFile } from "./utils";
import {
  defaultStakePool,
  defaultValidatorList,
  deserStakePool,
  deserValidatorList,
  getStakePool,
  getValidatorList,
  initSyncEmbed,
  serStakePool,
  serValidatorList,
  setStakePool,
  setValidatorList,
} from "@sanctumso/spl-stake-pool";

initSyncEmbed();

describe("serde", () => {
  it("de-stakepool-fixture-mut-then-ser", () => {
    const accountJson = readTestFixturesJsonFile("jupsol-stake-pool");
    const accountData = Buffer.from(accountJson.account.data[0], "base64");
    const bytes = new Uint8Array(accountData);

    // Deserialization + Getters
    const handle = deserStakePool(bytes)!;
    const stakePool = getStakePool(handle);
    assert.equal(stakePool.accountType, "StakePool");
    assert.equal(stakePool.totalLamports, 4135211783809274n);
    assert.equal(stakePool.poolTokenSupply, 3792758591416065n);

    // Setters
    const oldManager = stakePool.manager;
    const newAddress = "AxZfZWeqztBCL37Mkjkd4b8Hf6J13WCcfozrBY6vZzv3";
    stakePool.manager = newAddress;
    assert.deepStrictEqual(stakePool.manager, newAddress);

    // Serialization
    stakePool.manager = oldManager;
    const serialized = serStakePool(handle);
    assert.deepStrictEqual(serialized.slice(0, 435), bytes.slice(0, 435));
  });

  it("de-validatorlist-fixture-mut-then-ser", () => {
    const validatorListJson = readTestFixturesJsonFile("validator-list");
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
    assert.equal(firstValidator.activeStakeLamports, 2947319964963369n);
    assert.equal(firstValidator.transientStakeLamports, 0n);
    assert.equal(firstValidator.lastUpdateEpoch, 751n);

    // Serialization
    const serialized = serValidatorList(validatorListHandle);
    assert.deepStrictEqual(serialized, validatorListBytes);
  });

  it("stakepool-set-get-round-trip", () => {
    const sp = {
      accountType: "StakePool" as const,
      manager: randPubkey(),
      staker: randPubkey(),
      stakeDepositAuthority: randPubkey(),
      stakeWithdrawBumpSeed: 0,
      validatorList: randPubkey(),
      reserveStake: randPubkey(),
      poolMint: randPubkey(),
      managerFeeAccount: randPubkey(),
      tokenProgramId: randPubkey(),
      totalLamports: 0n,
      poolTokenSupply: 0n,
      lastUpdateEpoch: 0n,
      lockup: {
        unixTimestamp: 0n,
        epoch: 0n,
        custodian: randPubkey(),
      },
      epochFee: { denominator: 0n, numerator: 0n },
      nextEpochFee: "None" as const,
      stakeDepositFee: { denominator: 0n, numerator: 0n },
      stakeWithdrawalFee: { denominator: 0n, numerator: 0n },
      nextStakeWithdrawalFee: "None" as const,
      stakeReferralFee: 0,
      solDepositFee: { denominator: 0n, numerator: 0n },
      solReferralFee: 0,
      solWithdrawalFee: { denominator: 0n, numerator: 0n },
      nextSolWithdrawalFee: "None" as const,
      lastEpochPoolTokenSupply: 0n,
      lastEpochTotalLamports: 0n,
      // undefined fields, need to explicitly declare
      // for deepStrictEqual
      preferredDepositValidatorVoteAddress: undefined,
      preferredWithdrawValidatorVoteAddress: undefined,
      solDepositAuthority: undefined,
      solWithdrawAuthority: undefined,
    };

    const sph = defaultStakePool();
    setStakePool(sph, sp);
    assert.deepStrictEqual(getStakePool(sph), sp);
  });

  it("validatorlist-set-get-round-trip", () => {
    const maxValidators = 9;
    const list = {
      header: {
        account_type: "ValidatorList" as const,
        max_validators: maxValidators,
      },
      validators: [...Array(maxValidators).keys()].map((_) => ({
        activeStakeLamports: 0n,
        transientStakeLamports: 0n,
        lastUpdateEpoch: 0n,
        transientSeedSuffix: 0n,
        validatorSeedSuffix: 0,
        status: "Active" as const,
        voteAccountAddress: randPubkey(),
      })),
    };

    const vlh = defaultValidatorList();
    setValidatorList(vlh, list);
    assert.deepStrictEqual(getValidatorList(vlh), list);
  });
});
