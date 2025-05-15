import { createKeyPairSignerFromBytes, type KeyPairSigner } from "@solana/kit";
import { readFileSync } from "fs";

export function readTestFixturesJsonFile(fname: string): any {
  return JSON.parse(
    readFileSync(
      `${import.meta.dirname}/../../../test-fixtures/${fname}.json`,
      "utf8"
    )
  );
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
