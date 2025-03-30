require("dotenv").config();
import * as anchor from "@coral-xyz/anchor";
import { Server } from "../target/types/server";
import { HDNodeWallet, Wallet } from "ethers";
import { keccak256, toBuffer, ecsign } from "ethereumjs-utils";
import * as borsh from "borsh";
import { hex } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

export const repo = {
  owner: "JulioMh",
  name: "github-rewards",
  branch: "main",
};

export const program = anchor.workspace.Server as anchor.Program<Server>;
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

export const adminMock = {
  signer: provider.wallet.publicKey,
  be: [...hex.decode(process.env.BE_PUB)],
  teamWallet: anchor.web3.Keypair.generate().publicKey,
};

export const [repoPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("repo"),
    Buffer.from(repo.owner),
    Buffer.from(repo.name),
    Buffer.from(repo.branch),
  ],
  program.programId
);

export const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("token")],
  program.programId
);

export const signCoupon = async (
  hash: string,
  signer: Wallet | HDNodeWallet
) => {
  const sig = ecsign(toBuffer(hash), toBuffer(signer.privateKey));

  return {
    signature: Buffer.concat([sig.r, sig.s]).toString("hex"),
    recoveryId: sig.v - 27,
  };
};

export const generateHashBuffer = (values: Uint8Array) => {
  return keccak256(Buffer.from(values));
};

export const admin = new Wallet(process.env.BE_PRIV);

export const showLogs = async ({ program, tx }) => {
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

export const repoSchema = {
  struct: { owner: "string", name: "string", branch: "string" },
};

export const claimSchema = {
  struct: { commits: "u64", timestamp: "u128", userId: "string" },
};

export const addRepo = async (provider, repo) => {
  const serializedData = borsh.serialize(repoSchema, repo);
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
};

export const decodeMintAccountData = (data: Buffer) => {
  const mintAuthorityOption = data.readUInt32LE(0);
  const mintAuthority = mintAuthorityOption
    ? new anchor.web3.PublicKey(data.slice(4, 36))
    : null;
  const supply = data.readBigUInt64LE(36);
  const decimals = data.readUInt8(44);
  const isInitialized = data.readUInt8(45) !== 0;
  const freezeAuthorityOption = data.readUInt32LE(46);
  const freezeAuthority = freezeAuthorityOption
    ? new anchor.web3.PublicKey(data.slice(50, 82))
    : null;

  return {
    mintAuthority: mintAuthority ? mintAuthority.toBase58() : null,
    supply: supply.toString(),
    decimals,
    isInitialized,
    freezeAuthority: freezeAuthority ? freezeAuthority.toBase58() : null,
  };
};
