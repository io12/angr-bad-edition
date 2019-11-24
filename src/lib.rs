/// Solve for an assertion failure
/// - **Parameters**
///   - `prog`: program to solve
///   - `entry_name`: name of entry point function
/// - **Return value**: stdin bytes that cause an assertion failure, if found
pub fn solve_assert_fail(prog: llvm_ir::Module, entry_name: &str) -> Option<Vec<u8>> {
    let entry_func = prog.functions.iter().find(|func| func.name == entry_name)?;
    unimplemented!()
}
