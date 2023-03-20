import { web3, Program, AnchorProvider } from '@project-serum/anchor'
import { IDL, Whoopee } from '../_idl/whoopee'

class Connectivity {
    userId: web3.PublicKey;
    program: Program<Whoopee>
}
