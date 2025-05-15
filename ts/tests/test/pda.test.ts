import { findWithdrawAuthPda } from "@sanctumso/spl-stake-pool";
import { describe, it, assert } from "vitest";

describe("pda", () => {
  it("find-withdrawauth-basic", () => {
    const pool = "8VpRhuxa7sUUepdY3kQiTmX9rS5vx4WgaXiAnXq4KCtr";
    const programId = "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn";
    const [withdrawAuth] = findWithdrawAuthPda(programId, pool);
    assert.deepStrictEqual(
      withdrawAuth,
      "EMjuABxELpYWYEwjkKmQKBNCwdaFAy4QYAs6W9bDQDNw"
    );
  });
});
