/**
 * Overrides a stake account with desired data
 */

import { Base64EncodedBytes, getBase64Codec, type Lamports } from "@solana/kit";
import {
  getStakeStateV2Codec,
  stakeStateV2,
  type Meta,
  type Delegation,
} from "@solana-program/stake";
import { readTestFixturesAcc, U64_MAX, writeTestFixturesAcc } from "../utils";

/**
 * Edit these to desired changes
 */
const META_OVERRIDES: Partial<Meta> = {
  rentExemptReserve: 2282880n,
};
const DELEGATION_OVERRIDES: Partial<Delegation> = {
  stake: 100_000_000_000n,
  activationEpoch: 0n,
  deactivationEpoch: U64_MAX,
};

const STAKE_CODEC = getStakeStateV2Codec();
const B64 = getBase64Codec();

function main() {
  const [_node, _script, fixture] = process.argv;

  if (!fixture) {
    console.log("Usage: SCRIPT <test-fixtures-fname-no-json>");
    return;
  }

  const acc = readTestFixturesAcc(fixture);

  const accData = B64.encode(acc.account.data[0]);
  const stake = STAKE_CODEC.decode(accData);
  if (stake.__kind !== "Stake") {
    throw new Error("only works with StakeStateV2.state = Stake");
  }

  const newStake = stakeStateV2("Stake", [
    { ...stake.fields[0], ...META_OVERRIDES },
    {
      ...stake.fields[1],
      delegation: { ...stake.fields[1].delegation, ...DELEGATION_OVERRIDES },
    },
    stake.fields[2],
  ]);
  const lamports =
    BigInt(newStake.fields[1].delegation.stake) +
    BigInt(newStake.fields[0].rentExemptReserve);

  acc.account.data[0] = B64.decode(
    STAKE_CODEC.encode(newStake)
  ) as Base64EncodedBytes;
  acc.account.lamports = lamports as Lamports;
  writeTestFixturesAcc(fixture, acc);
}

main();
