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
    let entropy_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let fee_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let rng_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let system_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    //Creating account metas for CPI to RNG_PROGRAM
    let payer_meta = AccountMeta{ pubkey: *payer.key, is_signer: true, is_writable: true,};
    let entropy_account_meta = AccountMeta{ pubkey: *entropy_account.key, is_signer: false, is_writable: true,};
    let fee_account_meta = AccountMeta{ pubkey: *fee_account.key, is_signer: false, is_writable: true,};
    let system_program_meta = AccountMeta{ pubkey: *system_program.key, is_signer: false, is_writable: false,};


    //Creating instruction to cpi RNG PROGRAM
    let ix:Instruction = Instruction { program_id: *rng_program.key,
       accounts: [
        payer_meta,
        entropy_account_meta,
        fee_account_meta,
        system_program_meta,
       ].to_vec(), data: [100].to_vec() };

    //CPI to RNG_PROGRAM
    invoke(&ix, 
      &[
        payer.clone(),
        entropy_account.clone(),
        fee_account.clone(),
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

