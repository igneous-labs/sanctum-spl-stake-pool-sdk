/**
 * Sets SPL token mint supply.
 *
 */

import { Base64EncodedBytes, getBase64Codec, getU64Encoder } from "@solana/kit";
import { readTestFixturesAcc, writeTestFixturesAcc } from "../utils";

const B64 = getBase64Codec();
const SPL_MINT_SUPPLY_OFFSET = 36;

function main() {
  const [_node, _script, fixture, supplyStr] = process.argv;

  if (!fixture || !supplyStr) {
    console.log("Usage: SCRIPT <test-fixtures-fname-no-json> <supply-u64>");
    return;
  }

  const acc = readTestFixturesAcc(fixture);
  const supply = BigInt(supplyStr);

  const accData = new Uint8Array(B64.encode(acc.account.data[0]));
  accData.set(getU64Encoder().encode(supply), SPL_MINT_SUPPLY_OFFSET);

  acc.account.data[0] = B64.decode(accData) as Base64EncodedBytes;
  writeTestFixturesAcc(fixture, acc);
}

main();
