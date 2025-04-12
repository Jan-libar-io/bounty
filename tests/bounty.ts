import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Bounty } from "../target/types/bounty";
import { assert, expect } from "chai";
import { BN } from "bn.js";
import { setup } from "./setup";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  Mint,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";

describe("bounty", async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Bounty as Program<Bounty>;

  const SEED = new BN(12345678);
  const REPOSITORY_URL = "https://github.com/Jan-libar-io/bounty/";
  const amount = new BN(100);

  let maker: anchor.web3.Keypair;
  let taker: anchor.web3.Keypair;
  let mint: Mint;

  let bounty_address: anchor.web3.PublicKey;

  before(async () => {
    [maker, taker, mint] = await setup(provider);

    bounty_address = PublicKey.findProgramAddressSync(
      [
        Buffer.from("bounty"),
        maker.publicKey.toBuffer(),
        SEED.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];
  });

  describe("initialize", () => {
    it("should fail if repository url is too long", async () => {
      const tooLongUrl = new Array(300).fill("a").join("");

      try {
        await program.methods
          .initializeBounty(SEED, tooLongUrl, new BN(100))
          .accountsPartial({
            maker: maker.publicKey,
            mint: mint.address,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([maker])
          .rpc();
      } catch (error) {
        expect(error.message).to.contain("Provided url is too long");
      }
    });

    it("should initialize a bounty", async () => {
      await program.methods
        .initializeBounty(SEED, REPOSITORY_URL, amount)
        .accountsPartial({
          maker: maker.publicKey,
          mint: mint.address,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([maker])
        .rpc();

      const bounty = await program.account.bounty.fetch(bounty_address);

      assert.strictEqual(bounty.amount.toString(), amount.toString());

      let spl_vault = getAssociatedTokenAddressSync(
        mint.address,
        bounty_address,
        true,
        TOKEN_PROGRAM_ID
      );

      let spl_vault_account = await provider.connection.getTokenAccountBalance(
        spl_vault
      );

      assert.strictEqual(spl_vault_account.value.amount, amount.toString());
    });
  });

  describe("close", () => {
    it("should close a bounty", async () => {
      expect(
        await program.methods
          .closeBounty()
          .accountsPartial({
            maker: maker.publicKey,
            mint: mint.address,
            bounty: bounty_address,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          })
          .signers([maker])
          .rpc()
      ).to.not.throw;
    });
  });

  describe("submit", () => {
    it("should add a submission", async () => {
      await program.methods
        .initializeBounty(SEED, REPOSITORY_URL, amount)
        .accountsPartial({
          maker: maker.publicKey,
          mint: mint.address,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([maker])
        .rpc();

      expect(
        await program.methods
          .submitSolution(1)
          .accountsPartial({
            taker: taker.publicKey,
            bounty: bounty_address,
          })
          .signers([taker])
          .rpc()
      ).to.not.throw;

      const bounty = await program.account.bounty.fetch(bounty_address);

      assert.strictEqual(bounty.pullRequestNumber, 1);
      assert.strictEqual(bounty.taker.toBase58(), taker.publicKey.toBase58());
    });
  });

  describe("reject", () => {
    it("should fail if submission was accepted", async () => {
      try {
        program.methods
          .rejectSolution(204)
          .accountsPartial({
            maker: maker.publicKey,
          })
          .signers([maker])
          .rpc();
      } catch (error) {
        expect(error.message).to.contain("Submission accepted");
      }
    });

    it("should reject a submission", async () => {
      expect(
        await program.methods
          .rejectSolution(404)
          .accountsPartial({
            maker: maker.publicKey,
            bounty: bounty_address,
          })
          .signers([maker])
          .rpc()
      ).to.not.throw;

      const bounty = await program.account.bounty.fetch(bounty_address);

      assert.strictEqual(bounty.pullRequestNumber, 0);
      assert.strictEqual(bounty.taker.toBase58(), PublicKey.default.toBase58());
    });
  });

  describe("collect", () => {
    before(async () => {
      program.methods
        .submitSolution(1)
        .accountsPartial({
          taker: taker.publicKey,
        })
        .signers([taker])
        .rpc();
    });

    it("should fail if submission was not accepted", async () => {
      try {
        program.methods
          .collectBounty(404)
          .accountsPartial({
            maker: maker.publicKey,
            taker: taker.publicKey,
            mint: mint.address,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([taker])
          .rpc();
      } catch (error) {
        expect(error.message).to.contain("Pull request not merged");
      }
    });

    it("should collect a bounty", async () => {
      expect(
        await program.methods
          .collectBounty(204)
          .accountsPartial({
            taker: taker.publicKey,
            maker: maker.publicKey,
            mint: mint.address,
            bounty: bounty_address,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          })
          .signers([taker])
          .rpc()
      ).to.not.throw;

      const taker_ata = getAssociatedTokenAddressSync(
        mint.address,
        taker.publicKey,
        false,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

      const taker_ata_balance_after_tx = (
        await provider.connection.getTokenAccountBalance(taker_ata)
      ).value.amount;

      assert.strictEqual(taker_ata_balance_after_tx, (100).toString());
    });
  });
});
