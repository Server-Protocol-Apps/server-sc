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
  adminMock,
  showLogs,
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

  const [tokenomicsPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("tokenomics")],
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
      const tokenomics = await program.account.tokenomics.fetch(tokenomicsPda);

      expect(tokenomics.currentRewardedSupply.toNumber()).eq(10000);
      expect(balance.value.uiAmount).eq(10000);
      expect(reward.totalClaimed.toNumber()).eq(10000);
    });
    it("claim more than allowed supply", async () => {
      const newUser = anchor.web3.Keypair.generate();
      const tx = await provider.connection.requestAirdrop(
        newUser.publicKey,
        100000000000
      );
      const latestBlockHash = await provider.connection.getLatestBlockhash();
      await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
      });

      const destination2 = anchor.utils.token.associatedAddress({
        mint,
        owner: newUser.publicKey,
      });

      const [rewardPda2] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("sub"), Buffer.from("124"), repoPda.toBuffer()],
        program.programId
      );

      const claim2 = {
        commits: new anchor.BN(1000000000),
        timestamp: new anchor.BN(Date.now() + 10000000),
        userId: "124",
      };
      const serialized2 = borsh.serialize(claimSchema, claim2);
      const hash2 = generateHashBuffer(serialized2);

      const serialized_id = borsh.serialize("string", claim2.userId);

      const subscribeCoupon = await signCoupon(
        generateHashBuffer(serialized_id),
        admin
      );

      await program.methods
        .subscribe({
          repo: { owner: repo.owner, name: repo.name, branch: repo.branch },
          userId: claim2.userId,
          coupon: subscribeCoupon,
          timestamp: new anchor.BN(Date.now()),
        })
        .accounts({ signer: newUser.publicKey })
        .signers([newUser])
        .rpc();

      const { signature, recoveryId } = await signCoupon(hash2, admin);
      await program.methods
        .claimRewards({
          repo,
          coupon: { signature, recoveryId },
          claim: claim2,
        })
        .accounts({ destination: destination2, signer: newUser.publicKey })
        .signers([newUser])
        .rpc()
        .catch((e) => console.error(e));

      const balance = await provider.connection.getTokenAccountBalance(
        destination2
      );
      const reward = await program.account.subscription.fetch(rewardPda2);
      const tokenomics = await program.account.tokenomics.fetch(tokenomicsPda);

      expect(tokenomics.currentRewardedSupply.toNumber()).eq(500000000);
      expect(balance.value.uiAmount).eq(499990000);
      expect(reward.totalClaimed.toNumber()).eq(499990000);
    });
  });
  describe("errors", () => {
    it("mint above max supply", async () => {
      const claim2 = {
        commits: new anchor.BN(10),
        timestamp: new anchor.BN(Date.now() + 2000000),
        userId: "123",
      };
      const serialized2 = borsh.serialize(claimSchema, claim2);
      const hash2 = generateHashBuffer(serialized2);
      try {
        const { signature, recoveryId } = await signCoupon(hash2, admin);

        await program.methods
          .claimRewards({
            repo,
            coupon: { signature, recoveryId },
            claim: claim2,
          })
          .accounts({ destination })
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;
        expect(err.error.errorMessage).eq("Max supply exceeded");
      }
    });
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
