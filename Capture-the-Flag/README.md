# Capture the Flag Challenge

Welcome to the Capture the Flag Challenge!

The main goal of this CTF is to have fun and hack things!

There are 5 levels available, and the difficulty increases with each level. However, this may vary for each individual. Each level contains one or more secrets, which are necessary to obtain the secrets in the subsequent levels. Therefore, you won’t be able to jump straight to level 5.

Some levels contain `src-fragments`, which should help you spot bugs in the program. For those levels without fragments, the challenge should not be too difficult!

Good luck, and have fun!


## How to execute

> [!CAUTION]
> Do not build the projects. The projects are pre-build containing the Secrets within. In order to play, run
> ```bash
> anchor test --skip-build
> ```
>
> - Do not try to build src-fragments, as these are only examples which do not contain secrets in the source code!


> [!CAUTION]
>
> If you encounter panick during the `anchor test --skip-build` command, make sure that you have `Solana-CLI 1.18.18` installed!
