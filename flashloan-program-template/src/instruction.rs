use solana_program::program_error::ProgramError;

use crate::error::MyFlashloanProgramError::{InvalidInstruction,InstructionUnpackError};
use std::{convert::TryInto, mem::size_of, borrow::BorrowMut};

pub enum MyFlashloanProgramInstruction {

    /// Creates and populates MyFlashloanProgram account + transfers ownership of the given token account to the PDA
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing Flashloan Program
    /// 1. `[writable]` Flashloan Program's token account that should be created prior to this instruction and owned by the initializer
    /// 3. `[writable]` The Flashloan Program's account, it will hold all necessary info about the program.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    InitMyFlashloanProgram {

	},

	/// An instruction which will be called by the Lending program after receiving a flashloan by MyFlashloanProgram 
	// approves flashloan token amount transfer by the Lending program 
	///   0 '[]' Lending program id
    ///   1 '[]` Token program id
	ExecuteOperation {
		amount: u64,
	},

	/// Flash borrows tokens from a Lending Program borrowing reserve by invoking its flashloan instruction.
    ///   1. `[writable]` Destination liquidity token account (caller program's token account), minted by borrow reserve liquidity mint
    ///   2. `[writable]` Borrow reserve account.
    ///   3. `[writable]` Borrow reserve liquidity supply SPL Token account
    ///   4 `[]` Lending market account.
    ///   5 `[]` Derived lending market authority.
	///   6 '[]' Caller program id
    ///   7 '[]` Token program id
    MyFlashloanCall {
        /// token amount
        amount: u64,
		execute_operation_ix_data: Vec<u8>,
    },
}

impl MyFlashloanProgramInstruction {
    /// Unpacks a byte buffer into a [MyFlashloanProgramInstruction](enum.MyFlashloanProgramInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitMyFlashloanProgram {},
			1 => {
				let (amount, rest) = Self::unpack_u64(rest)?;

				Self::ExecuteOperation{
					amount
				}
			},
			2 => {
				let (amount, execute_operation_ix_data_slice) = Self::unpack_u64(rest)?;
                let execute_operation_ix_data = execute_operation_ix_data_slice.to_vec();
				
				Self::MyFlashloanCall {
                    amount,
					execute_operation_ix_data,
                }
			},
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(InstructionUnpackError)?;
            Ok((amount, rest))
        } else {
            Err(InstructionUnpackError.into())
        }
    }

	fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if !input.is_empty() {
            let (amount, rest) = input.split_at(1);
            let amount = amount
                .get(..1)
                .and_then(|slice| slice.try_into().ok())
                .map(u8::from_le_bytes)
                .ok_or(InstructionUnpackError)?;
            Ok((amount, rest))
        } else {
            Err(InstructionUnpackError.into())
        }
    }

	/// Packs a [LendingInstruction](enum.LendingInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match *self {
            Self::InitMyFlashloanProgram {} => {
                buf.push(0);
            }
            Self::ExecuteOperation{
                amount,
            } => {
                buf.push(1);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
			Self::MyFlashloanCall {
                amount,
				execute_operation_ix_data
            } => {
                buf.push(2);
                buf.extend_from_slice(&amount.to_le_bytes());
				let mut execute_operation_ix_data_borrowed = execute_operation_ix_data;
				buf.append(execute_operation_ix_data_borrowed.borrow_mut());
            }
        }
        buf
    }
}