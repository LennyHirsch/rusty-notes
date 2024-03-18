# Rusty Notes
A simple, command line-based note taking tool written in Rust.
Creates a note (with default format .md) and stores it in a given directory, then opens the file in neovim for editing.

## Installation
Clone the repo, and build using `cargo build --release`.
Edit the `config.json` file to point to the directory where you want your notes to be stored. You can also change the default file format here.
Place the `config.json` file in the same directory as the compiled binary i.e. `target/release/`.

## Using Rusty Notes
I recommend creating a new alias to run the executable.

Powershell: add `New-Alias note C:\path\to\rusty_notes.exe` to the bottom of your Powershell profile.
The location of the Powershell profile can be found by running `echo $profile` in Powershell.

Save changes to the profile, and restart the shell.
You should now be able to create a new note by typing `note hello_world` in Powershell. This will create a new note called `hello_world.md`, store it in your specified notes directory, and open it in neovim for editing.

Bash: I dunno I haven't got this far yet, bear with me.
