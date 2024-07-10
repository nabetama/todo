# todo

A simple command-line todo list written in Rust.

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
```

### Clean all completed todo

```
$ todo list
☐ 001 Play with cats
☑ 002 Stroke a cat
☑ 003 Feeding the cat
☐ 004 Sleeping with cats

$ todo clean
Tasks cleaned

$ todo list
☐ 001 Play with cats
☐ 002 Sleeping with cats
```
