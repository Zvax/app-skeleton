# Goal

This is a skeleton of a desktop application using boscop/webview along with SvelteJS to create a
simple Hello world executable with the Rust <-> JavaScript wiring sorted out.

The scope is currently limited to the critical pieces of said wiring and webview creation.
Technically one should be able to clone and install this repository and start
creating commands to exchange information between client and rust as well as these commands handlers,
and link that to sveltejs' components without having to play with the actual creation of the webview.

For persistence it's a bit harder, it also implies writing the sql and use the basic migration tool
in binaries to manually update or downgrade the versions.

# Installation & Usage

## External Dependencies

Apart from the Rust toolchain, one must have npm installed and visible from the shell that will be compiling,
as the compilation process uses `npm run build` internally.

## Hardcoded Values

There are currently two different strings that are hardcoded and represent local computer values:
- the absolute path towards the client folder (app-template/client)
- the connection string of the sqlite database (one in main.rs the other in bin/migration.rs)

## After cloning:

1. One must run `npm install` inside the client directory.
2. One must then either use the migration binaries (or anything they want) to set up the database schema with
   `cargo run --bin migration run 1.0`.
   To use the migration binary one must keep in mind that the db path is hardcoded there as well, meaning these are 
   two places to keep in sync with the actual sqlite db one is using.
3. One must write the absolute path to their client folder in main.rs, that ought to be known dynamically
   during compilation at some time.
4. It is then possible (allegedly) to run `cargo run` to launch the default webview with a few basic widgets.

## Rust Toolchain

I have only been able to compile using nightly-msvc to run this, haven't been successful with either gnu or stable toolchains.

# General Explanations

explain the weird trick with invoke = External.invoke and also that I'm not even sure it will exist forever or supported

explain about the manual set_something that will be transformed in auto-generated commands at some points

The set_something's need to be defined on the window with `window.set_something = n => local_variable = n`.
They can be defined in any svelte component. That allows us to use set_things in the rust js executing environment,
that can only see global scope and not the local values inside Svelte.

I've not been able to make the invoke command work when including the string from an index.html file.. so
I keep it directly as the first argument to `format!` since it's not really a problem if it stays that small.

# Todos

you must change the hardcoded path in main to your own, I don't know how to get that dynamically at compilation time

also the code in client is a bit weird because it's called public which is idiomatic of the web but that's changeable as one goes along their frontend app

how does one creates a menu in this context? is it even possible? Borderless? hovering over other applications?

So far we must manually create all svelte components, setters and getters for every widget we'd want to create.
I have a branch with a templating system dynamically creating entity views in Svelte from toml entity descriptions
before bundling them,
but the process of writing svelte templates that are parsed through `format!` is fairly the worse developer experience
I've ever had the displeasure of working with because of the pervasive doubly escaped `{{` screwing the syntax
highlighter around since there's `{` everywhere in javascript.
So I'm not sure how much of that is a good idea considering I won't write a special highlighter, also it kinda smells
that I'd even want to do that, writing svelte code, even destined to be templated [sic], should be written in svelte (?).

use cargo-make

# Dreaming

It is amusing to this one that the link in the svelte's default splash screen opens the link in the webview.
The possibilities are infinite! Why do we even use browsers this is the future :
portable applications capable of interacting with the old web as well as presenting content without the cruft of browsers
like unhandly urls.
