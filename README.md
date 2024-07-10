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

### Update todo

```
$ todo list
☑ 001 Play with cats
☐ 002 Stroke a cat
☑ 003 Feeding the cat
☐ 004 Sleeping with cats

$ todo update 1 Talking with Cats
Task 001 updated: Talking with Cats

$ todo list
☑ 001 Talking with Cats
☐ 002 Stroke a cat
☑ 003 Feeding the cat
☐ 004 Sleeping with cats
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
