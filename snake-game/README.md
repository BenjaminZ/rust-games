# A Snake Game

This is a Rust practice game.
It renders a snake that can be controlled by "W", "A", "S", "D", and arrow keys.

The snake body is assembled from "[⬛](https://www.compart.com/en/unicode/U+2B1B)" (if any),
head is from "[▲](https://www.compart.com/en/unicode/U+25B2)",
"[▼](https://www.compart.com/en/unicode/U+25BC)",
"[◀](https://www.compart.com/en/unicode/U+25C0)", and
"[▶](https://www.compart.com/en/unicode/U+25B6)" based on the facing.

The snake can eat apples to grow.
An apple is "[⬤](https://www.compart.com/en/unicode/U+2B24)".

The board is 20*20 size.
The rest of the board is filled with "[⬜](https://www.compart.com/en/unicode/U+2B1C)".
