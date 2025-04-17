import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Contracts } from "../target/types/contracts";
import { expect } from "chai";

const DAO_ACCOUNT_SEED = "dao_account";
const MOCK_MERKLE_TREE = [168, 197, 87, 239, 85, 72, 32, 109, 81, 198, 142, 207, 69, 43, 19, 206, 72, 158, 188, 165, 21, 255, 113, 161, 169, 68, 252, 168, 215, 161, 71, 160];

describe("contracts", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Contracts as Program<Contracts>;

  const DaoKeypair = anchor.web3.Keypair.generate();
  const DaoName = "Limechain";
  
  it("Dao is initialized!", async () => {
    await airdrop(program.provider.connection, DaoKeypair.publicKey);

    const [dao_publickey, dao_bump] = getDaoAddress(DaoKeypair.publicKey, program.programId);

    await program.methods.daoInitialize(DaoName).accounts({
      signer: DaoKeypair.publicKey,
      daoAccount: dao_publickey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers([
      DaoKeypair
    ])
    .rpc();
    

    let DaoAccountData = await program.account.daoAccount.fetch(dao_publickey);

    expect(DaoAccountData.name).to.eql(DaoName);
    expect(DaoAccountData.owner).to.eql(DaoKeypair.publicKey);
    expect(DaoAccountData.merkleRoot).to.eql(MOCK_MERKLE_TREE);
  })
});


// Function to airdrop tokens to an address
async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}

function getDaoAddress(DaoCreator: anchor.web3.PublicKey, programId: anchor.web3.PublicKey) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode(DAO_ACCOUNT_SEED),
      DaoCreator.toBuffer()
    ], programId);
}