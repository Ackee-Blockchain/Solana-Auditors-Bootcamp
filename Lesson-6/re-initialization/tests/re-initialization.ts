import * as anchor from "@coral-xyz/anchor";
import { web3 } from "@coral-xyz/anchor";

import { Program } from "@coral-xyz/anchor";
import { ReInitialization } from "../../target/types/re_initialization";

describe("re-initialization", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ReInitialization as Program<ReInitialization>;

  const creator = web3.Keypair.generate();
  const hacker = web3.Keypair.generate();

  before("Fund the users!", async () => {
    await airdrop(provider.connection, creator.publicKey);
    await airdrop(provider.connection, hacker.publicKey);
  });


  // x x x x x x x x x x x x x x x x x x x x x
  // | | | | | | | | | | | | | | | | | | | | |
  //           ADD YOUR CODE BELOW
  // | | | | | | | | | | | | | | | | | | | | |
  // v v v v v v v v v v v v v v v v v v v v v
  it("Insecure Initialize - 1!", async () => {
    const [metadata, mBump] = web3.PublicKey.findProgramAddressSync([Buffer.from("metadata")], program.programId);

    const createData = {
      name: "a",
      symbol: "a",
      uri: "a",
      yearOfCreation: new anchor.BN(2)
    }
    await program.methods.insecureInitializev1(createData).accounts({
      creator: creator.publicKey,
      metadata: metadata,
      systemProgram: web3.SystemProgram.programId,
    }).signers([creator]).rpc({ commitment: "confirmed" });
  });

  it("Insecure Initialize - 2!", async () => {
    const [metadata, mBump] = web3.PublicKey.findProgramAddressSync([Buffer.from("metadata")], program.programId);

    const createData = {
      name: "a",
      symbol: "a",
      uri: "a",
      yearOfCreation: new anchor.BN(2)
    }
    await program.methods.insecureInitializev2(createData).accounts({
      creator: creator.publicKey,
      metadata: metadata,
      systemProgram: web3.SystemProgram.programId,
    }).signers([creator]).rpc({ commitment: "confirmed" });
  });

  it("Secure Initialize!", async () => {
    const [metadata, mBump] = web3.PublicKey.findProgramAddressSync([Buffer.from("metadata")], program.programId);

    const createData = {
      name: "a",
      symbol: "a",
      uri: "a",
      yearOfCreation: new anchor.BN(2)
    }
    await program.methods.secureInitialize(createData).accounts({
      creator: creator.publicKey,
      metadata: metadata,
      systemProgram: web3.SystemProgram.programId,
    }).signers([creator]).rpc({ commitment: "confirmed" });
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
    'confirmed'
  );
}
