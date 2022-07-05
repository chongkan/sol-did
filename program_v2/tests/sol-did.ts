import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolDid } from "../target/types/sol_did";
import { PublicKey } from '@solana/web3.js';


import chai from 'chai';
import { expect } from 'chai';

describe("sol-did", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());


  const program = anchor.workspace.SolDid as Program<SolDid>;
  const programProvider = program.provider as anchor.AnchorProvider;


  it("Is initialized!", async () => {
    const authority = programProvider.wallet;

    const [data, dataPDABump] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("did-account"),
          authority.publicKey.toBuffer()
        ],
        program.programId
      );


    const tx = await program.methods.initialize([{
      alias: "default",
      publicKey: authority.publicKey,
      flags: 0,
      }], [])
      .accounts({
        data,
        authority: authority.publicKey
      })
      .rpc();

    // check data
    const didDataAccount = await program.account.didAccount.fetch(data)
    expect(didDataAccount.version).to.equal(0)
    expect(didDataAccount.bump).to.equal(dataPDABump)
    expect(didDataAccount.nativeControllers.length).to.equal(0)
    expect(didDataAccount.nativeVerificationMethods.length).to.equal(1)
    expect(didDataAccount.otherControllers.length).to.equal(0)
    const rawDidDataAccount = await programProvider.connection.getAccountInfo(data)
    expect(rawDidDataAccount.data.length).to.equal(10_000)


    console.log("Your transaction signature", tx);
  });


  it("Can add a Key to an account", async () => {
    // Add your test here.
    const newKey = anchor.web3.Keypair.generate();

    const tx = await program.methods.addVerificationMethod(newKey.publicKey).rpc();
    console.log("Your transaction signature", tx);
  });
});
