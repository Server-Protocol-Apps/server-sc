import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { adminMock, mint, program } from "./utils";

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
      await program.methods.init(adminMock).accounts({ metadata }).rpc();

      const mintInfo = await provider.connection.getAccountInfo(mint);

      expect(mintInfo.owner.toString()).eq(TOKEN_PROGRAM_ID.toString());

      const admin = await program.account.admin.fetch(adminPda);
      expect(admin.signer.toString()).eq(adminMock.signer.toString());
      expect(admin.be).deep.eq(adminMock.be);
    });
  });
});
