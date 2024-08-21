import * as web3 from '@solana/web3.js';
import * as constants from './constants';

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


export class SolanaError {
    static contains(logs, error): boolean {
        const match = logs?.filter(s => s.includes(error));
        return Boolean(match?.length)
    }
}


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


export function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
