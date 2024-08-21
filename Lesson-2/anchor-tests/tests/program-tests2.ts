import { Environment } from './environment';
import { assert } from 'chai';
import * as anchor from '@coral-xyz/anchor';
import * as utils from "./utils"
import * as constants from './constants';
import * as initialize from './instructions/initialize';
import * as read from './instructions/read';


export async function programTests2(test_env: Environment) {

    it('Initialize', async () => {

        await initialize.initialize(
            test_env,
            {
                signer: test_env.signer2,
                vault: test_env.vault2,
                mint: test_env.mint2,
                metadata: test_env.metadata2,
                open_time: constants.OPEN_TIME_5SEC,
                input: constants.INPUT2,
                name: constants.NAME2,
                symbol: constants.SYMBOL2,
                uri: constants.URI2
            }
        )

        await utils.delay(5500);
    });

    it('Read', async () => {
        await read.read(
            test_env,
            {
                signer: test_env.signer2,
                vault: test_env.vault2,
                expected_vault_content: constants.INPUT2
            }
        )

    });
}
