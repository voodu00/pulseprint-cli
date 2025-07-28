# Branch Protection Rules

To complete the CI/CD setup, configure the following branch protection rules for the `main` branch:

## Required Settings

1. **Navigate to**: Settings → Branches → Add rule
2. **Branch name pattern**: `main`
3. **Enable these protections**:
   - ✅ Require a pull request before merging
     - ✅ Require approvals: 1
     - ✅ Dismiss stale pull request approvals when new commits are pushed
   - ✅ Require status checks to pass before merging
     - ✅ Require branches to be up to date before merging
     - **Required status checks**:
       - `Test Suite (ubuntu-latest, stable)`
       - `Test Suite (windows-latest, stable)`
       - `Test Suite (macos-latest, stable)`
       - `Rustfmt`
       - `Clippy`
       - `Security Audit`
       - `Check`
   - ✅ Require conversation resolution before merging
   - ✅ Include administrators (optional, but recommended)

## Notes
- The CI workflow must run at least once before status checks appear in the selection list
- Consider making the beta and nightly tests non-required to prevent breaking changes from blocking PRs
- The status check names must match exactly as they appear after the first CI run