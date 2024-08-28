use fuzz_instructions::trident_lesson_part_i_fuzz_instructions::InitializeIx;
use trident_lesson_part_i::entry as entry_trident_lesson_part_i;
use trident_lesson_part_i::ID as PROGRAM_ID_TRIDENT_LESSON_PART_I;
const PROGRAM_NAME_TRIDENT_LESSON_PART_I: &str = "trident_lesson_part_i";
use fuzz_instructions::trident_lesson_part_i_fuzz_instructions::FuzzInstruction as FuzzInstruction_trident_lesson_part_i;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_trident_lesson_part_i;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init = FuzzInstruction::InitializeIx(InitializeIx::arbitrary(u)?);
        Ok(vec![init])
    }
    fn ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        Ok(vec![])
    }
    fn post_ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        Ok(vec![])
    }
}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_TRIDENT_LESSON_PART_I,
                &PROGRAM_ID_TRIDENT_LESSON_PART_I,
                processor!(convert_entry!(entry_trident_lesson_part_i))
            );

            let fuzzing_program2 = FuzzingProgram::new(
                "metaplex-token-metadata",
                &anchor_spl::metadata::ID,
                None
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1,fuzzing_program2])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_TRIDENT_LESSON_PART_I, &mut client);
        });
    }
}
