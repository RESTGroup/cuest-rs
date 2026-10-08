# Important notice to AI agent

- **Local development**: Some important files and notices at @CLAUDE.local.md or @AGENTS.local.md . If both files does not exist, you can request user to create one for local development.
- **Never reference local file path**: Local path should not be occur in source code or document, especially git tracked files. If you need record/benchmark, please use git ignored paths, where you can do and record anything.
- **Git commit rule**: Please follow skill `git-commit-coauthor` when git commit.
- **Write in English**: The user may interact in any language (Chinese, English, for example). As AI agent, communicate in the same language as the user. However, code, code comments, documentation, and commit messages should always be in English.
- **Code comments be concise and descriptive, not explanatory**: In many situations, code can explain itself. Some AI models can be too verbose in code comments if no harness restrictions. Make comments concise. Limit non-docstring comments to 1-2 lines, no more than 4 lines; break this rule unless very necessary. Docstring are for API callers; detailed docstring is better, but do not expand explanation on algorithms and techniques in docstring comments.
- **Large files**: If you find large or bulky files in a published repository — especially ones that are or would be handled via `.gitignore` — do not act on your own: ask the user. Whether such files stay ignored, get tracked (possibly via Git LFS), or get deleted is the maintainer's decision.

