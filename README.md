
<h1 align="center">
    <img src="./assets/icon.png" alt="Minesweeper-Nw Icon" width="100">
    <br>
    Minesweeper-Nw
    <br>
    <a href="https://github.com/Oignontom8283/Minesweeper-nw/releases">
        <img src="https://img.shields.io/github/v/release/Oignontom8283/Minesweeper-nw?include_prereleases&style=flat&logo=github" alt="Version"/>
    </a>
    <a href="https://github.com/Oignontom8283/Minesweeper-nw/actions">
        <img src="https://img.shields.io/github/actions/workflow/status/Oignontom8283/Minesweeper-nw/rust.yml?style=flat&logo=github&color=green" alt="Build Status"/>
    </a>
</h1>

---

Minesweeper-Nw (Minesweeper for Numworks) is a classic minesweeper game for the Numworks calculator.

It takes the form of an external application that must be injected into the calculator, making it available in the applications list alongside the default ones.

<p align="center">
  <img src="./docs/images/gameplay.png" alt="gameplay" width="45%">
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="./docs/images/hard.png" alt="hard mode" width="45%">
</p>
<p align="center">
  <img src="./docs/images/win.png" alt="win screen" width="45%">
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="./docs/images/lose.png" alt="lose screen" width="45%">
</p>
<table align="center" border="0" cellspacing="0">
  <tr>
    <td align="center">
      <img src="./docs/images/animation.gif" alt="animation" width="100%">
    </td>
    <td align="center">
      <p>Compilation Demo Video</p>
      <a href="https://youtu.be/Im5sqfB5wsw?si=7NMjHolD1MnU9g4v" target="_blank" rel="noopener noreferrer">
        <img src="./docs/images/Thumbnail_logo.png" alt="compilation demo video" width="75%">
      </a>
    </td>
  </tr>
</table>


## Controls

- Use the directional arrows to move the cursor on the grid.
- Press the "OK" key to reveal a cell.
- Press the "Back" key to flag/unflag a cell as a mine.
- The rest of the controls will be indicated on the screen at the appropriate time (menus).


## Installation

1. Download the `minesweeper.nwa` file from the [Releases](https://github.com/Oignontom8283/Minesweeper-nw/releases) section of this GitHub repository.

2. Connect your Numworks calculator to your computer using a USB data cable (official or compatible cable).

3. Open a Chromium-based web browser (Google Chrome, Microsoft Edge, Brave, etc.) on your computer or mobile device (yes, it works on mobile too!).

4. Go to the official Numworks injection page: [https://my.numworks.com/apps](https://my.numworks.com/apps).

5. Log in to your Numworks account (or create one if you don't have it).

6. Click the "Connect" button and select your calculator.
> [!NOTE]
> If it is not detected and is properly turned on, turn it off and on again until it works. It may take a few tries.

7. Once connected, click on "Select a NWA file or drop it here" and select the `minesweeper.nwa` file you downloaded.

8. Click on "Install" and wait for the installation to finish (Do not disconnect the calculator during the installation).

9. The **Minesweeper** app should now be visible at the very bottom of the application list (after the basic apps) on your calculator.

## Technologies
- This project uses the [Eadkp](https://github.com/Oignontom8283/eadkp/wiki) framework!
- Developed in Rust. C or C++ would have been easier, but I prefer Rust!
- Uses the following crates: `heapless`, `serde`, `postcard`, `embedded-alloc`, and `eadkp`.
- I'm using Blockbench to create assets (okay, they're ugly, but I'm no artist).

## Why this project?

Because an external application in machine code is much more performant than a Python script,
and much better looking thanks to the ability to use images.

Also, this project is primarily developed for fun, and to improve my Rust skills.


## Contributing

Contributions are welcome!

> The `main` branch is used for development. For stable versions, refer to the "Releases" section of this GitHub repository.

To contribute, use the provided Docker environment and the `justfile` which do all the setup work for you (develop in the container via **Dev Containers** on VS Code).


## Testing locally

1. Ensure `git`, `docker`, and `xhost` are installed. If you are on Windows, perform everything within WSL.
2. Clone the repository.
3. Run `chmod +x ./docker.sh` and `./docker start` to start the docker container (recommended).
4. Wait (this might take a while).
5. Run `./docker shell` to enter the docker environment (recommended).
6. Run `just sim` (`just sim [NB_THREAD]` to specify the number of threads to use, **HIGHLY RECOMMENDED**).
7. Wait (this might take a while).
8. A window representing the Numworks calculator should open.

## License

This project is licensed under the [GPL-3.0 License](./LICENSE) (GNU General Public License version 3).


## Legal Information

This project is in no way affiliated with Numworks or their partners.

