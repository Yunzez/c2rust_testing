# Anonymized data branch

This branch is an orphan snapshot of the working tree with author-identifying
strings removed for double-blind review. No git history is carried over.

Substitutions (same length, so binary layouts such as `.profraw` are preserved):

| original | replacement | where |
|---|---|---|
| the author's Unix user name | `anonym` | every text file (`/home/anonym/...`, `/tmp/claude-1000/-home-anonym-c2rust-testing/...`) |
| a second account name | `anonymx` | `.vscode/settings.json` |

43 `*.tar.gz` archives under `results/` whose contents embedded the user name
(llvm-cov JSON exports and `.profraw` files with cargo-registry paths) were
unpacked, rewritten with the same substitution, and repacked. Their checksums
therefore differ from the originals; the member files are otherwise unchanged.

Remaining home-directory paths (`/home/yzq`, `/Users/emre`, `/home/vagrant`,
`/home/u`) come from third-party translator artifacts and identify their
upstream authors, not the authors of this work.
