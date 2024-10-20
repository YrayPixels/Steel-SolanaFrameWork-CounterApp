mod add;
mod initialize;

use add::*;
use initialize::*;
        
use tutorial_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&tutorial_api::ID, program_id, data)?;

    match ix {
        TutorialInstruction::Initialize => process_initialize(accounts, data)?,
        TutorialInstruction::Add => process_add(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
