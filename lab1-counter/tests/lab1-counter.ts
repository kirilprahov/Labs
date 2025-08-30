import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { lab1Counter } from "../target/types/lab1_counter";

describe("lab1-counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.lab1Counter as Program<lab1Counter>;

  const counter = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
        .initialize()
        .accounts({ counter: counter.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([counter])
        .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(0);
  });

  it("Incremented the count", async () => {
    const tx = await program.methods
        .increment()
        .accounts({ counter: counter.publicKey, user: provider.wallet.publicKey })
        .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(1);
  });
  it("Decrement the count", async () => {
    const tx = await program.methods
        .decrement()
        .accounts({ counter: counter.publicKey, user: provider.wallet.publicKey })
        .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(0);
  });
});

