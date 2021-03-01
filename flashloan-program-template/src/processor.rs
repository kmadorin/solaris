use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
    program::invoke
};

use crate::{instruction::MyFlashloanProgramInstruction, error::MyFlashloanProgramError, state::MyFlashloanProgram};

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = MyFlashloanProgramInstruction::unpack(instruction_data)?;

        match instruction {
            MyFlashloanProgramInstruction::InitMyFlashloanProgram {} => {
                msg!("Instruction: InitMyFlashloanProgram");
                Self::process_init_my_flashloan_program(accounts, program_id)
            }
			MyFlashloanProgramInstruction::ExecuteOperation {
				amount
			} => {
                msg!("Instruction: ExecuteOperation");
                Self::process_execute_operation(accounts, amount, program_id)
            }
			MyFlashloanProgramInstruction::MyFlashloanCall {
				amount,
				execute_operation_ix_data,
			} => {
                msg!("Instruction: MyFlashloanCall");
                Self::process_my_flashloan_call(accounts, amount, execute_operation_ix_data, program_id)
            }
        }
    }

	fn process_init_my_flashloan_program(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

		let flashloan_token_account = next_account_info(account_info_iter)?;

		let flashloan_program_account = next_account_info(account_info_iter)?;
		let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

		if !rent.is_exempt(flashloan_program_account.lamports(), flashloan_program_account.data_len()) {
			return Err(MyFlashloanProgramError::NotRentExempt.into());
		}

		let mut flashloan_program_info = MyFlashloanProgram::unpack_unchecked(&flashloan_program_account.data.borrow())?;
		if flashloan_program_info.is_initialized() {
			return Err(ProgramError::AccountAlreadyInitialized);
		}

		flashloan_program_info.is_initialized = true;
		flashloan_program_info.initializer_pubkey = *initializer.key;
		flashloan_program_info.flashloan_token_account_pubkey = *flashloan_token_account.key;

		MyFlashloanProgram::pack(flashloan_program_info, &mut flashloan_program_account.data.borrow_mut())?;

		let (pda, _bump_seed) = Pubkey::find_program_address(&[b"my_flashloan_program"], program_id);
        
		let token_program = next_account_info(account_info_iter)?;
		let owner_change_ix = spl_token::instruction::set_authority(
			token_program.key,
			flashloan_token_account.key,
			Some(&pda),
			spl_token::instruction::AuthorityType::AccountOwner,
			initializer.key,
			&[&initializer.key],
		)?;

		msg!("Calling the token program to transfer token account ownership...");
		invoke(
			&owner_change_ix,
			&[
				flashloan_token_account.clone(),
				initializer.clone(),
				token_program.clone(),
			],
		)?;

		Ok(())
    }

	fn process_execute_operation(
        accounts: &[AccountInfo],
		amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {

        // Program now has the funds requested.
        // Your logic goes here.
        //

        // At the end of your logic above, this program owes
        // the flashloaned amounts + premiums.
        // Therefore ensure your program flashloan_token_account has enough to repay
        // these amounts.

        // Approve the Lending Program allowance to *pull* the owed amount


		Ok(())
	}

	fn process_my_flashloan_call(
        accounts: &[AccountInfo],
		amount: u64,
		execute_operation_ix_data: Vec<u8>,
        program_id: &Pubkey,
    ) -> ProgramResult {

		//invoke the flashloan instruction of the Lending Program


		Ok(())
	}
}