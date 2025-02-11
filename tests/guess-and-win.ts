import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { GuessAndWin } from '../target/types/guess_and_win';

describe('guess-and-win', () => {
  const program = anchor.workspace.GuessAndWin as Program<GuessAndWin>;
  console.log(program.programId.toString());
  // Configure the client to use the local cluster.
  const wallet = anchor.Wallet.local();
  const bonus = 5000000000;
  // const bonus = 1;
  // pda
  const title = 'guess-and-win';
  const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [wallet.publicKey.toBuffer(), Buffer.from(title)],
    program.programId
  );
  anchor.setProvider(anchor.AnchorProvider.env());

  it('initialize pool!', async () => {
    // Add your test here.
    const tx = await program.methods
      .initializePool(title, new anchor.BN(bonus))
      .accounts({
        pool: pda,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc();
    console.log('Your transaction signature', tx);
  });

  it('get pool!', async () => {
    // Add your test here.
    const account = await program.account.pool.fetch(pda);
    console.log('Your pool is:', account);
  });

  it('withdraw pool!', async () => {
    // Add your test here.
    const tx = await program.methods
      .withdrawPool()
      .accounts({
        pool: pda,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc();
    console.log('Your transaction signature', tx);
  });

  it('delete pool!', async () => {
    // Add your test here.
    const tx = await program.methods
      .deletePool()
      .accounts({
        pool: pda,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc();
    console.log('Your transaction signature', tx);
  });
});
