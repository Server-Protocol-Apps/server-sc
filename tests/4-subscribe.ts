import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import {
  addRepo,
  admin,
  generateHashBuffer,
  program,
  repo,
  repoPda,
  signCoupon,
} from "./utils";
import * as borsh from "borsh";

describe("subscribe", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const id = "123";
  describe("happy path", () => {
    it("subscribe", async () => {
      const serialized_id = borsh.serialize("string", id);

      const { signature, recoveryId } = await signCoupon(
        generateHashBuffer(serialized_id),
        admin
      );

      const [subscribePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("sub"), Buffer.from(id), repoPda.toBuffer()],
        program.programId
      );

      await program.methods
        .subscribe({
          repo: { owner: repo.owner, name: repo.name, branch: repo.branch },
          userId: id,
          coupon: { signature, recoveryId },
          timestamp: new anchor.BN(Date.now()),
        })
        .accounts({ signer: provider.publicKey })
        .rpc();

      const sub_res = await program.account.subscription.fetch(subscribePda);
      expect(sub_res.repoPda.toString()).equals(repoPda.toString());
      expect(sub_res.userId).equals(id);
    });
  });
  describe("errors", () => {
    it("subscribe an unapproved repo", async () => {
      const newRepo = {
        owner: "a",
        name: "b",
        branch: "z",
      };
      await addRepo(provider, newRepo);
      try {
        const serialized_id = borsh.serialize("string", id);

        const { signature, recoveryId } = await signCoupon(
          generateHashBuffer(serialized_id),
          admin
        );

        await program.methods
          .subscribe({
            repo: newRepo,
            coupon: { signature, recoveryId },
            userId: id,
            timestamp: new anchor.BN(Date.now()),
          })
          .accounts({ signer: provider.publicKey })
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;
        expect(err.error.errorMessage).eq("Repo needs to be approved");
      }
    });
  });
});
