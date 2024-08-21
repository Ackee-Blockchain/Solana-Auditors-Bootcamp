import * as web3 from '@solana/web3.js';
import * as constants from './constants';



export function get_vault_addresses(
    signer: web3.PublicKey,
    program_id: web3.PublicKey,
): [web3.Keypair, web3.PublicKey, web3.PublicKey] {

    const mint = web3.Keypair.generate();

    const [vault, vaultBump] = web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from('vault'),
            signer.toBuffer(),
            mint.publicKey.toBuffer(),
        ],
        program_id
    );
    const [metadata, metadata_bump] = web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from('metadata'),
            constants.MetaplexTokenMetadataProgram.toBuffer(),
            mint.publicKey.toBuffer(),
        ],
        constants.MetaplexTokenMetadataProgram
    );

    return [mint, vault, metadata];
}
