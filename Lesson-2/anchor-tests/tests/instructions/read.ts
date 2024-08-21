import { Environment } from "../environment";
import { PublicKey } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';
import { assert } from "chai";


interface Read {
    signer: anchor.web3.Keypair,
    vault: PublicKey,
    expected_vault_content: number,
}

export async function read(
    test_env: Environment,
    read: Read,
) {

    let evenListener = test_env.program.addEventListener('readEvent', (event) => {
        assert.strictEqual(event.vaultContent.toString(), read.expected_vault_content.toString())
        console.log("Event: ", event.vaultContent)
    });

    try {
        await test_env.program.methods.read().accounts({
            signer: read.signer.publicKey,
            vault: read.vault,
        }).signers([read.signer]).rpc({ commitment: "confirmed" })
    } catch (error) {
        throw error;
    } finally {
        test_env.program.removeEventListener(evenListener);
    }

}
