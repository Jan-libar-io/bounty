import * as anchor from "@coral-xyz/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccount,
  createMint,
  getMint,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

const sendSol = async (
  provider: anchor.Provider,
  keypairs: anchor.web3.Keypair[]
) => {
  for (const keypair of keypairs) {
    let token_airdrop = await provider.connection.requestAirdrop(
      keypair.publicKey,
      10000 * LAMPORTS_PER_SOL
    );

    const latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: token_airdrop,
    });

    console.log(`Airdropped 10'000 SOL to ${keypair.publicKey.toBase58()}`);
  }
};

export const setup = async (provider: anchor.Provider) => {
  const [maker, taker] = Array.from({ length: 8 }, () =>
    anchor.web3.Keypair.generate()
  );

  await sendSol(provider, [maker, taker]);

  const mint_pubkey = await createMint(
    provider.connection,
    maker,
    maker.publicKey,
    null,
    9,
    undefined,
    { commitment: "confirmed" },
    TOKEN_PROGRAM_ID
  );

  const mint = await getMint(provider.connection, mint_pubkey);

  const maker_ata = await createAssociatedTokenAccount(
    provider.connection,
    maker,
    mint.address,
    maker.publicKey,
    {
      commitment: "confirmed",
    },
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    false
  );

  await mintTo(
    provider.connection,
    maker,
    mint.address,
    maker_ata,
    maker,
    1 * Math.pow(10, 9),
    undefined,
    {
      commitment: "confirmed",
    },
    TOKEN_PROGRAM_ID
  );

  const acc = await provider.connection.getTokenAccountBalance(maker_ata);

  console.log(
    "Funded " + acc.value.amount + " tokens to " + maker_ata.toBase58()
  );

  return [maker, taker, mint, maker_ata] as const;
};
