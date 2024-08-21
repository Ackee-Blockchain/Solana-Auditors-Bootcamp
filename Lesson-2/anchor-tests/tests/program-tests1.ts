import { Environment } from './environment';
import { assert } from 'chai';
import * as anchor from '@coral-xyz/anchor';
import * as utils from "./utils"
import * as constants from './constants';
import * as initialize from './instructions/initialize';
import * as read from './instructions/read';


export async function programTests1(test_env: Environment) {

    it('Try to Initialize with the wrong Input', async () => {

        const input = 5;
        try {

            await initialize.initialize(
                test_env,
                {
                    signer: test_env.signer1,
                    vault: test_env.vault1,
                    mint: test_env.mint1,
                    metadata: test_env.metadata1,
                    open_time: constants.OPEN_TIME_7DAYS,
                    input: input,
                    name: constants.NAME1,
                    symbol: constants.SYMBOL1,
                    uri: constants.URI1
                }
            )
            assert.fail()
        } catch (error) {
            const err = anchor.AnchorError.parse(error.logs);
            assert.strictEqual(err.error.errorCode.code, 'IncorrectInput');
        }

    });

    it('Initialize', async () => {


        await initialize.initialize(
            test_env,
            {
                signer: test_env.signer1,
                vault: test_env.vault1,
                mint: test_env.mint1,
                metadata: test_env.metadata1,
                open_time: constants.OPEN_TIME_7DAYS,
                input: constants.INPUT1,
                name: constants.NAME1,
                symbol: constants.SYMBOL1,
                uri: constants.URI1
            }
        )
    });

    it('Cannot Re-Initialize', async () => {

        try {
            await initialize.initialize(
                test_env,
                {
                    signer: test_env.signer1,
                    vault: test_env.vault1,
                    mint: test_env.mint1,
                    metadata: test_env.metadata1,
                    open_time: constants.OPEN_TIME_7DAYS,
                    input: constants.INPUT1,
                    name: constants.NAME1,
                    symbol: constants.SYMBOL1,
                    uri: constants.URI1
                }
            )
            assert.fail()
        } catch (error) {
            assert.isTrue(utils.SolanaError.contains(error.logs, "already in use"), error.logs)
        }

    });
    it('Cannot Read before deadline reached', async () => {
        try {
            await read.read(
                test_env,
                {
                    signer: test_env.signer1,
                    vault: test_env.vault1,
                    expected_vault_content: constants.INPUT1
                }
            )
            assert.fail()
        } catch (error) {
            const err = anchor.AnchorError.parse(error.logs);
            assert.strictEqual(err.error.errorCode.code, 'NotOpenedYet');
        }

    });
}
