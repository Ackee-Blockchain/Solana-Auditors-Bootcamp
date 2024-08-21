import * as anchor from '@coral-xyz/anchor';
import { Environment } from './environment';
import { AnchorTests } from '../target/types/anchor_tests';
import * as constants from './constants';
import * as utils from './utils';
import * as web3 from '@solana/web3.js';

export async function initEnviroment(test_env: Environment) {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);
    test_env.provider = provider;
    test_env.program = anchor.workspace.AnchorTests as anchor.Program<AnchorTests>;

    await utils.airdrop(provider.connection, test_env.signer1.publicKey, constants.INITIAL_SOL_BALANCE * web3.LAMPORTS_PER_SOL);
    await utils.airdrop(provider.connection, test_env.signer2.publicKey, constants.INITIAL_SOL_BALANCE * web3.LAMPORTS_PER_SOL);

    [test_env.mint1, test_env.vault1, test_env.metadata1] = utils.get_vault_addresses(
        test_env.signer1.publicKey,
        test_env.program.programId
    );

    [test_env.mint2, test_env.vault2, test_env.metadata2] = utils.get_vault_addresses(
        test_env.signer2.publicKey,
        test_env.program.programId);

}
