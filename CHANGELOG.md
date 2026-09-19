# 0.2.2 (19-09-2026)

Add help message on how to install completions for zsh.

Add flag `--new` to `open` to create note if it does not exist.

Add a flag `--force` for `archive restore` to replace the existing note if the
name of the restored note is already taken.

Add more concrete error messages for invalid note names.

# 0.2.1 (15-09-2026)

Add `rename` command.

# 0.2.0 (23-08-2026)

Rename directory `notebooks` to `notebook`.
Since each entry is now called a note the sum of all notes is the notebook.

Improve error handling in `file_operations.rs`.

Handle some open edge cases with errors instead of panicking.
