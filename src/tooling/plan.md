# making tools

## planned structure

```bash
project-name/
  .git/
  main.rl
  rl.toml
  .gitignore
```

### `rl.toml`

```toml
[project]
name = "project-name"
version = "v0.0.1"
entry = "main.rl"
```

### main.rl

```rl
get std::display::println

fn main() {
  println("hello world")
}
```

`.gitignore` will be empty for now

`.git/` should be initialized via command in project folder

## dev command

reads `entry` and runs the file
