import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {Keypair,
        PublicKey,
        SystemProgram,
        LAMPORTS_PER_SOL} from "@solana/web3.js";
import {
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAccount,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { assert } from "chai";

import BN from "bn.js";
import type { Bounty } from "../target/types/bounty";
type BountyProgram = Program<Bounty>;
type BountyType = Bounty;
// Helpers functions for tests //

const BOUNTY_SEED = Buffer.from("bounty");
const VAULT_SEED = Buffer.from("vault");

function getBountyPDA(
  creator: PublicKey,
  bountyId: BN | number,
  programId: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      BOUNTY_SEED,
      creator.toBuffer(),
      new BN(bountyId).toArrayLike(Buffer, "le", 8)
    ],
    programId
  );
}

function getVaultPDA(
  bountyPDA: PublicKey,
  prograMId: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [VAULT_SEED, bountyPDA.toBuffer()],
    prograMId
  );
}

async function airdrop(
  connection: anchor.web3.Connection,
  pubkey: PublicKey,
  sol = 10
) {
  const sig = await connection.requestAirdrop(pubkey, sol * LAMPORTS_PER_SOL);
  await connection.confirmTransaction(sig, "confirmed");
}

async function getTokenBalance(
  connection: anchor.web3.Connection,
  tokenAccount: PublicKey
): Promise<bigint> {
  const account = await getAccount(connection, tokenAccount);
  return account.amount;
}




describe("bounty", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Bounty as Program<Bounty>; 
  const conn     = provider.connection;

  // Wallets
  let creator:  Keypair;
  let claimant: Keypair;
  let funder:   Keypair;

  // Token infrastructure
  let mint:                 PublicKey;
  let creatorTokenAccount:  PublicKey;
  let claimantTokenAccount: PublicKey;
  let funderTokenAccount:   PublicKey;

  // PDAs (filled in per test)
  let bountyPda:  PublicKey;
  let vaultPda:   PublicKey;

  // Shared bounty ID
  const bountyId = new BN(1);
  const MINT_DECIMALS = 6;
  const INITIAL_MINT  = 1_000_000 * 10 ** MINT_DECIMALS; // 1M tokens

  // ── Before all: wallets + SPL token setup ──────────────────────────────────
  before(async () => {
    creator  = Keypair.generate();
    claimant = Keypair.generate();
    funder   = Keypair.generate();

    await Promise.all([
      airdrop(conn, creator.publicKey),
      airdrop(conn, claimant.publicKey),
      airdrop(conn, funder.publicKey),
    ]);

    // Create SPL token mint (authority = creator)
    mint = await createMint(
      conn,
      creator,                // payer
      creator.publicKey,      // mint authority
      null,                   // freeze authority
      MINT_DECIMALS
    );

    // Create associated token accounts
    creatorTokenAccount = await createAssociatedTokenAccount(
      conn, creator, mint, creator.publicKey
    );
    claimantTokenAccount = await createAssociatedTokenAccount(
      conn, claimant, mint, claimant.publicKey
    );
    funderTokenAccount = await createAssociatedTokenAccount(
      conn, funder, mint, funder.publicKey
    );

    // Mint tokens to creator and funder
    await mintTo(conn, creator, mint, creatorTokenAccount,  creator, INITIAL_MINT);
    await mintTo(conn, creator, mint, funderTokenAccount,   creator, INITIAL_MINT);
  });

  // ── Before each: derive fresh PDAs ─────────────────────────────────────────
  beforeEach(async () => {
    [bountyPda] = getBountyPDA(creator.publicKey, bountyId, program.programId);
    [vaultPda]  = getVaultPDA(bountyPda, program.programId);
  });

  // ── CREATE BOUNTY ───────────────────────────────────────────────────────────────
  describe("create_bounty", () => {
    it("creates a bounty with the correct parameters", async () => {
      const reward = new BN(500 * 10 ** MINT_DECIMALS);

      await program.methods
        .createBounty(bountyId, reward)
        .accounts({
          bounty: bountyPda,
          vault: vaultPda,
          mint: mint,
          creator:       creator.publicKey,
          claimant:      claimant.publicKey,
          tokenProgram:  TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([creator])
        .rpc();

      const bountyAccount = await program.account.bounty.fetch(bountyPda);

      assert.ok(bountyAccount.creator.equals(creator.publicKey),   "creator mismatch");
      assert.ok(bountyAccount.claimant.equals(claimant.publicKey), "claimant mismatch");
      assert.ok(bountyAccount.mint.equals(mint),                   "mint mismatch");
      assert.ok(bountyAccount.vault.equals(vaultPda),              "vault mismatch");
      assert.equal(bountyAccount.bountyId.toString(), bountyId.toString());
      assert.equal(bountyAccount.reward.toString(),  reward.toString());
      assert.deepEqual(bountyAccount.status, { open: {} }, "status should be Open");
    })
  });

  // ── FUND BOUNTY ───────────────────────────────────────────────────────────────
  describe("fund_bounty", () => {
    const fundAmount = new BN(200 * 10 **MINT_DECIMALS);

    it("transfer token from the funder into the vault", async () => {
      const vaultBefore = await getTokenBalance(conn, vaultPda);
      const funderBefore = await getTokenBalance(conn, funderTokenAccount);

      await program.methods
        .fundBounty(fundAmount)
        .accounts({
          funder: funder.publicKey,
          funderTokenAccount,
          bounty: bountyPda,
          vault:  vaultPda,
          mint,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([funder])
        .rpc();

      const vaultAfter  = await getTokenBalance(conn, vaultPda);
      const funderAfter = await getTokenBalance(conn, funderTokenAccount);

      assert.equal(
        (vaultAfter - vaultBefore).toString(),
        fundAmount.toString(),
        "Vault should have received fund amount"
      );
      assert.equal(
        (funderBefore - funderAfter).toString(),
        fundAmount.toString(),
        "Funder balance should have decreased"
      );

    })

    it("allows multiple funders to top up the vault", async () => {
      const extraFund = new BN(50 * 10 ** MINT_DECIMALS);

      // Fund from creator's token account as a second funder
      await program.methods
        .fundBounty(extraFund)
        .accounts({
          funder:             creator.publicKey,
          funderTokenAccount: creatorTokenAccount,
          bounty:             bountyPda,
          vault:              vaultPda,
          mint,
          tokenProgram:       TOKEN_PROGRAM_ID,
        })
        .signers([creator])
        .rpc();

      const vaultBalance = await getTokenBalance(conn, vaultPda);
      // 200 + 50 = 250
      assert.equal(
        vaultBalance.toString(),
        new BN(250 * 10 ** MINT_DECIMALS).toString()
      );
    });

  })

});
