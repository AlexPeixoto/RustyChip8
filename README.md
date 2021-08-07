# RustyChip8
CHIP-8 emulator written in rust


This is my very first RUST project and it probably breaks a lot of expectations from what a good RUST code would look like.
So it was used as a learning opportunity (this was literally my hello world).

I still want to go back and reorder some stuff on the code, move around a little bit of the code and do some cleanup as I feel that I
can improve quite a bit how its organized.
A lot of this came from my learning process I am aware that I can do a few things:
- Improve the code overall (Move around some variables, rename, rethink some of them)
- Remove &mut self in several places
- Perhaps break down the fn main a bit more
- Add some methods to access some of the data structures.


The emulator doesn't have sound (YACH8WA), but at this time it was a design choice basically because there are only beeps
which can easily be done with SFML, but its not being done here, even if the sould timer do exist internally.

The code passes test rom, keyboard rom, PONG, Maze, RNG and probably several others and do not implement Super Chip-48 instructions.

Below are some images of it running

![It works](https://github.com/AlexPeixoto/RustyChip8/blob/main/imgs/logo_ch8.png)

![Pong](https://github.com/AlexPeixoto/RustyChip8/blob/main/imgs/pong.jpeg)

![Tests](https://github.com/AlexPeixoto/RustyChip8/blob/main/imgs/tests.jpeg)

