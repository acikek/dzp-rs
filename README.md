<div align="center">
  <br/>
  <p>
    <img src="https://imgur.com/GZwNotP.png" width="400" alt="dzp"/></a>
  </p>
</div>

## About

DeniZip (dzp), the Denizen Project Manager, is a command-line tool designed to complement [DenizenScript](https://denizenscript.com) projects.

`dzp-rs` is a rewrite of the [original](https://npmjs.com/package/dzp) in Rust.

### Features

- Intuitive interface powered by [clap](https://github.com/clap-rs/clap)
- Customizable project styles
- Detailed script analysis
- Efficient, lightweight

## Setup

Install with [cargo](https://crates.io/):
```sh
cargo install dzp
```

## Example

Create a new directory for your project. 
```sh
mkdir fiery && cd fiery
```

Run `dzp new fiery`. This will initialize a project named "fiery" in the current directory. It will prompt you for some information about the project, such as the name, description, and authors. You can skip each prompt by pressing enter.

```sh
Description: Set anything on fire
Authors: cool_guy78
Version: 0.1.0
License: MIT
Homepage: https://github.com/cool_guy78/fiery
Repository: https://github.com/cool_guy78/fiery
```

Without the `--style` option, you'll see the following tree appear:

```txt
.dzp/
├─ project
src/
├─ data/
├─ main/
│  ├─ fiery.dsc
├─ util/
.gitignore
LICENSE
README.md
```

The style you choose determines what files are created. 
> If you're going for a one-file project, consider `--style single`.

Project data resides in the `.dzp` directory, and the Denizen files are pre-fitted with boilerplate code.

To view all the commands and their usage, run `dzp help`.

## License

MIT © 2021 Skye P.