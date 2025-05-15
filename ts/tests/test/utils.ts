import {
  address,
  createKeyPairSignerFromBytes,
  type Address,
  type KeyPairSigner,
} from "@solana/kit";
import { readFileSync } from "fs";

export function readTestFixturesJsonFile(fname: string): any {
  return JSON.parse(
    readFileSync(
      `${import.meta.dirname}/../../../test-fixtures/${fname}.json`,
      "utf8"
    )
  );
}

export function readTestFixturesAccPk(fname: string): Address<string> {
  const { pubkey } = readTestFixturesJsonFile(fname);
  return address(pubkey);
}

export function readTestFixturesKeypair(
  fname: string
): Promise<KeyPairSigner<string>> {
  const bytes = JSON.parse(
    readFileSync(
      `${import.meta.dirname}/../../../test-fixtures/key/${fname}.json`,
      "utf8"
    )
  );
  return createKeyPairSignerFromBytes(new Uint8Array(bytes));
}
