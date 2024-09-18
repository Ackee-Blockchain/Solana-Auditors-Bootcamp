import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { InitializationFrontrunning } from "../target/types/initialization_frontrunning";

const bpfProgram = new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")

describe("initialization-frontrunning", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.InitializationFrontrunning as Program<InitializationFrontrunning>;

  const signer = anchor.web3.Keypair.generate()


  before("Prepare", async () => {
    await airdrop(provider.connection, signer.publicKey);

  });


  // x x x x x x x x x x x x x x x x x x x x x
  // | | | | | | | | | | | | | | | | | | | | |
  //           ADD YOUR CODE BELOW
  // | | | | | | | | | | | | | | | | | | | | |
  // v v v v v v v v v v v v v v v v v v v v v

  it.skip("Initialize Insecure", async () => {
    const globalConfig = get_config_addresses(program.programId);

    await program.methods.initializeInsecure(15).accounts({
      signer: signer.publicKey,
      globalConfig: globalConfig,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([signer]).rpc({ commitment: "confirmed" });
  });

  it("Initialize Secure - wrong Authority", async () => {
    const globalConfig = get_config_addresses(program.programId);
    const programData = get_program_data_addresses(program.programId);

    console.log("This Test case is meant to fail !!");

    await program.methods.initializeSecure(15).accounts({
      signer: signer.publicKey,
      globalConfig: globalConfig,
      programData: programData,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([signer]).rpc({ commitment: "confirmed" });


  });

  it("Initialize Secure - correct Authority", async () => {
    const globalConfig = get_config_addresses(program.programId);
    const programData = get_program_data_addresses(program.programId);


    await program.methods.initializeSecure(15).accounts({
      signer: provider.wallet.publicKey,
      globalConfig: globalConfig,
      programData: programData,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([]).rpc({ commitment: "confirmed" });
  });

  // ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^
  // | | | | | | | | | | | | | | | | | | | | |
  //           ADD YOUR CODE ABOVE
  // | | | | | | | | | | | | | | | | | | | | |
  // x x x x x x x x x x x x x x x x x x x x x

});


async function airdrop(
  connection: any,
  address: any,
  amount = 500_000_000_000
) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    'confirmed'
  );
}

function get_config_addresses(
  program_id: anchor.web3.PublicKey,
): anchor.web3.PublicKey {


  const [config, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('config'),
    ],
    program_id
  );
  return config;
}

function get_program_data_addresses(
  program_id: anchor.web3.PublicKey,
): anchor.web3.PublicKey {


  const [program_data, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      program_id.toBuffer(),
    ],
    bpfProgram
  );
  return program_data;
}
