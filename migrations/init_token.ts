// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.
require("dotenv").config();
import { AnchorProvider } from "@coral-xyz/anchor";

import * as anchor from "@coral-xyz/anchor";
import idl from "../target/idl/smart_contract.json";

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
  const keypair = anchor.web3.Keypair.fromSecretKey(
    Buffer.from([
      163, 195, 1, 85, 200, 56, 22, 141, 5, 64, 134, 41, 51, 112, 8, 2, 157,
      193, 154, 122, 16, 111, 250, 87, 35, 113, 154, 229, 82, 99, 241, 29, 243,
      184, 171, 13, 177, 95, 97, 37, 43, 189, 79, 104, 142, 242, 194, 174, 23,
      246, 53, 176, 66, 253, 33, 144, 64, 169, 84, 217, 52, 137, 145, 162,
    ])
  );
  const wallet = new anchor.Wallet(keypair);
  const provider = new AnchorProvider(
    new anchor.web3.Connection("http://127.0.0.1:8899"),
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
    .initToken()
    .accounts({ metadata })
    .signers([wallet.payer])
    .rpc()
    .catch((e) => console.log(e));

  showLogs({ program, tx });
};

(async () => {
  await init();
})();
