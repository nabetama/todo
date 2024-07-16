# todo

A simple command-line todo that
clone of https://github.com/mattn/todo written in Rust.

## Usage

### List todo

```
$ todo list
☐ 001: Play with cats
☑ 002: Stroke a cat
☐ 003: Feeding the Cat
```

### Add new todo

```
$ todo add Sleeping with cats
Task added: Sleeping with cats
$ todo list
☐ 001: Play with cats
☑ 002: Stroke a cat
☐ 003: Feeding the Cat
☐ 004: Sleeping with cats
```

### Update todo

```
$ todo update 2 Talking with Cats
Task 002: updated: Talking with Cats
$ todo list
☐ 001: Play with cats
☑ 002: Talking with Cats
☐ 003: Feeding the Cat
☐ 004: Sleeping with cats
```

### Delete todo

```
$ todo delete 1 3
Task 001 deleted: Play with cats
Task 003 deleted: Feeding the Cat
$ todo list
☑ 001: Talking with Cats
☐ 002: Sleeping with cats
```

### Done todo

```
$ todo done 2
Task 002 marked as done: Sleeping with cats
$ todo list
☑ 001: Talking with Cats
☑ 002: Sleeping with cats
```

### Undone todo

```
$ todo undone 2
Task 2 marked as undone: Sleeping with cats
$ todo list
☑ 001: Talking with Cats
☐ 002: Sleeping with cats
```

### Sort todo

```
$ todo list
☐ 001: Sleeping with cats
☑ 002: Feeding the cat
☐ 003: Stroke a cat
$ todo sort
$ todo list
☑ 001: Feeding the cat
☐ 002: Sleeping with cats
☐ 003: Stroke a cat
```

### Clean all completed todo

```
$ todo list
☑ 001: Talking with Cats
☐ 002: Sleeping with cats
☑ 003: Play with cats
$ todo clean
Tasks cleaned
$ todo list
☐ 001: Sleeping with cats
```

## Installation

```sh
$ cargo install --git https://github.com/nabetama/todo
$ type -a todo
todo is /Users/nabetama/.cargo/bin/todo
```

or build your own.

```sh
$ git clone git@github.com:nabetama/todo.git
$ cargo build --release
```

## LICENSE

Apache License Version 2.0

## Author

Mao Nabeta(@nabetama)
