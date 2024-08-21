import * as anchor from '@coral-xyz/anchor';
import { AnchorTests } from '../target/types/anchor_tests';

export class Environment {
    provider: anchor.AnchorProvider;
    program: anchor.Program<AnchorTests>;

    signer1: anchor.web3.Keypair;
    mint1: anchor.web3.Keypair;
    vault1: anchor.web3.PublicKey;
    metadata1: anchor.web3.PublicKey;


    signer2: anchor.web3.Keypair;
    mint2: anchor.web3.Keypair;
    vault2: anchor.web3.PublicKey;
    metadata2: anchor.web3.PublicKey;


    constructor() {
        this.signer1 = anchor.web3.Keypair.generate();
        this.signer2 = anchor.web3.Keypair.generate();

    }
}
