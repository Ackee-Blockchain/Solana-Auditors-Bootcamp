import * as anchor from '@coral-xyz/anchor';
import { BankrunTests } from '../target/types/bankrun_tests';
import { ProgramTestContext } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";

export class Environment {
    provider: BankrunProvider;
    context: ProgramTestContext;
    program: anchor.Program<BankrunTests>;

    signer1: anchor.web3.Keypair;
    mint1: anchor.web3.Keypair;
    vault1: anchor.web3.PublicKey;
    metadata1: anchor.web3.PublicKey;


    constructor() {
        this.signer1 = anchor.web3.Keypair.generate();

    }
}
