**Give an overview of the tasks completed**
Give a clear and high level level overview of the tasks completed in this pull request. 

**Link to issue(s) that this PR closes**
Pull requests are required to close a related issue. If there is no issue, please create one first.

**Please look at but feel free to delete the following sections in your PR**

**Provide an adequate title with tags and a brief description**
The tags should be in the format of "tag:" Description. The tags are as follows:
- "feat:" A new feature
- "fix:" A bug fix
- "docs:" Documentation only changes
- "style:" Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
- "refactor:" A code change that neither fixes a bug nor adds a feature
- "perf:" A code change that improves performance
- "test:" Adding missing or correcting existing tests
- "chore:" Changes to the build process or auxiliary tools and libraries such as documentation generation
- "repo:" Changes to the repository itself

**REMINDER! Please check that you have done the following prior to submitting this PR:**
- [ ] Checked that the relevant version(s) have been incremented if necessary.
- [ ] Ran both and made any changes necessary for:
    - `cargo +nightly fmt --all`
    - `cargo clippy --all`
    
You can install the nightly toolchain on your system by:
```bash
rustup toolchain install nightly
```
