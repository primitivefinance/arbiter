#[allow(missing_docs)]
#[cfg(test)]

// TODO: Add contract bindings and integration tests here. 
// We need to test things like: having managers create environments with agents that run.
// This will lead to testing out transactions and contract calls.

mod tests {
    use anyhow::Result;

    #[test]
    /// Test that the writer contract can echo a string.
    /// The writer contract takes in no constructor args.
    fn string_write() -> Result<()> {
        Ok(())
    }

    #[test]
    fn token_mint() -> Result<()> {
        Ok(())
    }

    #[test]
    fn auto_deploy() -> Result<()> {
        Ok(())
    }

    #[test]
    fn arbiter_math() -> Result<()> {
        Ok(())
    }
}
