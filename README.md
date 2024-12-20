# Advent of Code

Usage: 
- Copy main.rs.disabled from a days folder to src/main.rs
- cargo build --release
- time target/release/advent.exe <src/dayX/input

My IDE doesn't like having multiple rust files in scope unless I declare them as modules, and that's not needed since
its dead code. My workaround is to name them x.disabled so the IDE ignores them.

On day 5 or 6 I spent over my alotted time and consulted reddit for the solution, otherwise its just me and Gemini
learning Rust together.

I'm very happy with how day 10 turned out. Day 9 on this computer is incomplete I will fix later.

If you want part 1 solutions you may have to dig through git history.

### Building
```shell
cargo build --release
```

### Measuring

``` shell
hyperfine --setup 'cargo build --release' --warmup 3 --shell zsh './target/release/advent.exe < ./src/day11/input' 
```

### Helper

```shell
DAYXX=day21
mkdir src/$DAYXX
touch src/$DAYXX/input{,a,b}
```