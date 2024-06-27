# coin-flip-unity-example
A simple coin flip game using feed protocol random number generator with Solana Unity SDK and rust solana-program package

Implementing Feed Protocol RNG to your program is very easy. You derive the needed accounts and pass into the instruction. And then in your program make a CPI to Feed Protocal RNG. 
In these simple example program we will cover every step of the implamaentation.
Lets say you want to build an on-chain coin flip game. 
First user chooses heads or tails and send this decision to your coinflip program. 
Your coin flip program calls Feed Protocol RNG. 
RNG program return a random number to your program.
You compare the returned random number with the user's decision in coinflip program.
Finally coin flip program logs a message according to result.
THIS ALL HAPPENS IN ONE TRANSACTION.
You dont need to provide any account to store random number.

# Derivation of accounts

Feed Protocol RNG Program address(It is the same address for devnet, testnet and mainnet-beta)

        private PublicKey rng_programId = new PublicKey("9uSwASSU59XvUS8d1UeU8EwrEzMGFdXZvQ4JSEAfcS7k");

Deriving a PDA that store the required feed accounts

        byte[] seed1 = Encoding.UTF8.GetBytes("c");
        byte[] seed2 = new byte[] { 1 };

       PublicKey.TryFindProgramAddress(new List<byte[]>{seed1,seed2},rng_programId, out current_feed_account, out bump);

Getting account_info from the blockchain

       var result = await  rpcClient.GetAccountInfoAsync(current_feed_account);
       var data = result.Result.Value.Data;

       byte[] encoded_data = Convert.FromBase64String(data.First());


Parsing required data from the account data

        var account1 = Deserialization.GetPubKey(encoded_data, 17);
        var account2 = Deserialization.GetPubKey(encoded_data, 49);
        var account3 = Deserialization.GetPubKey(encoded_data, 81);
        var fallback_account = Deserialization.GetPubKey(encoded_data, 113);

Generating a keypair to use in RNG program

        Account temp =  new Account();

# Creating Instruction

           TransactionInstruction ix = new TransactionInstruction
           {
                ProgramId = coin_flip_programId,
                Keys = {AccountMeta.Writable(player.PublicKey,isSigner:true),
                        AccountMeta.ReadOnly(account1,isSigner:false),
                        AccountMeta.ReadOnly(account2,isSigner:false),
                        AccountMeta.ReadOnly(account3,isSigner:false),
                        AccountMeta.ReadOnly(fallback_account,isSigner:false),
                        AccountMeta.Writable(current_feed_account,isSigner:false),
                        AccountMeta.Writable(temp.PublicKey,isSigner:true),
                        AccountMeta.ReadOnly(rng_programId,isSigner:false),
                        AccountMeta.ReadOnly(SystemProgram.ProgramIdKey,isSigner:false),
                 },
                Data= {}
           };

           
