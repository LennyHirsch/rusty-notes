# Rusty Notes
Rusty Notes is a simple, command line-based note taking tool written in Rust.
It creates a note and stores it in a given directory, then opens the file in Neovim for editing.

A `#rustynote` tag is inserted on the first line. This should allow for relatively easy integration into note-taking software such as Logseq.
By setting the notes directory to Logseq's `pages` folder, any new notes are put directly in here, and can be found by Logseq. These notes can be found simply by searching for the `#rustynote` tag.

## Installation
Clone the repo, edit the `config.json` file to point to the directory where you want your notes to be stored. You can also change the default file format here.
Build using either `cargo build` (debug), or `cargo build --release` (optimised release build).

## Usage
After running `cargo build --release`, place the `config.json` file in the same directory as the compiled binary i.e. `target/release/`.

To run the binary, I recommend creating a new alias to run the executable.

### Powershell
Add `New-Alias note C:\path\to\rusty_notes.exe` to the bottom of your Powershell profile.
The location of the Powershell profile can be found by running `echo $profile` in Powershell.
Save changes to the profile, and restart the shell.

### Bash/Zsh/Everything else
I dunno I haven't got this far yet, bear with me.

Once the new alias is created, run the binary with a single argument: the name of the new note e.g. `note hello_world`. A file called `hello_world.md` will be created in the directory specified in `config.json`, and opened in Neovim for editing.
If you don't add a file extension to the name, it will be created with the default file format (.md). If a file extension is given, that extension will be used for the newly created note.
The default file format can be changed in the config.json file.

[NOTE]: Once the binary is compiled, only the `rusty_notes.exe` binary and the `config.json` file are needed. The source files can be discarded; just make sure to place `config.json` in the same directory as `rusty_notes.exe`.

### Using `cargo build` (debug build)
If you have not built for release, place the `config.json` file in the same directory as the compiled binary i.e. `target/debug/`.
You should now be able to run the script by calling `cargo run -- filename`.
