import * as anchor from "@coral-xyz/anchor";
import { web3 } from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from 'chai';

import { DuplicateMutableAccounts } from "../../target/types/duplicate_mutable_accounts";

describe("duplicate-mutable-accounts", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DuplicateMutableAccounts as Program<DuplicateMutableAccounts>;

  const creator = web3.Keypair.generate();

  const vaultAOwner = web3.Keypair.generate();
  const vaultBOwner = web3.Keypair.generate();

  const hacker = web3.Keypair.generate();

  before("Fund the users!", async () => {
    await airdrop(provider.connection, creator.publicKey);
    await airdrop(provider.connection, hacker.publicKey);
    await airdrop(provider.connection, vaultAOwner.publicKey);
    await airdrop(provider.connection, vaultBOwner.publicKey);

  });

  // x x x x x x x x x x x x x x x x x x x x x
  // | | | | | | | | | | | | | | | | | | | | |
  //           ADD YOUR CODE BELOW
  // | | | | | | | | | | | | | | | | | | | | |
  // v v v v v v v v v v v v v v v v v v v v v

  it("Initialize Fee Vault", async () => {
    const [feeVault, fBump] = deriveFeeVaultAddress(program.programId);

    await program.methods.initializeFeeVault().accounts({
      authority: creator.publicKey,
      vault: feeVault,
    }).signers([creator]).rpc({ commitment: "confirmed" })

  });
  it("Initialize Vault A and Deposit Amount", async () => {
    const [vaultA, vBump] = deriveVaultAddress(vaultAOwner.publicKey, program.programId);

    await program.methods.initializeVault().accounts({
      creator: vaultAOwner.publicKey,
      vault: vaultA,
    }).signers([vaultAOwner]).rpc({ commitment: "confirmed" });


    const depositAmount = new anchor.BN(1000)
    await program.methods.deposit(depositAmount).accounts({
      owner: vaultAOwner.publicKey,
      vault: vaultA,
    }).signers([vaultAOwner]).rpc({ commitment: "confirmed" });

  });

  it("Initialize Vault B and Deposit Amount", async () => {
    const [vaultB, vBump] = deriveVaultAddress(vaultBOwner.publicKey, program.programId);

    await program.methods.initializeVault().accounts({
      creator: vaultBOwner.publicKey,
      vault: vaultB,
    }).signers([vaultBOwner]).rpc({ commitment: "confirmed" });


    const depositAmount = new anchor.BN(587)
    await program.methods.deposit(depositAmount).accounts({
      owner: vaultBOwner.publicKey,
      vault: vaultB,
    }).signers([vaultBOwner]).rpc({ commitment: "confirmed" });

  });

  it("Atomic Transfer with Duplicate Vault", async () => {
    const [feeVault, fBump] = deriveFeeVaultAddress(program.programId);

    const [vaultA, vBump] = deriveVaultAddress(vaultAOwner.publicKey, program.programId);

    const tradeAmount = new anchor.BN(100)

    const signature = await program.methods.insecureAtomicTrade(tradeAmount).accounts({
      signerA: vaultAOwner.publicKey,
      signerB: vaultAOwner.publicKey,
      vaultA: vaultA,
      vaultB: vaultA,
      feeVault: feeVault
    }).signers([vaultAOwner]).rpc({ commitment: "confirmed" });

    const transactionDetails = await getTx(signature, provider.connection);

    console.log("Transaction Logs:", transactionDetails.meta.logMessages);

    const vailtAAmountAfter = await program.account.vault.fetch(vaultA);
    console.log("Vault A amount after TX: ", vailtAAmountAfter.amount.toString())
  });

  it("Atomic Transfer with Duplicate Vault - Secure", async () => {
    const [feeVault, fBump] = deriveFeeVaultAddress(program.programId);

    const [vaultA, vBump] = deriveVaultAddress(vaultAOwner.publicKey, program.programId);

    const tradeAmount = new anchor.BN(100)

    try {
      await program.methods.secureAtomicTrade(tradeAmount).accounts({
        signerA: vaultAOwner.publicKey,
        signerB: vaultAOwner.publicKey,
        vaultA: vaultA,
        vaultB: vaultA,
        feeVault: feeVault
      }).signers([vaultAOwner]).rpc({ commitment: "confirmed" });
      assert.isTrue(false)
    } catch (error) {
      const err = anchor.AnchorError.parse(error.logs);
      assert.strictEqual(err.error.errorCode.code, "DuplicateVaults");

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
    'confirmed'
  );
}


function deriveVaultAddress(creator: web3.PublicKey, programId: web3.PublicKey): [web3.PublicKey, number] {
  return web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("vault"),
      creator.toBuffer()
    ], programId);
}

function deriveFeeVaultAddress(programId: web3.PublicKey): [web3.PublicKey, number] {
  return web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("fee_vault"),
    ], programId);
}


async function getTx(signature: string, connection: web3.Connection): Promise<anchor.web3.VersionedTransactionResponse> {
  return connection.getTransaction(signature, { commitment: "confirmed", maxSupportedTransactionVersion: 2 });
}
