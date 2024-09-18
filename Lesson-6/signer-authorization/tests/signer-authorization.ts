import * as anchor from "@coral-xyz/anchor";
import { web3 } from "@coral-xyz/anchor";

import { Program } from "@coral-xyz/anchor";
import { SignerAuthorization } from "../../target/types/signer_authorization";
import { assert } from "chai";

describe("signer-authorization", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .SignerAuthorization as Program<SignerAuthorization>;

  const creator = web3.Keypair.generate();
  const hacker = web3.Keypair.generate();

  before("Fund the users!", async () => {
    await airdrop(provider.connection, creator.publicKey);
    await airdrop(provider.connection, hacker.publicKey);
  });

  const initialData: Number = 9;
  const newData: Number = 3;

  // x x x x x x x x x x x x x x x x x x x x x
  // | | | | | | | | | | | | | | | | | | | | |
  //           ADD YOUR CODE BELOW
  // | | | | | | | | | | | | | | | | | | | | |
  // v v v v v v v v v v v v v v v v v v v v v
  it("Initialize Escrow", async () => {
    const escrowAddress = getEscrowAddress(program.programId);

    await program.methods
      .initialize(initialData)
      .accounts({
        authority: creator.publicKey,
        escrow: escrowAddress,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([creator])
      .rpc({ commitment: "confirmed" });
  });

  it("Secure Authorization - Invalid Signer", async () => {
    const escrowAddress = getEscrowAddress(program.programId);

    try {
      await program.methods
        .secureAuthorization(newData)
        .accounts({
          authority: hacker.publicKey,
          escrow: escrowAddress,
        })
        .signers([hacker]) // Invalid signer!!!
        .rpc({ commitment: "confirmed" });
    } catch {
      const escrowAccount = await program.account.escrow.fetch(escrowAddress);
      assert.strictEqual(escrowAccount.data, initialData);
    }
  });

  it("Insecure Authorization", async () => {
    const escrowAddress = getEscrowAddress(program.programId);

    await program.methods
      .insecureAuthorization(newData)
      .accounts({
        authority: hacker.publicKey,
        escrow: escrowAddress,
      })
      .signers([hacker]) // Invalid signer!!!
      .rpc({ commitment: "confirmed" });

    const escrowAccount = await program.account.escrow.fetch(escrowAddress);
    assert.strictEqual(escrowAccount.data, newData);
  });

  it("Secure Authorization - Valid Signer", async () => {
    const escrowAddress = getEscrowAddress(program.programId);

    await program.methods
      .secureAuthorization(newData)
      .accounts({
        authority: creator.publicKey,
        escrow: escrowAddress,
      })
      .signers([creator])
      .rpc({ commitment: "confirmed" });

    const escrowAccount = await program.account.escrow.fetch(escrowAddress);
    assert.strictEqual(escrowAccount.data, newData);
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

function getEscrowAddress(programId: web3.PublicKey): web3.PublicKey {
  return web3.PublicKey.findProgramAddressSync(
    [Buffer.from("escrow")],
    programId
  )[0];
}
