import { Environment } from './environment';
import * as web3 from '@solana/web3.js';
import * as constants from './constants';
import * as spl_token from '@solana/spl-token';
import { Clock } from "solana-bankrun";
import * as anchor from '@coral-xyz/anchor';
import { assert } from 'chai';

export async function programTests1(test_env: Environment) {

    it('Try to Initialize with the wrong Input', async () => {
        const input = 5;

        const mintParams = {
            name: constants.NAME1,
            symbol: constants.SYMBOL1,
            uri: constants.URI1,

        }

        try {
            await test_env.program.methods.initialize(
                new anchor.BN(constants.DAYS_7),
                input,
                mintParams,
            ).accounts({
                signer: test_env.signer1.publicKey,
                vault: test_env.vault1,
                mint: test_env.mint1.publicKey,
                mintMetadata: test_env.metadata1,
                mplTokenMetadata: constants.MetaplexTokenMetadataProgram,
                tokenProgram: spl_token.TOKEN_PROGRAM_ID,
                associatedTokenProgram: spl_token.ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: web3.SystemProgram.programId
            }).signers([test_env.signer1, test_env.mint1]).rpc()
        } catch (error) { }
    });

    it('Initialize', async () => {

        const mintParams = {
            name: constants.NAME1,
            symbol: constants.SYMBOL1,
            uri: constants.URI1,
        }

        await test_env.program.methods.initialize(
            new anchor.BN(constants.DAYS_7),
            constants.INPUT1,
            mintParams,
        ).accounts({
            signer: test_env.signer1.publicKey,
            vault: test_env.vault1,
            mint: test_env.mint1.publicKey,
            mintMetadata: test_env.metadata1,
            mplTokenMetadata: constants.MetaplexTokenMetadataProgram,
            tokenProgram: spl_token.TOKEN_PROGRAM_ID,
            associatedTokenProgram: spl_token.ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: web3.SystemProgram.programId
        }).signers([test_env.signer1, test_env.mint1]).rpc();

    });

    it('Cannot Re-Initialize', async () => {

        const mintParams = {
            name: constants.NAME1,
            symbol: constants.SYMBOL1,
            uri: constants.URI1,
        }

        try {

            await test_env.program.methods.initialize(
                new anchor.BN(constants.DAYS_7),
                constants.INPUT2,
                mintParams
            ).accounts({
                signer: test_env.signer1.publicKey,
                vault: test_env.vault1,
                mint: test_env.mint1.publicKey,
                mintMetadata: test_env.metadata1,
                mplTokenMetadata: constants.MetaplexTokenMetadataProgram,
                tokenProgram: spl_token.TOKEN_PROGRAM_ID,
                associatedTokenProgram: spl_token.ASSOCIATED_TOKEN_PROGRAM_ID,
                systemProgram: web3.SystemProgram.programId
            }).signers([test_env.signer1, test_env.mint1]).rpc();


        } catch (error) { }

    });
    it('Cannot Read yet', async () => {
        try {
            await test_env.program.methods.read().accounts({
                signer: test_env.signer1.publicKey,
                vault: test_env.vault1,
            }).signers([test_env.signer1]).rpc();
        } catch (error) {

        }


    });
    it("Forward in Time", async () => {
        let clock = await test_env.context.banksClient.getClock()

        const now = clock.unixTimestamp;

        const in_future_7_days = now + BigInt(7 * 24 * 60 * 60);

        let new_clock = new Clock(clock.slot, clock.epochStartTimestamp, clock.epoch, clock.leaderScheduleEpoch, in_future_7_days);

        test_env.context.setClock(new_clock);

    });

    it('Read', async () => {

        await test_env.program.methods.read().accounts({
            signer: test_env.signer1.publicKey,
            vault: test_env.vault1,
        }).signers([test_env.signer1]).rpc();

    });
}
