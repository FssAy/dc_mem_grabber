# dc_mem_grabber [POC]
Discord token grabber, but it grabs tokens from the processes' memory.

This is just a proof of concept, I have no intentions for further development of this project.

## Usage
x64
```commandline
cargo run --release
```

x86
```commandline
cargo run --release --target=i686-pc-windows-msvc
```

## How does it work?
First it finds all Discord pids *(process' unique identifiers)*. <br>
Including **Discord**, **DiscordCanary** and **DiscordPTB** (last one not tested).

Then it reads memory of every single process and dumps tailing 1/3 amount of data *(We don't need that 2/3)* into `Vector` and `File`.

Finally, our program finds tokens thanks to simple `String::find`. <br>
Regexes are too slow.

Getting **mfa** tokens is straightforward, just look for `mfa.` substring and get the `84` characters after.

Issue appears only with **normal tokens** as they don't have any characteristic prefixes or suffixes. <br>
But fortunately first part of the token is user's ID encoded in base64, so all we need to do is to find the user's ID and encode it. <br>
That can be done by searching for parts like `"user":{"id":"` and grabbing the `18` characters after this substring.

## Disclaimer
Of course, it's an unstable solution. We can get invalid tokens, or non at all. <br>
