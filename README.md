# Steam Deck SD Card Scanner

 An application to help you keep track of the different games you have on your SD Cards. If you ever found yourself wondering if you already have a game installed on a different SD Card then this is for you.

<img alt="SD Card Scanner" src="https://i.imgur.com/NJYFN3t.jpg">

### How to use:

- Switch to Desktop mode to download and install
- Download the pre-compiled binary from the [Releases](https://github.com/ddotthomas/SteamDeck_SD_Card_Scanner/releases) or compile your own

- Add the downloaded binary as a Non-Steam Game

<img alt="Non-Steam Game" src="https://snowydunestorage.blob.core.windows.net/web/pinmore/howto/non-steam-1.png">

Then when you launch the program, it will scan the current inserted SD Card and add it and all its games to the list. You can switch SD Cards and restart the program or hit Scan Card to force a scan of the current card, the program will detect them as different cards and keep track of which one has which games.

### Things to be added or improved/ known issues

- There's currently an issue where it looks like gamescope sees the application at a different resolution than it actually is, more info [here](https://www.reddit.com/r/SteamDeck/comments/10jk36q/having_issues_with_the_windows_size_while/). I recommend switching the controller layout to the "Web Browser" scheme provided by Valve. Otherwise, the mouse can't make it to the scroll bar on the right side to scroll down.
- Let the program scan and add Non-Steam Games to the list. (Lutris and Heroic)
- Allow the user to search for a game, filtering the list
- Add a settings page, allow the user some control over how the cards are identified, scanned, and more
- Make a better UI
- Cards are named automatically and the system for picking a number isn't perfect

## How to Compile

- First, download Rust, I recommend using [Rustup](https://www.rust-lang.org/tools/install)
- Update Rust ```rustup update```
- Clone the repository, ```https://github.com/ddotthomas/SteamDeck_SD_Card_Scanner```
- Switch to the new Directory and install ```cd SteamDeck_SD_Card_Scanner; cargo build --release```
- Look for the binary in the ```target/release``` directory titled "steamdeck_sd_card_scanner"
- I recommend moving the binary to ```~/.local/bin/``` but you can leave it anywhere you want when you add it as a Non-Steam Game

## Support

If you would like to support the project feel free to make a pull request with improvements or changes, or consider supporting me on [Patreon](patreon.com/ddotthomas)


