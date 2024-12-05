import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { adminMock, decodeMintAccountData, mint, program } from "./utils";

describe("init_token", () => {
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const [adminPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ADMIN")],
    program.programId
  );

  const [tokenomicsPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("tokenomics")],
    program.programId
  );

  const [metadata] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  describe("happy path", () => {
    it("create token mint", async () => {
      await program.methods
        .init({
          ...adminMock,
          totalSupply: new anchor.BN(1000000000),
          decimals: 2,
          uri: "ey",
          name: "SERVER",
          symbol: "SERVER",
          rewardsPercentage: 50,
          teamPercentage: 15,
          tokensPerCommit: new anchor.BN(1000),
        })
        .accounts({ metadata })
        .rpc();

      const mintInfo = await provider.connection.getAccountInfo(mint);
      const mintData = decodeMintAccountData(mintInfo.data);

      expect(mintData.mintAuthority).eq(mint.toString());
      expect(mintData.decimals).eq(2);
      expect(mintData.isInitialized).eq(true);
      expect(mintData.supply).eq("0");

      expect(mintInfo.owner.toString()).eq(TOKEN_PROGRAM_ID.toString());

      const admin = await program.account.admin.fetch(adminPda);
      expect(admin.signer.toString()).eq(adminMock.signer.toString());
      expect(admin.teamWallet.toString()).eq(adminMock.teamWallet.toString());
      expect(admin.be).deep.eq(adminMock.be);

      const tokenomics = await program.account.tokenomics.fetch(tokenomicsPda);
    });
  });
});
