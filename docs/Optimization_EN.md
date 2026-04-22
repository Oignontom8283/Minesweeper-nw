# Optimizations

The translation might be incorrect or have some strange phrasing; sorry, I'm not a native English speaker 😓.

The code comments are also in French, sorry about that as well.

## Memory

### Bitpacking

I use bitpacking to store the data for the game grid cells.
Instead of using a structure that would take several bytes for the data of a single cell,
I use a single `u8` byte. I store the data in the byte as follows:

```
Bit :  7  6  5  4  3  2  1  0
       └───┬────┘     │  │  └─ Is mine
           │          │  └──── Is revealed
           │          └─────── Is flagged
           │       
           └────────────────── Adjacent mines (0-8)
```

For low-level enthusiasts, the masks are as follows:

| Bits     | Mask          | Role                                        |
| -------- | ------------- | ------------------------------------------- |
| Bit 0    | `0b0000_0001` | **Mine** - 1 = there is a mine              |
| Bit 1    | `0b0000_0010` | **Revealed** - 1 = the cell has been opened |
| Bit 2    | `0b0000_0100` | **Flag** - 1 = flag placed                  |
| Bit 3    | `0b0000_1000` | *unused*                                    |
| Bits 4–7 | `0b1111_0000` | **Adjacent mines** - number from 0 to 8     |

I allocate 4 bits for the number of adjacent mines, because `2^4 = 16`, which is enough.
Why not 3 bits? Because `2^3 = 8`, so `8-1=7`; I would have needed bit-shifting operations, and I was not motivated enough 😅.

This system reduces memory usage by 4. A standard struct in its best configuration
would take one byte per field, so 4 bytes instead of one. Therefore, for a grid of `10*10 = 100` -> `100 * 4 = 400`,
which gives 0.39 KB, whereas with bitpacking we only use `100 * 1 = 100`, or 0.097 KB.

You can find the relevant code in [`src/grid.rs`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/grid.rs)
at the beginning of the file.

### Memory fragmentation

To avoid memory fragmentation (which can happen very quickly), instead of deallocating the `Vec`
that stores the grid data, I reuse it with `.clear() + .resize()`.

This avoids creating holes in memory and therefore unusable free spaces that are too small.

Code location: [`src/scenes/playing.rs#L14`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/scenes/playing.rs#L14)

## Algorithms

The `reveal_infect` function is used to reveal empty cells (0 adjacent mines) and their neighbors.
I used an iterative approach (with my own list of cells to reveal) rather than
a recursive one to avoid stack overflow issues.

It only returns the cells that need to be redrawn and marks them as revealed.

Also, I do not like recursive functions, regardless of the language.
They are often more resource-hungry than an iterative approach.

The function: [`src/grid.rs#L132`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/grid.rs#L132)

## Rendering

### Dirty render

The calculator screen has severe screen tearing, which prevents me from redrawing the entire screen every refresh.

So I use a dirty rect system through a render instruction pipeline:
I only redraw the elements that have changed (dirty), not the whole screen.

#### Cursor

When the cursor moves, only the cell it came from is redrawn.
For the destination cell, the cursor is drawn on top of the existing display.

[`src/render.rs#L64`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/render.rs#L64)

#### Revealed cell propagation

The `reveal_infect` function returns the list of cells to redraw,
and the game logic is responsible for sending that list of cells into the render pipeline.

[`src/scenes/playing.rs#215`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/scenes/playing.rs#L215)
[`src/grid.rs#L132`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/grid.rs#L132)

#### Timer

The in-game clock display is updated once per second, right after the time increases by one second.
Same issue here: I cannot redraw the clock constantly because it would create visual artifacts on the screen.

[`src/scenes/playing.rs#234`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/scenes/playing.rs#L234)

## Saving

The calculator file system has a very limited size (42 KB), so large files cannot be stored without reason.

That is why, instead of using a classic text file format (such as JSON), I directly save binary files
that are serializations of structures (such as `GameSave`), serialized/deserialized with `serde` and `postcard`.

This makes save files very compact.

[`src/save.rs`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/save.rs)
[`src/common.rs#127`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/common.rs#L127)
[`src/common.rs#142`](https://github.com/Oignontom8283/Minesweeper-nw/blob/e0d4959b9f70b94c06e0c4f22b6344a244f8a8ff/src/common.rs#L142)
