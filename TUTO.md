Hello guys !

Arn here and I am a programmer, mainly `front_end & full_stack` *React/Angular/Redux/Ngrx/Javascript/Typescript + Node* **I was** and **now surely**  *Rust/[seed-rs](https://github.com/seed-rs/seed)* `full_stack`.

Today I write my first big tutorial ever and I 'll show you something amazing I have discovered last week. I am new to [Rust](https://www.rust-lang.org/) and I love it, so if there are few experienced Rustaceans who come by and see my ugly code, then your welcome to tell me :D.

Anyway I discovered [Wasm](https://webassembly.org/) few months ago and I started to play with this tutorial there :

https://rustwasm.github.io/docs/book/introduction.html

This book explains the basics about how to use Rust to actually make some front end stuff and use the compiled code with Javascript. I was so curious about it especially because Rust and also because my computer get too slow when doing front end stuff on larger apps... RAM issue there when too many files in Angular.

I started to really work on the tutorial last week and then it went " Ho My God " in my head. Having the rust toolchain, the amazing smart compiler with you while doing front end work and the performance.... 

Just the feelings of using little bit of rust to make front end made me feel so satisfied and happy and so motivated.

So after playing with this tutorial, I was like trying to work with WebGL instead of Canvas rendering and then maybe somebody made something using WebGl. Then I started to think, maybe somebody made a Web Framework so we can use only Rust and no need for Javascript. Then I guess it would be so crazy. Having the most solid/robust/safe/efficient programming language and use it for front end.

And I saw [Seed](https://github.com/seed-rs/seed) and then my head blew up!

The day after I cloned a quick starter app for Seed, I decided to adapt the Game of Life tutorial into **Rust only** code.

It took me few hours to do it. Since I am new to Rust and I am also a bit rusty ( so bad joke ) , I think that few hours is fairly good when new to a new Framework & language.

So I am gonna show you in this tutorial the differences between the original [game of life I made](https://github.com/arn-the-long-beard/wasm-game-of-life) and the seeded_game_of_life.
It is recommended to know some stuff for this tutorial 
- You have read about Rust
- You have read about Javascript
- You have read about Wasm
- It would be very nice if you have followed once the original [game of life tutorial](https://rustwasm.github.io/docs/book/introduction.html) 

NB :
- I show you a very brutal and straight forward conversion. There is awesome stuff in Seed that can be used and that improves the code a lot like **Reference** to element from the doc =>  `ElRef<T>` . I just skip it for now.



### Summary 

1 - Setup
2 - Take the Rust code from the original tutorial for `Universe`
3 - Let's build the core of the app from Js to Rust
4 - Let's try test & benchmark
5 - How to improve performances
6 - Conclusion


Let's go !

### 1 - Setup 

You can skip this section if you clone the quick start found at 'https://github.com/seed-rs/seed-quickstart' but the below shows quickly steps to create a project from scratch.

`cargo new seeded-game-of-life --lib`

Our project is a library.

We are gonna add few things :

Let's update our `Cargo.toml`

`Cargo.toml`
```TOML
[package]
version = "0.1.0"
name = "seeded-game-of-life"
repository = "https://github.com/seed-rs/seed-quickstart"
authors = ["Your Name <email@address.com>"]
description = "App Description"
categories = ["category"]
license = "MIT"
readme = "./README.md"
edition = "2018"

[lib]
crate-type = ["cdylib", ,"rlib"]

[dependencies]
seed = { git = "https://github.com/seed-rs/seed", rev = "0a538f0" }
[dependencies.web-sys]
version = "0.3"
```

Okay here we have 3 important points :

 - `cdylib` is for compiling to .wasm for what I understand. If you try to compile without it you are gonna get errors

 - `seed = { git = "https://github.com/seed-rs/seed", rev = "0a538f0" }`

This is our main dependency because it is the Web Framework :D

 - ` [dependencies.web-sys]
version = "0.3"
`

[web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/index.html) exposes most of the Web browser Api to Rust. Depending of what you need you might activate some features.


We need to run few tasks to `watch and compile` and also `serve` files. 

`cargo install cargo-make`

Then make this file `MakeFile.toml`

We can also use it to make release build. There is no need to understand what there is inside :P for now.

`MakeFile.toml`
```TOML
[env]
PORT = "8000"

[config]
skip_core_tasks = true

# ---- BASIC ----

[tasks.watch]
description = "Watch files and recompile the project on change"
run_task = [
    { name = "build" },
]
watch = true

[tasks.serve]
description = "Start server"
install_crate = { crate_name = "microserver", binary = "microserver", test_arg = "-h" }
command = "microserver"
args = ["--port", "${PORT}"]

[tasks.verify]
description = "Format, lint with Clippy and run tests"
dependencies = ["fmt", "clippy", "test_h_firefox"]

# ---- BUILD ----

[tasks.build]
description = "Build with wasm-pack"
install_crate = { crate_name = "wasm-pack", binary = "wasm-pack", test_arg = "-V" }
command = "wasm-pack"
args = ["build", "--target", "web", "--out-name", "package", "--dev"]

[tasks.build_release]
description = "Build with wasm-pack in release mode"
install_crate = { crate_name = "wasm-pack", binary = "wasm-pack", test_arg = "-V" }
command = "wasm-pack"
args = ["build", "--target", "web", "--out-name", "package"]

# ---- LINT ----

[tasks.clippy]
description = "Lint with Clippy"
install_crate = { rustup_component_name = "clippy", binary = "cargo-clippy", test_arg = "--help" }
command = "cargo"
args = ["clippy", "--all-features", "--", "--deny", "warnings", "--deny", "clippy::pedantic", "--deny", "clippy::nursery"]

[tasks.fmt]
description = "Format with rustfmt"
install_crate = { rustup_component_name = "rustfmt", binary = "rustfmt", test_arg = "-V" }
command = "cargo"
args = ["fmt"]


# ---- TEST ----

[tasks.test_h]
description = "Run headless tests. Ex: 'cargo make test_h firefox'. Test envs: [chrome, firefox, safari]"
extend = "test"
args = ["test", "--headless", "--${@}"]

[tasks.test_h_firefox]
description = "Run headless tests with Firefox."
extend = "test"
args = ["test", "--headless", "--firefox"]

[tasks.test]
description = "Run tests. Ex: 'cargo make test firefox'. Test envs: [chrome, firefox, safari]"
install_crate = { crate_name = "wasm-pack", binary = "wasm-pack", test_arg = "-V" }
command = "wasm-pack"
args = ["test", "--${@}"]

```


Add the index.html file 

Our app is a Single Page Application. We are gonna feed it with `Wasm` and also some generated js.
 
`index.html`

```Html
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
    <title>Seeded Game of life</title>
</head>

<body>
    <section id="app"></section>
    <script type="module">
        import init from '/pkg/package.js';
        init('/pkg/package_bg.wasm');
    </script>
</body>
</html>

```

We have an import to a generated js file that contains communication between JS and WASM, because under the hood we have sometimes calls to some standard js function used to talk with the web browser. 

This should change in the future.

Then we have the package_bg.wasm that contains compiled binaries from our **lovely** Rust :)


Now let's add an example code in our `lib.rs` and see if it works.

`lib.rs`

```Rust
use seed::{prelude::*, *};

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// `Model` describes our app state.
type Model = i32;

// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![
            model,
            ev(Ev::Click, |_| Msg::Increment),
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
```

In your .gitignore

```
/target
Cargo.lock
/pkg  
```

At least on linux, I need to do this otherwise the change are detected in `/pkg` and cargo keeps rebuilding every time I have compiled :P Maybe a tiny bug to fix there :D

We have now :

```
seeded-game-of-life/
├── .gitignore
├── Cargo.toml
├── index.html
├── MakeFile.toml
└── src
    ├── lib.rs

```
Then

`cargo make watch` in one terminal -> compile on changes

`cargo make serve` in an other one -> serve the file


[http://localhost:8000](http://localhost:8000)

You should have a nice counter.

There will be no more setup to do except for test & release build  and usage of `web-sys`

If we compare to the original setup for game of life we do not need `www` folder
- no package json
- no webpack
- no javascript code
- no node_modules

Basically, with 4 files  & 2 dependencies ( sys-web + seed) we have the base to make web developement.

This is a huge improvement !!!! We can easily focus and be productive :).
Last week_end I showed this to a friend of mine new to web development. He has been struggling to use React, because there are so many dependencies and knowledge to get on webpack, package.json and so on. The same applies to Angular or Vue. Even if with time all of them get easier to use and configure, they still can be pretty challenging to people not familiar with Javascript and coming from more low level programming. There are so many files everywhere :P
And so many concepts and levels of abstraction to understand.

In the first hour with Rust/Seed my friend did update the code you will see and use `ElRef<T>` instead of the dirty DOM call I am doing. It was quite impressive because :
- He never touched Rust before
- He never touched Elm before but he understand just from the counter example the Pattern
- He is not a web developer 
- He is not a Js dude but a Python dude.

So now we have a very light setup to start and have fun with :D


###2 - Take the Rust code from the original tutorial

*My original* `lib.rs` from https://github.com/arn-the-long-beard/wasm-game-of-life/blob/master/src/lib.rs


```Rust

mod utils;

use rand_core::{OsRng, RngCore};
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::console;
#[wasm_bindgen]
extern crate web_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }
    pub fn reset(&mut self) {
        let cells = (0..self.width * self.height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        self.cells = cells;
    }
    pub fn death() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        let cells = (0..width * height).map(|i| Cell::Dead).collect();

        Universe {
            width,
            height,
            cells,
        }
    }
    pub fn random() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;
        let mut key = [0u8; 16];
        OsRng.fill_bytes(&mut key);
        let random_u64 = OsRng.next_u64();
        let cells = (0..width * height)
            .map(|i| {
                if random_u64 % 2 == 0 || random_u64 % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    ///
    /// Toggle a cell on specific coordinates
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next = { self.cells.clone() };

        {
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);

                    let next_cell = match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (Cell::Alive, x) if x < 2 => Cell::Dead,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (Cell::Alive, x) if x > 3 => Cell::Dead,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (Cell::Dead, 3) => Cell::Alive,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    };

                    next[idx] = next_cell;
                }
            }
        }

        self.cells = next;
    }
    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }
    // ...
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
```
We take out the part only about Universe. We merge the 2 implementations and create a new file `universe.rs` 

NB : my original lib.rs contains more than the what you can find on the [repository from the tutorial](https://github.com/rustwasm/wasm_game_of_life) because I worked out few of the nice exercises given by the author.

I did implemented the Random Universe and Ultimate Death. 

I did not succeed to make the Random Universe works on the original game of life :( 

Here are some extra dependencies we need for generating random stuff. Add this under `[dependencies]` in your `Cargo.toml`
```toml
rand = "0.7.3"
rand_core = "0.5.1"
```
I admit I am super lazy. There is surely an easy way to do it by hand without dependencies :D

Now in `/src` 

`universe.rs`
```Rust
use rand_core::{OsRng, RngCore};
use std::fmt;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}
/// Public methods, exported to JavaScript.

impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }
    pub fn reset(&mut self) {
        let cells = (0..self.width * self.height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        self.cells = cells;
    }
    /// Kill all the cells
    pub fn death() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height).map(|i| Cell::Dead).collect();

        Universe {
            width,
            height,
            cells,
        }
    }
    /// Generate random state for cell
    pub fn random() -> Universe {
        let width = 64;
        let height = 64;
        let mut key = [0u8; 16];
        OsRng.fill_bytes(&mut key);

        let cells = (0..width * height)
            .map(|i| {
                if OsRng.next_u64() % 2 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    ///
    /// Toggle a cell on specific coordinates
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn cell_at_index(&self, index: usize) -> Cell {
        self.cells[index]
    }

    pub fn tick(&mut self) {
        let mut next = { self.cells.clone() };

        {
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);

                    let next_cell = match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (Cell::Alive, x) if x < 2 => Cell::Dead,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (Cell::Alive, x) if x > 3 => Cell::Dead,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (Cell::Dead, 3) => Cell::Alive,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    };

                    next[idx] = next_cell;
                }
            }
        }

        self.cells = next;
    }
    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }
    // ...
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}


```
 
We can see major improvements there :
- No need to use `#[wasm_bindgen]`
- No need to use `sys_web`
- No need to use  `log` , `Seed` has one we can use if we want
- No need to use `utils` module . Try to make a normal `panic!` and you see the line where the error appear in your rust code inside the web browser console :) 
- We merged the different `impl` for Universe together
- We can directly unit test `Universe` if we want
- We can directly benchmark `Universe` if we want


Now let's go to the core of the app



###3 - Let's build the core of the app from Js to Rust.

Remember guys that we are translating **[from this repos](https://github.com/arn-the-long-beard/wasm-game-of-life)** which is my work when I followed this nice tutorial https://rustwasm.github.io/docs/book/introduction.html

==> **to Rust/Seed**

My original Js file is ugly because, well, I did not care about making it beautiful, create class or other file. And even on the example repos, the js file is messy :P.

Here is my [original](https://github.com/arn-the-long-beard/wasm-game-of-life/blob/master/www/index.js) `index.js`
```javascript

import { Universe, Cell } from "wasm-game-of-life";
// Import the WebAssembly memory at the top of the file.
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";
const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
let  universe = Universe.new();

// universe.set_height(100);
// universe.set_width(100);
universe.reset();
const width = universe.width();
const height = universe.height();


// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');
let animationId = null;
const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the latest 100 timings.
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // Find the max, min, and mean of our 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

        // Render the statistics.
        this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
    }
};
const renderLoop = () => {
    fps.render();
    let ticks = document.getElementById("ticks").value;
    for (let i = 0; i <  ticks ; i++) {
        universe.tick();
    }
    drawGrid();
    drawCells();

 animationId = requestAnimationFrame(renderLoop);
};


const ultimateDeath = document.getElementById("death");

ultimateDeath.addEventListener("click", event=> {
  universe = Universe.death();
})

// const reset = document.getElementById("reset");
//
// reset.addEventListener("click", event=> {
//     universe = Universe.random();
// })
const isPaused = () => {
    return animationId === null;
};
const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});
const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {

        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {

        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};
const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    // Alive cells.
    ctx.fillStyle = ALIVE_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if (cells[idx] !== Cell.Alive) {
                continue;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

// Dead cells.
    ctx.fillStyle = DEAD_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if (cells[idx] !== Cell.Dead) {
                continue;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};


canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawGrid();
    drawCells();
});



drawGrid();
drawCells();
play();

```

Here is the original `index.html` 

```Html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Hello wasm-pack!</title>
    <style>
        body {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
        }
        #fps {
            white-space: pre;
            font-family: monospace;
        }
    </style>
</head>
<body>
<div id="fps"></div>
<p>Tick settings:</p>

<div>
    <input type="range" id="ticks" name="ticks"
           min="0" max="10">
    <label for="ticks">Ticks</label>
</div>
<button id="reset">Random reset</button>
<button id="death">Ultimate death</button>
<button id="play-pause"></button>
<canvas id="game-of-life-canvas"></canvas>

<noscript>This page contains webassembly and javascript content, please enable javascript in your browser.</noscript>
<script src="./bootstrap.js"></script>
</body>
</html>


```


The content of `index.js ` and the body of `index.html` will be written to pure  Rust.

`Seed` uses a kind of pattern similar to Elm by the way.

What we need to do now 

- Define the content ( the state aka Model) 
- Define the view
- Define how we change stuff ( update)
- Define init

Then let's go !


#### Model ####

We need to :
- Have width and height for canvas 
- Have Cell size
- Have the universe
- Know if we play or not
- Display stuff about fps
- Have a value for range ( I skip this one because I do it the dirty way in this tutorial, link at the end show you how it should be)
- Have Colors for alive and dead
- Have Color for grid

We focus at start on displaying the universe with pause/play and click on cell, so let's keep the `fps` & `range` for later :)

`lib.rs`

```Rust
// `Model` describes our app state.
pub struct Model {
    cell_size: u32,
    grid_color: String,
    dead_color: String,
    alive_color: String,
    universe: Universe,
    pause: bool,
    canvas_height: u32,
    canvas_width: u32,
}

```


Let's init the stuff

```Rust
// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {

    let universe = Universe::new();
    let cell_size = 5;
    let canvas_width = (cell_size + 1) * universe.width() + 1;
    let canvas_height = (cell_size + 1) * universe.height() + 1;

    Model {
        cell_size,
        grid_color: "#CCCCCC".to_string(),
        dead_color: "#FFFFFF".to_string(),
        alive_color: "#000000".to_string(),
        pause: false,
        universe,
        canvas_height,
        canvas_width,
    }
}
```



Let's list what's gonna happen in the app with our messages !

```Rust

// `Msg` describes the different events you can modify state with.
enum Msg {
    /// We need to play the game
    Play,
    /// We need to pause
    Pause,
    /// We need to draw stuff
    Draw,
    /// We need to destroy the universe
    Destroy,
    /// We need to generate a random Universe
    Random,
    /// We need to click on a cell
    CellClick
}
```
This step of writing messages/events remembers me when I wrote actions/action_type in Redux/Ngrx.

It is a good process to force yourself to think about what you are actually doing and what's gonna happen.

Just for fun let's write update with empty match. We will focus on this later.

```Rust
// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Play => {},
        Msg::Pause => {},
        Msg::Draw => {},
        Msg::Destroy => {},
        Msg::Random => {},
        Msg::CellClick => {}
    }
}
```

Let's write the view

```Rust
// `view` describes what to display.
 

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
   section![
     button![
            id!("random"),
            ev(Ev::Click, |_| Msg::Random),
            "Random Reset"
        ],
        button![
            id!("destroy"),
            ev(Ev::Click, |_| Msg::Destroy),
            "Ultimate Death"
        ],
        button![
            id!("play-pause"),
            if model.pause {
                ev(Ev::Click, |_| Msg::Play)
            } else {
                ev(Ev::Click, |_| Msg::Pause)
            },
            if model.pause { "▶" } else { "⏸" }
        ],
        canvas![
            id!("game-of-life-canvas"),
            ev(Ev::Click, |event| {
                let mouse_event: web_sys::MouseEvent = event.unchecked_into();
                Msg::CellClick(mouse_event)
            })
        ],
    ]
}
```

Here there are few things to note :

- As a user of Jetbrain Clion, I do not have any intellisense or strong color inside `Macros` so it is harder to code there.

Users of Visual Studio Code get a bit more lucky there because the intellisense and color work inside :D

- We always need for now to encapsulate the code in one single node for Seed. I choose a section. At the end we will have 2 sections then.

- We can use Rust code there which is an enormous benefit. So event if my intellisense and color code does not appear, all the stuff is backed up by our lovely compiler .D


Now if you refresh the page you should see something like this

![Alt Text](https://dev-to-uploads.s3.amazonaws.com/i/sngqy7v4f4qgrlylo4sq.png)


We have some stuff to do now in the update function.

#### Draw our canvas #### 

As explained at start, this tutorial is one brutal conversion from Js to Rust. I try to not be too much exotic :D.

When we draw, we actually draw cells and draw grid as well.

`lib.rs`
```Rust

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Play => {},
        Msg::Pause => {},
        Msg::Draw => {
            if model.pause {
            } else {
                model.universe.tick();
                draw_grid(model);
                draw_cells(model);           
            }
        },
        Msg::Destroy => {},
        Msg::Random => {},
        Msg::CellClick(event) => {}
    }
}

fn draw_grid(model: &mut Model) {
// equivalent to js const canvas = document.getElementById("game-of-life-canvas");
// could be written let canvas document().get_element_by_id("game-of-life-canvas");
// I used a shortcut here which is fine.
// There is even a better way that I will show you later
    let canvas = canvas("game-of-life-canvas").unwrap(); 
    canvas.set_width(model.canvas_width);
    canvas.set_height(model.canvas_height);
    let ctx = seed::canvas_context_2d(&canvas);
    ctx.begin_path();
    ctx.set_stroke_style(&JsValue::from_str(model.grid_color.as_str()));

    // Vertical lines.
    for i in 0..model.universe.width() {
        ctx.move_to((i * (model.cell_size + 1) + 1).into(), 0.);
        ctx.line_to(
            (i * (model.cell_size + 1) + 1).into(),
            ((model.cell_size + 1) * model.universe.height() + 1).into(),
        );
    }
    // Horizontal lines.
    for j in 0..model.universe.height() {
        ctx.move_to(0., (j * (model.cell_size + 1) + 1).into());
        ctx.line_to(
            ((model.cell_size + 1) * model.universe.width() + 1).into(),
            (j * (model.cell_size + 1) + 1).into(),
        )
    }

    ctx.stroke();
}

fn draw_cells(model: &mut Model) {
    let canvas = canvas("game-of-life-canvas").unwrap();
    let ctx = seed::canvas_context_2d(&canvas);
    ctx.begin_path();

    // Alive cells.
    ctx.set_fill_style(&JsValue::from_str(model.alive_color.as_str()));
    for row in 0..model.universe.height() {
        for col in 0..model.universe.width() {
            let idx = model.universe.get_index(row, col);
            if model.universe.cell_at_index(idx) != Cell::Alive {
                continue;
            }

            ctx.fill_rect(
                (col * (model.cell_size + 1) + 1).into(),
                (row * (model.cell_size + 1) + 1).into(),
                (model.cell_size).into(),
                (model.cell_size).into(),
            );
        }
    }

    // Dead cells.
    ctx.set_fill_style(&JsValue::from_str(model.dead_color.as_str()));
    for row in 0..model.universe.height() {
        for col in 0..model.universe.width() {
            let idx = model.universe.get_index(row, col);
            if model.universe.cell_at_index(idx) != Cell::Dead {
                continue;
            }

            ctx.fill_rect(
                (col * (model.cell_size + 1) + 1).into(),
                (row * (model.cell_size + 1) + 1).into(),
                (model.cell_size).into(),
                (model.cell_size).into(),
            );
        }
    }

    ctx.stroke();
}
```
You need to update the import on top of your file `use crate::universe::{Universe, Cell};`

Well, I just converted the code in lazy way without thinking much.
As explained in comment there are different way to get the canvas from the DOM.
Later I ll show you the **right** way.

NB : wait there are weird stuff there :

- We draw in theory the stuff, but only if we get a message  `Msg::Draw`
- Where is ` animationId = requestAnimationFrame(renderLoop);` ? There is no loop anymore

Héhé, technically we can use `request_animation_frame` , your intellisense will find all the API you have in Javascript as long as you use the snake_case Rust Style,

but you will also see that it has been deprecated. Even if I am lazy, I do not want to use deprecated stuff.

There are a set of some side effect with some standard methods. Seed has been making new tools to fit their pattern and give us more control.

Let's introduce [Orders](https://docs.rs/seed/0.7.0/seed/app/orders/trait.Orders.html)

Looks like Reactive programming to me. We can use them to queue messages in our case.
- start drawing on init
- continue drawing 

`lib.rs`
```Rust 

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Draw);

//... the other stuff from before
}

```

`lib.rs` 
```Rust

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Play => {},
        Msg::Pause => {},
        Msg::Draw => {
            if model.pause {
            } else {
                model.universe.tick();
                draw_grid(model);
                draw_cells(model);
                orders.after_next_render(|_| Msg::Draw);
            }
        },
        Msg::Destroy => {},
        Msg::Random => {},
        Msg::CellClick(event) => {}
    }
}
```

In both cases we just do what the code says. Simple & easy.

Refresh your page ( hot reloading is coming soon) and you should see life moving.

We need some css there that we can put in index.html inside the `<head> INSERT THERE </head> `

```css

    <style>
        section {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
        }
    </style>
```

We need to target section since we used it in the view and voila !

![Alt Text](https://dev-to-uploads.s3.amazonaws.com/i/piem637tuoz6ugwoi9pg.png)

The cells should be moving :D

NB: 
- The border on bottom and right seems to be eaten a bit ( probably because of the conversion I am doing)
- The grid looks like a bit different that the one from the original ( Because same reason as above I think)

I am a **baby** Rustacean, so I did things quick & dirty. My code produces inaccurate values .

Before we go further, let's move the draw function in a file, let's call draw.rs

No code to show this time, you'll fix the import and the stuff by yourself :P


#### Destroy & Generate Random


Just need to call the `universe::death()` & `universe::random()` in the update function and that's it !


#### Play & Pause

You can do it by yourself.

Just make sure to send an other `msg::Draw` when it is playing time :P


#### Click on a cell

Still in lazy mode we just convert he old Js to Rust.


```Rust

/// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Draw => draw::draw(model, orders),
        Msg::Play => {
            model.pause = false;
            orders.after_next_render(|_| Msg::Draw);
        }
        Msg::Pause => model.pause = true,
        Msg::Destroy => model.universe = Universe::death(),
        Msg::Random => model.universe = Universe::random(),
        Msg::CellClick(event) => {
            let canvas = canvas("game-of-life-canvas").unwrap();
            let bounding_rect = canvas.get_bounding_client_rect();

            let scale_x: f64 = f64::from(canvas.width()) / bounding_rect.width();
            let scale_y: f64 = f64::from(canvas.height()) / bounding_rect.height();

            let canvas_left: f64 = (f64::from(event.client_x()) - bounding_rect.left()) * scale_x;
            let canvas_top: f64 = (f64::from(event.client_y()) - bounding_rect.top()) * scale_y;

            let row_pos: f64 = (canvas_top / f64::from(model.cell_size + 1)).floor();
            let col_pos: f64 = (canvas_left / f64::from(model.cell_size + 1)).floor();

            let row: u32 = cmp::min(row_pos as u32, model.universe.height() - 1);
            let col: u32 = cmp::min(col_pos as u32, model.universe.width() - 1);

            model.universe.toggle_cell(row, col);
        }
    }
}
```

Now you can try to fix the import but you end up with an error

```
error[E0599]: no method named `get_bounding_client_rect` found for struct `seed::prelude::web_sys::HtmlCanvasElement` in the current scope
  --> src/lib.rs:78:40
   |
78 |             let bounding_rect = canvas.get_bounding_client_rect();
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `seed::prelude::web_sys::HtmlCanvasElement

```

Well ... what is happening there...?

- Strangely the compiler doesn't find this method from canvas
- You do not see the method there neither https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlCanvasElement.html

I tell you why :D

- You will find the method there https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html#method.get_bounding_client_rect on Element 
- Rust does not use inheritance because well, this concept has too many issues I guess and we use `traits` in Rust ( Spoiler Alert : They are awesome !!! )
- You actually need to activate features to use for web_sys as mention on the page I linked just before.

Remember that Rust is made for being performant, so it let you in this context choose the stuff that you want to compile for use.

So your binaries are smaller, your IDE is also quicker to search for stuff, your web browser will not make your PC out of RAM because of x thousands of package/files ( cf node_modules ) and so many more good stuff to say about this strategy.

`Cargo.toml`
```Toml
#stuff from before still there
[dependencies.web-sys]
version = "0.3"
features=[ "DomRect", "Element"]
```


Now it will work. The `cargo make watch` should have recompiled for you :) and you can click on a cell.

Let's move the code to click on cell to `draw.rs`

I renamed draw.rs to canvas.rs.

I let you make the necessary change there :P

Regarding the import, I have 2 choices for `cmp`
- `use seed::prelude::wasm_bindgen::__rt::std::cmp;`
- `use std::cmp;`


I do not know the difference between them really. I need to ask to the other more experienced `Seed` people. Are we gardeners ? I have so many useless jokes :( .



#### Choose how many ticks per frame

Okay there, from the original tutorial, I did implemented the range to choose how many ticks per frame rendering we want.

Let's start with the view because it is the easier to convert ( maybe).
I needed to check some examples from [Seed repo](https://github.com/seed-rs/seed/tree/master/examples)  because macro stuff is not easy without intellisense for me.


`lib.rs`
```Rust
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    section![
        p!["Ticks settings :"],
        div![
            input![
                id!("ticks"),
                1,
                attrs! {
                    At::Name => "ticks",
                    At::Type => "range",
                    At::Min =>"1",
                    At::Max =>"10"
                }
            ],
            label![attrs! { At::For => "ticks"}, "ticks"]
        ],
        button![
            id!("random"),
            ev(Ev::Click, |_| Msg::Random),
            "Random Reset"
        ],
        button![
            id!("destroy"),
            ev(Ev::Click, |_| Msg::Destroy),
            "Ultimate Death"
        ],
        button![
            id!("play-pause"),
            if model.pause {
                ev(Ev::Click, |_| Msg::Play)
            } else {
                ev(Ev::Click, |_| Msg::Pause)
            },
            if model.pause { "▶" } else { "⏸" }
        ],
        canvas![
            id!("game-of-life-canvas"),
            ev(Ev::Click, |event| {
                let mouse_event: web_sys::MouseEvent = event.unchecked_into();
                Msg::CellClick(mouse_event)
            })
        ],
    ]
}

```


There are a better way to do the range and the state that goes within. I will show the recommended way later. Let's make it quick & dirt for now.

`lib.rs`
```Rust
// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Play => {
            model.pause = false;
            orders.after_next_render(|_| Msg::Draw);
        }
        Msg::Pause => model.pause = true,
        Msg::Draw => {
            if model.pause {
            } else {
                let tick_input = document().get_element_by_id("ticks").unwrap();
                let tick_frequency = get_value(tick_input.as_ref()).unwrap();
                let tick_number = tick_frequency.parse::<u32>().unwrap();

                for i in 0..tick_number {
                    model.universe.tick();
                }
                canvas::draw_grid(model);
                canvas::draw_cells(model);
                orders.after_next_render(|_| Msg::Draw);
            }
        }
        Msg::Destroy => {
            model.universe = Universe::death();
        }
        Msg::Random => {
            model.universe = Universe::random();
        }
        Msg::CellClick(event) => {
            let position = canvas::find_cell_from_click(model, event);

            model.universe.toggle_cell(position.0, position.1);
        }
    }
}
```

The part to take out the value is not as small as in Js because of Rust :
- No inheritance
- Need to unwrap stuff because we have result and possibility to handle errors
- Need to cast String to u32

Even if we have a little bit of more to do there, we have also more control.
Remember that this way I wrote it is not the best nor the recommanded way. We can improve this a lot.

You can change the range and see how it goes. In my case I felt that the seeded-game-of-life perform ( or tiny better if many tabs opened / the same ) at least in standard `build` / `--debug` than the old.

#### Display FPS

Let's now display the number of Frames Per Second as shown in the original tutorial.

Jut add ` div![id!["fps"]],` in `fn view()` body before `p!["Ticks settings :"],`

Now, in the original Javascript file we had a `Class` like this for me

```javascript
const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the latest 100 timings.
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // Find the max, min, and mean of our 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

        // Render the statistics.
        this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
    }
};

```

The variable was actually used only one time in `index.js` in `renderLoop()`

```javascript
const renderLoop = () => {
    fps.render();
    let ticks = document.getElementById("ticks").value;
    for (let i = 0; i <  ticks ; i++) {
        universe.tick();
    }
    drawGrid();
    drawCells();

 animationId = requestAnimationFrame(renderLoop);
};

```

Then let's do the same. Let's use fps inside `Msg::Draw` and instead of a `Class`, let's use a `struct` and `impl`.

Create `fps.rs` and make some better code.

Since we want to expose fps in `lib.rs`, let's try to minimize calls to the Web Browser api and focus on calculation.

We can change the `render` method and call it `calculate` to return the statistics.

`fps.rs`

```Rust
use seed::window;
use std::cmp;

pub struct FpsCounter {
    frames: Vec<f64>,
    last_frame_timestamp: f64,
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter {
            frames: Vec::new(),
            last_frame_timestamp: window().performance().unwrap().now(), //should have it outside so it would be  more beautiful
        }
    }
    /// Ex- Render function
    /// Same as original in JS , I moved most of call to DOM outside to make it "cleaner"
    /// Maybe I could also have passed time as an argument to make it even better
    /// If we removed calls to window() we could make unit test and benchmark    
        pub fn calculate(&mut self) -> FpsStatistic {
        let now = window().performance().unwrap().now();
        let delta = now - self.last_frame_timestamp;
        self.last_frame_timestamp = now;

        let fps = 1. / delta * 1000.;

        self.frames.push(fps);

        if self.frames.len() > 100 {
            self.frames.remove(0);
        }

        let mut min = i32::MAX;
        let mut max = i32::MIN;

        let mut sum: f64 = 0.;

        for i in 0..self.frames.len() {
            sum = sum + self.frames[i] as f64;

            min = cmp::min(self.frames[i] as i32, min);

            max = cmp::max(self.frames[i] as i32, max);
        }
        let mean = sum / self.frames.len() as f64;

        FpsStatistic {
            fps: fps as u32,
            mean: mean as u32,
            min,
            max,
        }
    }
}

pub struct FpsStatistic {
    pub fps: u32,
    pub mean: u32,
    pub min: i32,
    pub max: i32,
}

```


To have access to the `fps` at anytime let's add it to the state :)

`lib.rs`
```Rust

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Draw);

    let universe = Universe::new();
    let cell_size = 5;
    let canvas_width = (cell_size + 1) * universe.width() + 1;
    let canvas_height = (cell_size + 1) * universe.height() + 1;

    Model {
        cell_size,
        grid_color: "#CCCCCC".to_string(),
        dead_color: "#FFFFFF".to_string(),
        alive_color: "#000000".to_string(),
        pause: false,
        universe,
        canvas_height,
        canvas_width,
        fps:FpsCounter::new()
    }
}

// `Model` describes our app state.
pub struct Model {
    cell_size: u32,
    grid_color: String,
    dead_color: String,
    alive_color: String,
    universe: Universe,
    pause: bool,
    canvas_height: u32,
    canvas_width: u32,
    fps: FpsCounter,
}
```

Then we can add this code in `Msg::Draw` just before the code to get value for tick frequencies

`lib.rs`
```Rust
 let fps = document().get_element_by_id("fps").unwrap();
                let stats = model.fps.calculate();
                let text = format!(
                    "\
                Frames per Second:
         latest = {:?}
avg of last 100 = {:?}
min of last 100 = {:?}
max of last 100 = {:?}
                \
                ",
                    stats.fps, stats.mean, stats.min, stats.max
                );

                fps.set_text_content(Some(text.as_str()));

```
We need some css , let's use the same from he original tutorial, so inside the `<style>` in `index.html`

```css
      #fps {
            white-space: pre;
            font-family: monospace;
        }
```


You should have something nice now :)

![Alt Text](https://dev-to-uploads.s3.amazonaws.com/i/utqsyn0q27u0ioq7gwqh.png)

Well Let's resume few Pros & Cons from what we have compare to the original source code :

**Benefits :**
- One language for everything <3
- Only few dependencies
- You become productive very quickly
- Easier to split and structure the code => the code looks so much better visually
- More control about what is happening ( more typing/syntax/tools available/IDE support )
- We get state management for free without extra library/logic  : It's included & Easy !
- The compiler checks everything, including the Macros/html-like stuff.
- No nodes_modules & no javascript dependencies
- My web browser does not eat my RAM when I open the web browser dev tools
- Choose what you compile when you need in `Cargo.toml`
- & many more positive statements <3

**Cost**
- You need to at least be baby Rustacean to do stuff or having somebody near by.
- The IDE intellisense does not work everywhere ( but in Js it is also failing by catching actually too much and flooding the IDE )
- The Typing does not always come up from web_sys stuff/Seed
- I cannot see the content of package.wasm in the web browser, but I can in the original tutorial
- Not sure I can debug at run time
- Please add some pain that you feel in your comment and I will add it there :)
- We have more generated Js inside our `package.js` 
 
### 4 - Let's try test & benchmark

#### Let's test that thing


Let's add the following in `Cargo.toml`

`Cargo.toml`
```Toml
[dev-dependencies]
wasm-bindgen-test = "0.3"
```

Let's make a test file

`./tests/web.rs`

```Rust

//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate seeded_game_of_life;
use seeded_game_of_life::universe::Universe;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_spaceship();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_spaceship();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}


```

`wasm-pack test --firefox` and then click on the address and you should see the test in firefox

You can do the same if you have Chrome on your computer of course :)

At the moment, since today 29/07/2020 

`wasm-pack test --firefox --headless` seems to fail. I made an issue about it there https://github.com/rustwasm/wasm-bindgen/issues/2261


Anyway testing is easy and we can also make standard tests as well

`tests/universe.rs`
```Rust
#[cfg(test)]
mod tests {
    use seeded_game_of_life::universe::Universe;

    #[test]
    fn big_bang_works() {
       let universe = Universe::new();


        assert_eq!(universe.width(),64);
        assert_eq!(universe.height(),64);
    }
}
```

#### Let's bench it !

`rustup default nightly`

We need the `Darkside` if we want to use some features for benchmarking.

`./benches/bench.rs`
```Rust
#![feature(test)]
extern crate seeded_game_of_life;
extern crate test;

use seeded_game_of_life::universe::Universe;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = Universe::new();

    b.iter(|| {
        universe.tick();
    });
}

```

You should have something like this

```
Finished bench [optimized] target(s) in 5.83s
     Running target/release/deps/seeded_game_of_life-e5bb887774c98fd8

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/release/deps/bench-a3412253977e35f0

running 1 test
test universe_ticks ... bench:      38,410 ns/iter (+/- 29,201)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out

```

NB : There are few differences here compare to the [original tutorial](https://rustwasm.github.io/docs/book/game-of-life/time-profiling.html) :
```
We also have to comment out all the #[wasm_bindgen] annotations, and the "cdylib" bits from Cargo.toml or else building native code will fail and have link errors.

```
- No need to care about  `#[wasm_bindgen]`
- No need to remove  `"cdylib"`



Probably because of the `Seed` magic, our rust code does not need the tag, so it is easier :D

Now if we compare to a javascript version of the code base :

- No need to find a library like jest/mocha or other to make test because rust has everything we need :D
- Tags and tools are there to target which kind of test we do
- Benchmark code as native gives us easy overview of performance ( I never had such opportunity in js/ts) , can use `perf` on Linux

To resume, test & benchmark were already easy because of Rust, now it is even more easier since there is only Rust <3 . Thank you Seed !

###5 - How to improve performances

Of course, there are many ways to improve the performances. 

- Take care more of what we are doing in the code:
    - correct type ? 
    - conversion ? 
    - call to DOM ?
    - follow the doc :D
    
- Making a specific build for release:
   - small compiled code strategy
   - faster runtime strategy
   
   

Now in the code we have, we could actually remove every code like `document()` or `canvas()` and use `Ref` and access their state in the `M̀odel`

We can also use options for building as mentioned in the original tutorial. Let's try it!

`Cargo.toml`
```Toml
[profile.release]
lto = true
opt-level = 'z'
codegen-units = 1

[package.metadata.wasm-pack.profile.release]
wasm-opt = ['-O3']

```

You can now run `cargo make build_release`

After refreshing your web browser, you will see that you are at 60 fps always with this size of grid and max ticks.
For me going from 40fps --ish to 60 is a nice improvement :D

You can try to compare the different generated packages if you want with the help of the [tutorial](https://rustwasm.github.io/docs/book/game-of-life/time-profiling.html)


###6 - Conclusion


There is much to say :D and I already talked/wrote sooo much :P

We just have shown together with this post that we can use `Rust` only ( with still html +css of course ) to code **web** `front_end` work.

From a personal point of view, I think of `Rust` as a language like **Latin** or **Icelandic**:

- It can be very short and effective and sometimes very long because of the possibilities to add much context and details that enhance the sens of it.
- Gives more control and nuances on what we need to say, write and communicate.

Rust is **rich** and **powerful** with a `compiler` who moves so many errors and bugs from `run_time` to `compile_time`. 

- It asks us to be better programmers & produce better code
- It secures the code we produce
- It really does indicate/encourage good practice and consistency ( Consistency in Javscript/Typescript even inside the same **dev team** is very hard to accomplish)
- It makes you **love** and **hate** the compiler at the same time **De l'amour à la haine, il n'y a qu'un pas** :D


From a 5 years experience web-programmer that I am :

I really did enjoy my time with React/Redux. Especially redux, because it gives so much control and predictability over pure javascript. Same about Ngrx on Angular.
So I got a good time on React & Angular, but I also know all the pains, the endless maintenance and flood of dependencies to take care of in daily work.
They are part of my experience and I do not have any regret about them. 
But now I have some friends who want to start Web stuff and the layer of complexity in Js/Ts/React/Angular∕vue environment are a huge gap to fight.

All the safety and paradigm ( & soo much more ) made by the front-end frameworks like React or Angular to make web development possible are now more accessible with Rust with much lighter abstraction.


Here are few of the many benefits I see by using **Rust/Seed**

- One language for all logic
- Less dependencies
- Code robust and easy to predict & trust
- We have state_management included
- Visually more organized & structured code
- Higher productivity
- Compiler that helps us to build Robust/Solid Code
- Less time to debug stuff
- Much more predictability
- Easy to maintain
- One language for front_end & back_end
- More control on performances
- Probably better performances in the future also as well


We also have some challenges of course

- IDEs still do not support all the disruptive concepts from Rust
- Seed is not ready for production
- We have still some generated js
- No extension to watch the state at runtime in your web browser like our **lovely** `redux-dev-tools` can do 
- Not sure we can debug the code at runtime neither , but do we need it ?
- No libraries available for specific needs, you will need to do it by yourself ( no UI library for now like Google Material or Bootstrap I think)


Here is the [source](https://github.com/arn-the-long-beard/seeded_game_of_life) code for this post.

Some parts of the code are gonna be updated and improved as I explained. I will do a new branch for that and link it below.

Rust + Seed made me happy to code again. It really make me feel to be part of a great adventure, meet new people, get new skills and discover new opportunities ! There is such much to build :)
 
 
Please guys, **try** & **have fun** & **comment** !!!













