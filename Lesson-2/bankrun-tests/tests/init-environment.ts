import * as anchor from '@coral-xyz/anchor';
import { Environment } from './environment';
import { BankrunTests } from '../target/types/bankrun_tests';
import * as constants from './constants';
import * as utils from './utils';
import { startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";

const IDL = require("../target/idl/bankrun_tests.json");

export async function initEnviroment(test_env: Environment) {
    test_env.context = await startAnchor(
        ".",
        [
            { name: "mpl-token-metadata", programId: constants.MetaplexTokenMetadataProgram }
        ],
        []
    );
    const provider = new BankrunProvider(test_env.context);

    test_env.provider = provider;

    test_env.program = new anchor.Program<BankrunTests>(
        IDL,
        provider,
    );

    test_env.signer1 = test_env.context.payer;

    [test_env.mint1, test_env.vault1, test_env.metadata1] = utils.get_vault_addresses(test_env.signer1.publicKey, test_env.program.programId);
}
