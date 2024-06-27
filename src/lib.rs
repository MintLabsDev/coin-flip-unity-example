use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},entrypoint, entrypoint::ProgramResult, instruction::{AccountMeta, Instruction}, msg, program::{get_return_data, invoke}, pubkey::Pubkey
  };
  
  entrypoint!(process_instruction);


  #[derive(BorshDeserialize)]
pub struct RandomNumber{
  pub random_number:u64,
}

#[derive(BorshDeserialize)]
pub struct PlayersDecision{
    pub decision:u64,
  }

  fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    
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

    //Creating account metas for CPI to RNG_PROGRAM
    let payer_meta = AccountMeta{ pubkey: *payer.key, is_signer: true, is_writable: true,};
    let price_feed_account_1_meta = AccountMeta{ pubkey: *price_feed_account_1.key, is_signer: false, is_writable: false,};
    let price_feed_account_2_meta = AccountMeta{ pubkey: *price_feed_account_2.key, is_signer: false, is_writable: false,};
    let price_feed_account_3_meta = AccountMeta{ pubkey: *price_feed_account_3.key, is_signer: false, is_writable: false,};
    let fallback_account_meta = AccountMeta{ pubkey: *fallback_account.key, is_signer: false, is_writable: false,};
    let current_feed_accounts_meta = AccountMeta{ pubkey: *current_feed_accounts.key, is_signer: false, is_writable: true,};
    let temp_meta = AccountMeta{ pubkey: *temp.key, is_signer: true, is_writable: true,};
    let system_program_meta = AccountMeta{ pubkey: *system_program.key, is_signer: false, is_writable: false,};


    //Creating instruction to cpi RNG PROGRAM
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

    //CPI to RNG_PROGRAM
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

    //Checking players input - zero is head, one is tails
    let players_decision: PlayersDecision = PlayersDecision::try_from_slice(&instruction_data)?;
    if players_decision.decision != 0 && players_decision.decision != 1 {panic!()}


    let returned_data:(Pubkey, Vec<u8>)= get_return_data().unwrap();

    //Random number is returned from the RNG_PROGRAM
    let random_number:RandomNumber;
    if &returned_data.0 == rng_program.key{
      random_number = RandomNumber::try_from_slice(&returned_data.1)?;
      msg!("{}",random_number.random_number);
    }else{
        panic!();
    }

    //we get the mod 2 of the random number. It is either one or zero
    let head_or_tails: u64 = random_number.random_number % 2;

    //then we compare with the player's decision just log a message. you can put here your program logic
    if head_or_tails != players_decision.decision {
        msg!("you lost");
    }else{
        msg!("congragulations you win");
    }

    Ok(())
  }

