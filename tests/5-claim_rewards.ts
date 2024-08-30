import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Server } from "../target/types/server";
import {
  repo,
  repoPda,
  mint,
  claimSchema,
  generateHashBuffer,
  signCoupon,
  admin,
  addRepo,
} from "./utils";
import { expect } from "chai";
import * as borsh from "borsh";

describe("claim_rewards", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Server as Program<Server>;

  const [rewardPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("sub"), Buffer.from("123"), repoPda.toBuffer()],
    program.programId
  );

  const destination = anchor.utils.token.associatedAddress({
    mint,
    owner: provider.publicKey,
  });
  const claim = {
    commits: new anchor.BN(10),
    timestamp: new anchor.BN(Date.now() + 1000000),
    userId: "123",
  };
  const serialized = borsh.serialize(claimSchema, claim);
  const hash = generateHashBuffer(serialized);

  describe("happy path", () => {
    it("claim rewards", async () => {
      const { signature, recoveryId } = await signCoupon(hash, admin);
      await program.methods
        .claimRewards({
          repo,
          coupon: { signature, recoveryId },
          claim,
        })
        .accounts({ destination })
        .rpc();

      const balance = await provider.connection.getTokenAccountBalance(
        destination
      );
      const reward = await program.account.subscription.fetch(rewardPda);

      expect(balance.value.uiAmount).eq(10);

      expect(reward.totalClaimed.toNumber()).eq(10);
      expect(true).eq(true);
    });
  });
  describe("errors", () => {
    it("claim same coupon twice", async () => {
      try {
        const { signature, recoveryId } = await signCoupon(hash, admin);

        await program.methods
          .claimRewards({
            repo,
            coupon: { signature, recoveryId },
            claim,
          })
          .accounts({ destination })
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;
        expect(err.error.errorMessage).eq("Commits already claimed");
      }
    });
    it("claim repo you are not subscribed to", async () => {
      try {
        const claim1 = {
          commits: new anchor.BN(10),
          timestamp: new anchor.BN(Date.now() + 10),
          userId: "123",
        };
        const serialized = borsh.serialize(claimSchema, claim1);
        const hash = generateHashBuffer(serialized);
        const newRepo = {
          owner: "1a",
          name: "b",
          branch: "z",
        };
        await addRepo(provider, newRepo);

        const { signature, recoveryId } = await signCoupon(hash, admin);

        await program.methods
          .claimRewards({
            repo: newRepo,
            coupon: { signature, recoveryId },
            claim: claim1,
          })
          .accounts({ destination })
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;
        expect(err.error.errorMessage).eq(
          "The program expected this account to be already initialized"
        );
      }
    });
  });
});
