import { Environment } from "../environment";
import * as spl_token from '@solana/spl-token';
import { PublicKey } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';
import * as constants from "../constants";
import * as web3 from '@solana/web3.js';
import { assert } from 'chai';


interface Initialize {
    signer: anchor.web3.Keypair,
    vault: PublicKey,
    mint: anchor.web3.Keypair,
    metadata: PublicKey,
    open_time: number,
    input: number,
    name: string,
    symbol: string,
    uri: string,
}

export async function initialize(
    test_env: Environment,
    initialize: Initialize,
) {

    const mintParams = {
        name: initialize.name,
        symbol: initialize.symbol,
        uri: initialize.uri,

    }

    await test_env.program.methods.initialize(
        new anchor.BN(initialize.open_time),
        initialize.input,
        mintParams,
    ).accounts({
        signer: initialize.signer.publicKey,
        vault: initialize.vault,
        mint: initialize.mint.publicKey,
        mintMetadata: initialize.metadata,
        mplTokenMetadata: constants.MetaplexTokenMetadataProgram,
        tokenProgram: spl_token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: spl_token.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId
    }).signers([initialize.signer, initialize.mint]).rpc({ commitment: "confirmed" })

    let vaultData = await test_env.program.account.vault.fetch(initialize.vault);
    assert.strictEqual(vaultData.vaultContent, initialize.input);
    assert.strictEqual(vaultData.mint.toString(), initialize.mint.publicKey.toString());

}
