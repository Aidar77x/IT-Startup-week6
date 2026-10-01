import { AnchorProvider, Program, Wallet } from "@anchor-lang/core";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import fs from "fs";

async function main() {
  const RPC_URL = "https://api.devnet.solana.com";

  const PROGRAM_ID = new PublicKey(
    "EgvAE7sfipvKGE4wa238K1E5WK2HNFVJ4qcsJ4dn5KRg"
  );

  const keypairPath = "/home/roma/.config/solana/id.json";

  const secretKey = JSON.parse(
    fs.readFileSync(keypairPath, "utf8")
  );

  const keypair = Keypair.fromSecretKey(
    Uint8Array.from(secretKey)
  );

  const connection = new Connection(RPC_URL, "confirmed");
  const wallet = new Wallet(keypair);

  const idl = JSON.parse(
    fs.readFileSync("./week6_profile.json", "utf8")
  );

  const provider = new AnchorProvider(
    connection,
    wallet,
    {
      commitment: "confirmed",
    }
  );

  const program = new Program(idl, provider);

  const [profilePda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("profile"),
      keypair.publicKey.toBuffer(),
    ],
    PROGRAM_ID
  );

  console.log("Wallet:", keypair.publicKey.toBase58());
  console.log("Program:", PROGRAM_ID.toBase58());
  console.log("Profile PDA:", profilePda.toBase58());

  const tx = await program.methods
    .initialize("roma", "Solana Web3 developer")
    .accounts({
      payer: keypair.publicKey,
      profile: profilePda,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  console.log("Transaction:", tx);
  console.log("Profile initialized successfully!");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
