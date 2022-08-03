# Friendly errors

> Note, this is still a work in progress and is very experimental.

Awesome error messages should be a core part of the experience of using any language. Us programmers hate debugging, so let's make it as easy as possible to debug errors.

## Guiding principles

Error messages should be helpful. To that end, they should do five things:

1. Be clear and concise.
2. Help the user understand what is wrong
3. Give the user suggestions for how to fix the problem
4. Help the user prevent similar issues in the future
5. If applicable, link to the docs or other relevant information

This allows users to quickly understand the problem and hopefully fix it in just a few seconds.

## Formatting

Here's the general format for error messages:

```txt
--- Error(E1234): name of the error -------------------------------------------

text explaining the error

    filepath:line:column
    |
 72 | function hello_world() {
    |          ^^^^^^^^^^^

  --> try changing "hello_world" to "helloWorld"

Explanation. Lorem ipsum dolor sit amet, consectetur adipiscing elit.

To learn more, read the docs at https://docs.example.com/
```

Let's break it down.

1. Headline with the general error type, with an error code. This not only allows users to quickly identify the error, but also helps categorize them and let other people help debug the error. The horizontal bar helps separate successive errors.
2. Text explaining the error in plain English. This should be clear and concise, but it can be as long as necessary. In general, it should be as short as possible but no shorter.
3. Link to the file path, line number, and column number. This lets users know exactly where the error is occurring. Plus, with this format, editors like VS Code can jump to the error location when users click on that "link".
4. Code snippet with line numbers. If needed, you can add more context. Though most importantly, carrots `^` should be used to highlight the exact error location.
5. If possible, a suggestion on how to fix the error. This is optional, but it's a good idea to include it. This is especially useful if the error is a quick and easy fix.
6. Explanation with more detail. If this is a common error because someone used syntax from another language (e.g., coming from JavaScript), explain the difference with the other language here. If applicable, also give examples. For instance, if the error is about importing code, give a few examples on how to import code.
7. Link to the docs or other relevant information if available.

## Usage

To get started, just add the following to your Cargo.toml file:

```toml
[dependencies]
friendly-errors = "0.1"
```

Then, you can build up your error message. Every item in this crate uses the builder pattern:

```rust
let error_message = FriendlyError::new()
    .title("variable is undefined")
    .code("E1234")
    .add_code_snippet(
        FriendlyCodeSnippet::new()
            .filepath("src/main.rs")
            .contents("let x = foo;")
            .line(1)
            .index_start(8)
            .index_end(10)
            .text("let x = foo;")
            .caption("foo is not defined")
    )
    .add_suggestion("Try defining foo before using it. All variables must be defined before they're used.")
    .doc_url("https://github.com/Nick-Mazuk/friendly-errors")
```

This will produce the following error message:

```txt
--- Error(E1234): variable is undefined ---------------------------------------

    src/main.rs:1:8
    |
 1  | let x = foo;
    |         ^^^

  --> foo is not defined.

Try defining foo before using it. All variables must be defined before they're
used.

To learn more, read the docs at https://github.com/Nick-Mazuk/friendly-errors
```
