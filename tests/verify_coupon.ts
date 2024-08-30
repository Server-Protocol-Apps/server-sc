import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Server } from "../target/types/server";
import { Wallet, ethers } from "ethers";
import * as borsh from "borsh";
import { expect } from "chai";
import { admin, generateHashBuffer, showLogs, signCoupon } from "./utils";

describe("verify_coupon", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Server as Program<Server>;
  const schema = { struct: { a: "u8", b: "string" } };

  describe("happy path", () => {
    it("signed by admin", async () => {
      const serializedData = borsh.serialize(schema, { a: 1, b: "ooo" });
      const hash = generateHashBuffer(serializedData);

      const { signature, recoveryId } = await signCoupon(hash, admin);
      await program.methods
        .verifyCoupon({
          coupon: {
            signature: signature,
            recoveryId,
          },
          data: {
            a: 1,
            b: "ooo",
          },
        })
        .accounts({})
        .rpc();

      expect(true);
    });
    it("signed by different payload", async () => {
      const serializedData = borsh.serialize(schema, { a: 5, b: "6" });
      const hash = generateHashBuffer(serializedData);
      const { signature, recoveryId } = await signCoupon(hash, admin);
      const tx = await program.methods
        .verifyCoupon({
          coupon: {
            signature: signature,
            recoveryId,
          },
          data: {
            a: 5,
            b: "6",
          },
        })
        .accounts({})
        .rpc();

      expect(true);
    });
  });

  describe("errors", () => {
    it("signed by bad actor", async () => {
      const serializedData = borsh.serialize(schema, { a: 1, b: "2" });
      const hash = generateHashBuffer(serializedData);
      const signer = Wallet.createRandom();

      const { signature, recoveryId } = await signCoupon(hash, signer);
      try {
        await program.methods
          .verifyCoupon({
            coupon: {
              signature,
              recoveryId,
            },
            data: {
              a: 1,
              b: "2",
            },
          })
          .accounts({})
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;
        expect(err.error.errorMessage).eq("This coupon is invalid");
      }
    });

    it("signed by admin but bad payload", async () => {
      const serializedData = borsh.serialize(schema, { a: 1, b: "2" });
      const hash = generateHashBuffer(serializedData);
      const { signature, recoveryId } = await signCoupon(hash, admin);
      try {
        await program.methods
          .verifyCoupon({
            coupon: {
              signature,
              recoveryId,
            },
            data: {
              a: 2,
              b: "2",
            },
          })
          .accounts({})
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;
        expect(err.error.errorMessage).eq("This coupon is invalid");
      }
    });
  });
});
