# Instructions for use: 
- Ensure you have ran Starcraft 2 and played at least 1 game of Runling Run 4 before using this tool. 
- Download the .exe file from the latest release, and execute it via powershell. 
    - Using --help should give a lot more information about usage instructions
    - The .exe currently has 2 subcommands - it can unlock all units, or create a new unit with a given type and level
    - An example command should look something like `.\Downloads\runling-run-edit.exe --file-location 'C:\Users\<USERNAME>\Documents\StarCraft II\Accounts\<ACCOUNT ID>\1-S2-1-<PLAYER HANDLE>\Banks\1-S2-1-417073\RunlingRun004.SC2Bank' --backup-location C:\Users\<USERNAME>\Desktop\SC2backup3 create-new-unit --runling-type baneling --level 5`
- You should now be able to boot up SC2 and see your changes reflected in the game

- You can also build and run this from source via cargo - install rust and cargo, then `cargo run -- <ARGS>`

# Heavily inspired by https://github.com/Apollys/rlr4-bank-editor - all credit goes to them