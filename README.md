# Coin Flip Unity Example
A simple coin flip game using  FEED PROTOCOL RANDOM NUMBER GENERATOR  with Solana Unity SDK and rust solana-program package


Implementing FEED PROTOCOL RANDOM NUMBER GENERATOR PROGRAM (FPRNG) to your program is very easy. You derive the needed accounts and pass into the instruction. And then in your program make a CPI to FPRNG. 
In these simple example program we will cover every step of the implamaentation.
Lets say you want to build an on-chain coin flip game. 
First user chooses heads or tails and send this decision to your coinflip program. 
Your coin flip program calls FPRNG. 
FPRNG return a random number to your program.
You compare the returned random number with the user's decision in coinflip program.
Finally coin flip program logs a message according to result.
THIS ALL HAPPENS IN ONE TRANSACTION.
You can store the random number in an account in your program.
You can also try coinflip program on Devnet and Testnet.

Now lets take a look at how we use FPRNG in coinflip game program

# Derivation of accounts

Let players decide on your game interface

        public ulong public ulong decision;

FPRNG address(It is the same address for devnet, testnet and mainnet-beta)

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

Generating a keypair to use in FPRNG

        Account temp =  new Account();

# Creating Instruction

Player's decision(head or tails) is serialized to pass as instruction data. 

        byte[] buffer = new byte[8]; // Ensure the buffer is large enough for a u64 (8 bytes)

        Serialization.WriteU64(buffer, decision, 0);
        
We create our instruction, then build it and finally send. Below account are necassary to CPI FPRNG. 
You can also include the accounts you want to use in your program. 
However, when you make cpi into FPRNG the order of these accounts and their properties should be as below

        var ix = new TransactionInstruction
           {
                ProgramId = coin_flip_programId,
                Keys = new List<AccountMeta>{AccountMeta.Writable(player.PublicKey,isSigner:true),
                        AccountMeta.ReadOnly(account1,isSigner:false),
                        AccountMeta.ReadOnly(account2,isSigner:false),
                        AccountMeta.ReadOnly(account3,isSigner:false),
                        AccountMeta.ReadOnly(fallback_account,isSigner:false),
                        AccountMeta.Writable(current_feed_account,isSigner:false),
                        AccountMeta.Writable(temp.PublicKey,isSigner:true),
                        AccountMeta.ReadOnly(rng_programId,isSigner:false),
                        AccountMeta.ReadOnly(SystemProgram.ProgramIdKey,isSigner:false),
                 },
                Data = buffer
           };
        

       var tx = new TransactionBuilder()
            .SetRecentBlockHash((await rpcClient.GetLatestBlockHashAsync()).Result.Value.Blockhash)
            .SetFeePayer(player.PublicKey)
            .AddInstruction(ix)
            .Build(new List<Account> { player,temp });



        RequestResult<string> firstSig = await rpcClient.SendTransactionAsync(tx);

           
# Coin flip program

We get our accounts

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let payer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let price_feed_account_1: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let price_feed_account_2: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let price_feed_account_3: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let fallback_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let current_feed_accounts: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let temp: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let rng_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let system_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;

Creating account metas for CPI to FPRNG

    let payer_meta = AccountMeta{ pubkey: *payer.key, is_signer: true, is_writable: true,};
    let price_feed_account_1_meta = AccountMeta{ pubkey: *price_feed_account_1.key, is_signer: false, is_writable: false,};
    let price_feed_account_2_meta = AccountMeta{ pubkey: *price_feed_account_2.key, is_signer: false, is_writable: false,};
    let price_feed_account_3_meta = AccountMeta{ pubkey: *price_feed_account_3.key, is_signer: false, is_writable: false,};
    let fallback_account_meta = AccountMeta{ pubkey: *fallback_account.key, is_signer: false, is_writable: false,};
    let current_feed_accounts_meta = AccountMeta{ pubkey: *current_feed_accounts.key, is_signer: false, is_writable: true,};
    let temp_meta = AccountMeta{ pubkey: *temp.key, is_signer: true, is_writable: true,};
    let system_program_meta = AccountMeta{ pubkey: *system_program.key, is_signer: false, is_writable: false,};


Creating instruction to cpi FPRNG

    let ix:Instruction = Instruction { program_id: *rng_program.key,
       accounts: [
        payer_meta,
        price_feed_account_1_meta,
        price_feed_account_2_meta,
        price_feed_account_3_meta,
        fallback_account_meta,
        current_feed_accounts_meta,
        temp_meta,
        system_program_meta,
       ].to_vec(), data: [0].to_vec() };

CPI to FPRNG

    invoke(&ix, 
      &[
        payer.clone(),
        price_feed_account_1.clone(),
        price_feed_account_2.clone(),
        price_feed_account_3.clone(),
        fallback_account.clone(),
        current_feed_accounts.clone(),
        temp.clone(),
        system_program.clone()
        ])?;

Checking players input - zero is head, one is tails

    let players_decision: PlayersDecision = PlayersDecision::try_from_slice(&instruction_data)?;
    if players_decision.decision != 0 && players_decision.decision != 1 {panic!()}


    let returned_data:(Pubkey, Vec<u8>)= get_return_data().unwrap();

Random number is returned from the FPRNG

    let random_number:RandomNumber;
    if &returned_data.0 == rng_program.key{
      random_number = RandomNumber::try_from_slice(&returned_data.1)?;
      msg!("{}",random_number.random_number);
    }else{
        panic!();
    }

We get the mod 2 of the random number. It is either one or zero

    let head_or_tails: u64 = random_number.random_number % 2;

Then we compare with the player's decision just log a message. you can put here your program logic

    if head_or_tails != players_decision.decision {
        msg!("you lost");
    }else{
        msg!("congragulations you win");
    }
