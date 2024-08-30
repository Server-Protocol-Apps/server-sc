import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { program, repo, repoPda, showLogs } from "./utils";

describe("vote_repo", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const [votePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vote"), Buffer.from("123"), repoPda.toBuffer()],
    program.programId
  );

  describe("happy path", () => {
    it("vote down", async () => {
      const a = await program.methods
        .voteRepo({
          repo: { owner: repo.owner, name: repo.name, branch: repo.branch },
          timestamp: new anchor.BN(Date.now()),
          voteType: { down: {} },
          userId: "123",
        })
        .accounts({ voter: provider.publicKey })
        .rpc()
        .catch((e) => console.log(e));

      const vote_res = await program.account.vote.fetch(votePda);
      const repo_res = await program.account.repo.fetch(repoPda);
      expect(vote_res.repoPda.toString()).equals(repoPda.toString());
      expect(repo_res.votes.toNumber()).equals(-1);
      expect(repo_res.approved).equals(false);
    });

    it("vote up", async () => {
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

      await program.methods
        .voteRepo({
          repo: { owner: repo.owner, name: repo.name, branch: repo.branch },
          timestamp: new anchor.BN(Date.now()),
          voteType: { up: {} },
          userId: "124",
        })
        .signers([newUser])
        .accounts({ voter: newUser.publicKey })
        .rpc();

      const [votePda1] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("vote"), Buffer.from("124"), repoPda.toBuffer()],
        program.programId
      );
      const vote_res = await program.account.vote.fetch(votePda1);
      const repo_res = await program.account.repo.fetch(repoPda);
      expect(vote_res.repoPda.toString()).equals(repoPda.toString());
      expect(repo_res.votes.toNumber()).equals(0);
      expect(repo_res.approved).equals(false);
    });

    it("change vote", async () => {
      await program.methods
        .voteRepo({
          repo: { owner: repo.owner, name: repo.name, branch: repo.branch },
          voteType: { up: {} },
          timestamp: new anchor.BN(Date.now()),
          userId: "123",
        })
        .accounts({ voter: provider.publicKey })
        .rpc();

      const vote_res = await program.account.vote.fetch(votePda);
      const repo_res = await program.account.repo.fetch(repoPda);
      expect(vote_res.repoPda.toString()).equals(repoPda.toString());
      expect(repo_res.votes.toNumber()).equals(2);
      expect(repo_res.approved).equals(true);
    });
  });
  describe("errors", () => {
    it("prevent voting the same twice", async () => {
      try {
        await program.methods
          .voteRepo({
            repo: { owner: repo.owner, name: repo.name, branch: repo.branch },
            voteType: { up: {} },
            timestamp: new anchor.BN(Date.now()),
            userId: "123",
          })
          .accounts({ voter: provider.publicKey })
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;
        expect(err.error.errorMessage).eq("User has already voted");
      }
    });
  });
});
