require("dotenv").config();
import { bs58, hex } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import * as anchor from "@coral-xyz/anchor";
import idl from "../target/idl/server.json";

const showLogs = async ({ program, tx }) => {
  const connection = program.provider.connection;

  const latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction(
    {
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    },
    "confirmed"
  );

  const txDetails = await program.provider.connection.getTransaction(tx, {
    maxSupportedTransactionVersion: 0,
    commitment: "confirmed",
  });

  const logs = txDetails?.meta?.logMessages || null;
  console.log(logs);
};

const init = async () => {
  // // Configure client to use the provider.

  const be: number[] = [...hex.decode(process.env.BE_PUB)];
  const keypair = anchor.web3.Keypair.fromSecretKey(
    bs58.decode(process.env.ADMIN_PRIV_KEY)
  );
  const wallet = new anchor.Wallet(keypair);
  const provider = new anchor.AnchorProvider(
    new anchor.web3.Connection("https://api.devnet.solana.com"),
    wallet
  );
  const program = new anchor.Program(idl as anchor.Idl, provider);
  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("token")],
    program.programId
  );
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const [metadata] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  const tx = await program.methods
    .init({
      signer: keypair.publicKey,
      be,
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
    .signers([wallet.payer])
    .rpc()
    .catch((e) => console.log(e));

  showLogs({ program, tx });
};

(async () => {
  await init();
})();
