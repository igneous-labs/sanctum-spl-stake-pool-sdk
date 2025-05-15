import { describe, it, assert } from "vitest";
import { readTestFixturesJsonFile } from "./utils";
import {
  deserStakePool,
  deserValidatorList,
  getStakePool,
  getValidatorList,
  serStakePool,
  serValidatorList,
} from "@sanctumso/spl-stake-pool";

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
});
