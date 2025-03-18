import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Test } from "../target/types/test";
import { ComputeBudgetProgram, Transaction } from "@solana/web3.js";

describe("test", () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Test as Program<Test>;

  it("Is initialized!", async () => {
    // Create a transaction with increased compute unit limit
    const tx = new Transaction().add(
      ComputeBudgetProgram.setComputeUnitLimit({
        units: 300000, // Increase the compute unit limit
      }),
    );

    // Add the initialize instruction to the transaction
    tx.add(await program.methods.initialize().instruction());

    // Send the transaction
    const txSignature = await anchor.AnchorProvider.env().sendAndConfirm(tx);
    console.log("Your transaction signature", txSignature);
  });
});
