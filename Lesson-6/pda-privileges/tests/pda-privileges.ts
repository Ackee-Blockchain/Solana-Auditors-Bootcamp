import * as anchor from "@coral-xyz/anchor";
import { web3 } from "@coral-xyz/anchor";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  createMint,
} from "@solana/spl-token";
import { assert } from "chai";
import { Program } from "@coral-xyz/anchor";
import { PdaPrivileges } from "../../target/types/pda_privileges";

describe("pda-privileges", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PdaPrivileges as Program<PdaPrivileges>;

  const creator = web3.Keypair.generate();
  const hacker = web3.Keypair.generate();

  let mint;

  before("Fund the users!", async () => {
    await airdrop(provider.connection, creator.publicKey);
    await airdrop(provider.connection, hacker.publicKey);
  });

  // x x x x x x x x x x x x x x x x x x x x x
  // | | | | | | | | | | | | | | | | | | | | |
  //           ADD YOUR CODE BELOW
  // | | | | | | | | | | | | | | | | | | | | |
  // v v v v v v v v v v v v v v v v v v v v v
  it("Initialize Vaults", async () => {
    mint = await createMint(
      provider.connection,
      creator,
      creator.publicKey,
      null,
      9
    );

    const creatorMetadataAddress = getMetadataAccountAddress(
      creator.publicKey,
      program.programId
    );
    const creatorVaultAddress = getVaultAddress(creatorMetadataAddress, mint);

    await program.methods
      .initializeVault()
      .accounts({
        vaultCreator: creator.publicKey,
        vault: creatorVaultAddress,
        metadataAccount: creatorMetadataAddress,
        mint: mint,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([creator])
      .rpc({ commitment: "confirmed" });

      const hackerMetadataAddress = getMetadataAccountAddress(
        hacker.publicKey,
        program.programId
      );
      const hackerVaultAddress = getVaultAddress(hackerMetadataAddress, mint);

      await program.methods
        .initializeVault()
        .accounts({
          vaultCreator: hacker.publicKey,
          vault: hackerVaultAddress,
          metadataAccount: hackerMetadataAddress,
          mint: mint,
          systemProgram: web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([hacker])
        .rpc({ commitment: "confirmed" });
  });

  it("Insecure Withdraw", async () => {
    const creatorMetadataAddress = getMetadataAccountAddress(
      creator.publicKey,
      program.programId
    );

    const hackerMetadataAddress = getMetadataAccountAddress(
      hacker.publicKey,
      program.programId
    );

    const creatorVaultAddress = getVaultAddress(creatorMetadataAddress, mint);
    const hackerVaultAddress = getVaultAddress(hackerMetadataAddress, mint);

    await program.methods
      .insecureWithdraw()
      .accounts({
        creator: hacker.publicKey,
        vault: creatorVaultAddress,
        withdrawDestination: hackerVaultAddress,
        metadataAccount: creatorMetadataAddress,
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([hacker])
      .rpc({ commitment: "confirmed" });
  });

    it("Secure Withdraw", async () => {
      const creatorMetadataAddress = getMetadataAccountAddress(
        creator.publicKey,
        program.programId
      );

      const hackerMetadataAddress = getMetadataAccountAddress(
        hacker.publicKey,
        program.programId
      );

      const creatorVaultAddress = getVaultAddress(creatorMetadataAddress, mint);
      const hackerVaultAddress = getVaultAddress(hackerMetadataAddress, mint);

      try {
        await program.methods
          .secureWithdraw()
          .accounts({
            creator: hacker.publicKey,
            vault: creatorVaultAddress,
            withdrawDestination: hackerVaultAddress,
            metadataAccount: creatorMetadataAddress,
            mint: mint,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([hacker])
          .rpc({ commitment: "confirmed" });
      } catch (error) {
        const err = anchor.AnchorError.parse(error.logs);
        assert.strictEqual(err.error.errorCode.code, "ConstraintHasOne");
      }
    });
  // ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^
  // | | | | | | | | | | | | | | | | | | | | |
  //           ADD YOUR CODE ABOVE
  // | | | | | | | | | | | | | | | | | | | | |
  // x x x x x x x x x x x x x x x x x x x x x
});
export async function airdrop(
  connection: any,
  address: any,
  amount = 500_000_000_000
) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    "confirmed"
  );
}

function getMetadataAccountAddress(
  creator: web3.PublicKey,
  programId: web3.PublicKey
): web3.PublicKey {
  return web3.PublicKey.findProgramAddressSync(
    [Buffer.from("metadata_account"), creator.toBuffer()],
    programId
  )[0];
}

function getVaultAddress(
  creator: web3.PublicKey,
  mint: web3.PublicKey
): web3.PublicKey {
  return getAssociatedTokenAddressSync(mint, creator, true);
}
