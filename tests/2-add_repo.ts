import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import * as borsh from "borsh";
import {
  admin,
  generateHashBuffer,
  program,
  repo,
  repoPda,
  signCoupon,
} from "./utils";

describe("add_repo", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const schema = {
    struct: { owner: "string", name: "string", branch: "string" },
  };
  const serializedData = borsh.serialize(schema, repo);
  it("create repo", async () => {
    const hash: string = generateHashBuffer(serializedData);

    const { signature, recoveryId } = await signCoupon(hash, admin);
    await program.methods
      .addRepo({
        coupon: {
          signature,
          recoveryId,
        },
        timestamp: new anchor.BN(Date.now()),
        repo,
      })
      .accounts({ publisher: provider.publicKey })
      .rpc();

    const account = await program.account.repo.fetch(repoPda);

    expect(account.publisher.toString()).equals(provider.publicKey.toString());
    expect(account.votes.toNumber()).equals(0);
    expect(account.approvedTimestamp.toNumber()).equals(0);
    expect(account.owner).equals(repo.owner);
    expect(account.name).equals(repo.name);
    expect(account.branch).equals(repo.branch);
  });
});
