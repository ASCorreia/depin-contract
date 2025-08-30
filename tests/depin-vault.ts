import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DepinVault } from "../target/types/depin_vault";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

describe("depin-vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.depinVault as Program<DepinVault>;

  let mint: anchor.web3.PublicKey;
  let payerTokenAccount: anchor.web3.PublicKey;
  let vaultTokenAccount: anchor.web3.PublicKey;

  it("Creates mint and token accounts", async () => {
    // Create new mint
    mint = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      6 // Decimals
    );

    // Create payer's associated token account
    const payerTokenAcc = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mint,
      provider.wallet.publicKey
    );
    payerTokenAccount = payerTokenAcc.address;

    // Create vault's associated token account
    const vaultPublicKey = anchor.web3.Keypair.generate().publicKey;
    const vaultTokenAcc = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mint,
      vaultPublicKey
    );
    vaultTokenAccount = vaultTokenAcc.address;

    // Mint some tokens to payer's account
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mint,
      payerTokenAccount,
      provider.wallet.publicKey,
      100_000_000_000 // 100 000 token with 6 decimals
    );

    console.log("\nSetup complete:");
    console.log("Mint:", mint.toBase58());
    console.log("Payer Token Account:", payerTokenAccount.toBase58());
    console.log("Vault Token Account:", vaultTokenAccount.toBase58());
  });

  it("Transfer to vault!", async () => {
    // Add your test here.
    const tx = await program.methods.transferToVault()
    .accountsPartial({
      from: provider.wallet.publicKey,
      fromAta: payerTokenAccount,
      vaultAta: vaultTokenAccount,
      mint: mint,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();
    console.log("\nTransferred to vault");
    console.log("Your transaction signature", tx);
  });
});
