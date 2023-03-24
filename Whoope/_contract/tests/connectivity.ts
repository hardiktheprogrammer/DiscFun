import { web3, Program, AnchorProvider, Provider } from '@project-serum/anchor';
import {
    TOKEN_PROGRAM_ID,
    MINT_SIZE,
    createInitializeMintInstruction,
    getAssociatedTokenAddressSync,
    createAssociatedTokenAccountInstruction,
} from '@solana/spl-token';
import { utf8 } from '@project-serum/anchor/dist/cjs/utils/bytes';
import { IDL, Whoopee } from '../_idl/whoopee'
import { SystemTypesCoder } from '@project-serum/anchor/dist/cjs/coder/system/types';

const Seeds = {
    MAIN: utf8.encode("main"),
    USER_PROFILE: utf8.encode("userProfile"),
    USER_SERVER_PROFILE: utf8.encode("userServerProfile"),
    SERVER_PROFILE: utf8.encode("serverProfile"),
    SERVER_SECTION: utf8.encode("serverSection"),
}

const Constants = {
    MAX_NAME_SIZE: 32,
    MAX_PROFILE_SUMMARY_SIZE: 256,
    MAX_SERVER_SUMMARY_SIZE: 256,
    MAX_SERVER_SECTION_NAME_SIZE: 64,
    MAX_SERVER_SECTION_DETAILS_SIZE: 256,
    MAX_NFT_NAME_SIZE: 32,
    MAX_NFT_SYMBOL_SIZE: 32,
    MAX_NFT_URI_SIZE: 128,
}

const SYSTEM_PROGRAM_ID = web3.SystemProgram.programId;
const MPL_PROGRAM_ID = new web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
const log = console.log;

interface ICreateServerInstructions {
    name: string,
    symbol: string,
    uri: string,
    summary: string,
}

interface IUpdateServerInstructions {
    name: string,
    symbol: string,
    uri: string,
    summary: string,
    serverToken: web3.PublicKey,
}

interface ICreateServerSectionInstructiuons {
    serverToken: web3.PublicKey,
    sectionName: string,
    sectionDetails: string,
}

class Connectivity {
    userId: web3.PublicKey | null;
    // wallet: WalletContextState | null;
    wallet: AnchorProvider | null;
    static programId: web3.PublicKey;
    programId: web3.PublicKey;
    program: Program<Whoopee>;
    connection: web3.Connection;
    main_account: web3.PublicKey;
    txis: web3.TransactionInstruction[] = [];

    // constructor(wallet: WalletContextState) { //NOTE: for fontend side
    constructor(provider: AnchorProvider) {
        // this.programId = new web3.PublicKey("") //NOTE: we can also take `programId` as argument
        this.wallet = provider;
        this.programId = Connectivity.programId;
        this.program = new Program(IDL, this.programId, provider);
        this.main_account = web3.PublicKey.findProgramAddressSync([Seeds.MAIN], this.programId)[0]

        //TODO: this can be changed for fontend connectivity codding.
        //TODO: need to find way to make connection by WalletContextState details 
        this.connection = new web3.Connection("https://api.devnet.solana.com", { commitment: 'finalized' })
    }

    static __getUserProfile(userId: web3.PublicKey): web3.PublicKey {
        return web3.PublicKey.findProgramAddressSync([Seeds.USER_PROFILE, userId.toBuffer()], Connectivity.programId)[0]
    }

    static __getServerProfile(serverTokenId: web3.PublicKey): web3.PublicKey {
        return web3.PublicKey.findProgramAddressSync([Seeds.SERVER_PROFILE, serverTokenId.toBuffer()], Connectivity.programId)[0]
    }

    static __getUserSeverProfile(userId: web3.PublicKey, serverTokenId: web3.PublicKey): web3.PublicKey {
        return web3.PublicKey.findProgramAddressSync(
            [
                Seeds.USER_SERVER_PROFILE,
                userId.toBuffer(),
                serverTokenId.toBuffer()
            ],
            Connectivity.programId)[0]
    }

    static __getMetadataAccount(tokenId: web3.PublicKey): web3.PublicKey {
        return web3.PublicKey.findProgramAddressSync(
            [
                utf8.encode("metadata"),
                MPL_PROGRAM_ID.toBuffer(),
                tokenId.toBuffer()
            ],
            MPL_PROGRAM_ID)[0]
    }

    static __getMasterEditionAccount(tokenId: web3.PublicKey): web3.PublicKey {
        return web3.PublicKey.findProgramAddressSync(
            [
                utf8.encode("metadata"),
                MPL_PROGRAM_ID.toBuffer(),
                tokenId.toBuffer(),
                utf8.encode("edition"),
            ],
            MPL_PROGRAM_ID)[0]
    }

    static __convertArrayFromString(txt: string, size: number): number[] {
        const buffer = utf8.encode(txt)
        let res: number[] = [...buffer.slice(0, buffer.length > size ? size : buffer.length)]
        for (let i = res.length; i < size; ++i) {
            res.push(0)
        }
        return res;
    }


    async _getOrCreateAta(token: web3.PublicKey, owner: web3.PublicKey, isOwnerOffCurve: boolean = false): Promise<web3.PublicKey> {
        const ata = getAssociatedTokenAddressSync(token, owner, isOwnerOffCurve)
        let info = await this.connection.getAccountInfo(ata);
        if (info == null) { //Means account is't initialised
            const ix = createAssociatedTokenAccountInstruction(
                this.userId,
                ata,
                owner,
                token
            )
            this.txis.push(ix)
        }
        return ata;
    }

    async _createToken(token: web3.Keypair, owner: web3.PublicKey): Promise<web3.PublicKey> {
        const rent = await this.connection.getMinimumBalanceForRentExemption(MINT_SIZE);
        const ix1 = web3.SystemProgram.createAccount(
            {
                newAccountPubkey: token.publicKey,
                fromPubkey: TOKEN_PROGRAM_ID,
                programId: this.programId,
                space: MINT_SIZE,
                lamports: rent,
            }
        );
        this.txis.push(ix1);
        const ix2 = createInitializeMintInstruction(token.publicKey, 0, owner, owner);
        this.txis.push(ix2);
        return token.publicKey;
    }

    async _gerOrCreateUserServerProfileAccount(serverToken: web3.PublicKey, serverAccount: web3.PublicKey): Promise<web3.PublicKey> {
        let uspAccount = Connectivity.__getUserSeverProfile(this.userId, serverToken);
        let info = await this.connection.getAccountInfo(uspAccount);

        if (info == null) {
            const ix = await this.program.methods.initUserServerProfile().accounts({
                user: this.userId,
                serverAccount: serverAccount,
                userServerProfileAccount: uspAccount,
                serverToken: serverToken,
                systemProgram: SYSTEM_PROGRAM_ID,
            }).instruction()
            this.txis.push(ix);
        }

        return uspAccount;
    }

    async _sendTransaction(signatures: web3.Keypair[] = []): Promise<String> {
        let tx = new web3.Transaction();
        tx.add(...this.txis);
        this.txis = []

        //TODO: may need to change for fontend connectivity
        // try {
        //     const sign = this.wallet.sendAndConfirm(tx, [...signatures]);
        //     return sign;
        // } catch (e) {
        //     return null;
        // }
        // finally {
        //     // this.txis = []
        // }

        const sign = await this.wallet.sendAndConfirm(tx, [...signatures])
        log("Trasaction Signatures: ", sign);

        return sign;
    }

    async createServer(instruction: ICreateServerInstructions) {
        const tokenKeypair = web3.Keypair.generate();
        const serverAccount = Connectivity.__getServerProfile(tokenKeypair.publicKey);
        const serverToken = await this._createToken(tokenKeypair, serverAccount)
        const userTokenAccount = await this._getOrCreateAta(serverToken, this.userId);
        const metadataAccount = Connectivity.__getMetadataAccount(serverToken);

        const name = Connectivity.__convertArrayFromString(instruction.name, Constants.MAX_NFT_NAME_SIZE);
        const symbol = Connectivity.__convertArrayFromString(instruction.symbol, Constants.MAX_NFT_SYMBOL_SIZE);
        const uri = Connectivity.__convertArrayFromString(instruction.uri, Constants.MAX_NFT_URI_SIZE);
        const summary = Connectivity.__convertArrayFromString(instruction.summary, Constants.MAX_SERVER_SUMMARY_SIZE);

        const ix = await this.program.methods.createServer(
            name, symbol, uri, summary,
        ).accounts({
            admin: this.userId,
            adminTokenAccount: userTokenAccount,
            serverToken: serverToken,
            serverAccount: serverAccount,
            mainAccount: this.main_account,
            metadataAccount: metadataAccount,
            mplProgram: MPL_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SYSTEM_PROGRAM_ID,
        }).instruction()
        this.txis.push(ix);

        //TODO: 

        await this._sendTransaction();
    }

    async updateServer(instruction: IUpdateServerInstructions) {
        const serverToken = instruction.serverToken;
        const serverAccount = Connectivity.__getServerProfile(serverToken);
        const metadataAccount = Connectivity.__getMetadataAccount(serverToken);

        const name = Connectivity.__convertArrayFromString(instruction.name, Constants.MAX_NFT_NAME_SIZE);
        const symbol = Connectivity.__convertArrayFromString(instruction.symbol, Constants.MAX_NFT_SYMBOL_SIZE);
        const uri = Connectivity.__convertArrayFromString(instruction.uri, Constants.MAX_NFT_URI_SIZE);
        const summary = Connectivity.__convertArrayFromString(instruction.summary, Constants.MAX_SERVER_SUMMARY_SIZE);

        const ix = await this.program.methods.createServer(
            name, symbol, uri, summary,
        ).accounts({
            admin: this.userId,
            serverToken: serverToken,
            serverAccount: serverAccount,
            mainAccount: this.main_account,
            metadataAccount: metadataAccount,
            mplProgram: MPL_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SYSTEM_PROGRAM_ID,
        }).instruction()
        this.txis.push(ix);

        await this._sendTransaction();
    }

    async createServerSection(instructions: ICreateServerSectionInstructiuons) {
        const serverAccount = Connectivity.__getServerProfile(instructions.serverToken);

        const sectionName = Connectivity.__convertArrayFromString(instructions.sectionName, Constants.MAX_SERVER_SECTION_NAME_SIZE);
        const sectionDetails = Connectivity.__convertArrayFromString(instructions.sectionDetails, Constants.MAX_SERVER_SECTION_DETAILS_SIZE);

        const ix = await this.program.methods.createServerScection(sectionName, sectionDetails).accounts({
            admin: this.userId,
            serverAccount: serverAccount,
            serverToken: instructions.serverToken,
        }).instruction()

        this.txis.push(ix);

        await this._sendTransaction();
    }

    async joinServer(serverToken: web3.PublicKey) {
        const serverAccount = Connectivity.__getServerProfile(serverToken);
        const userServerProfileAccount = await this._gerOrCreateUserServerProfileAccount(serverToken, serverAccount);
        const userTokenAccount = await this._getOrCreateAta(serverToken, this.userId);

        const ix = await this.program.methods.joinServer().accounts({
            user: this.userId,
            userServerProfileAccount: userServerProfileAccount,
            serverAccount: serverAccount,
            serverToken: serverToken,
            tokenProgram: TOKEN_PROGRAM_ID,
            userTokenAccount: userTokenAccount
        }).instruction()
        this.txis.push(ix);

        await this._sendTransaction();
    }

    async leaveServer(serverToken: web3.PublicKey) {
        const userTokenAccount = await this._getOrCreateAta(serverToken, this.userId);
        const userServerProfileAccount = Connectivity.__getUserSeverProfile(this.userId, serverToken);
        const serverAccount = Connectivity.__getServerProfile(serverToken);

        const ix = await this.program.methods.leaveServer().accounts({
            user: this.userId,
            serverToken,
            serverAccount,
            userTokenAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
            userServerProfileAccount,
        }).instruction()

        await this._sendTransaction()
    }
}
