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

### Clean all completed todo

```
$ todo list
☐ 001: Play with cats
☑ 002: Stroke a cat
☑ 003: Feeding the cat
☐ 004: Sleeping with cats
$ todo clean
Tasks cleaned

$ todo list
☐ 001: Play with cats
☐ 002: Sleeping with cats
```
