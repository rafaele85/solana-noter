import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { Noter } from "../target/types/noter"
import * as assert from "assert";

describe("noter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.Noter as Program<Noter>

  let note = anchor.web3.Keypair.generate()

  it("can create a note", async () => {
      const accounts = {
          note: note.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
      }

      const contentOfNewNote = 'Content of new note'
      await program.methods.createNote(contentOfNewNote).accounts(accounts).signers([note]).rpc()

      let newNote = await program.account.note.fetch(note.publicKey)

      assert.strictEqual(newNote.content, contentOfNewNote)
      assert.strictEqual(newNote.user.toBase58(), provider.wallet.publicKey.toBase58())
  });

    it("can delete a note", async () => {
        const accounts = {
            note: note.publicKey,
            user: provider.wallet.publicKey,
        }
        await program.methods.deleteNote().accounts(accounts).rpc()

        let deleteNote = await program.account.note.fetchNullable(note.publicKey)

        assert.ok(deleteNote == null)
    });
});
