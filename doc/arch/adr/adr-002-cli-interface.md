# Style of the CLI interface

- **Status**: Proposed
- **Date**: 2025-11-02
- **Decision-maker**: Aleksandr Ganiukhin


## Context and Problem Statement

It is requires to select user-friendly and consistent style for the CLI interface of the application.
- The CLI interface should be intuitive and easy to use for end users.
- The CLI interface should follow best practices and conventions for command-line applications.


## Considered Options

* Flag-based interface
  * Example
    * `cheatsheet --add --title "Git Commit" --command "git commit -m 'message'" --description "Commit changes to git repository"`
  * Conclusion:
    * **Discarded** since "--" options shall not change the whole behavior, but just slightly modify it: filter, format, etc
* Subcommand-based interface
  * Example
    * `cheatsheet add --title "Git Commit" --description "Commit changes to git repository" "git commit -m 'message'"`
    * `cheatsheet remove --title "Git Commit"`
    * `cheatsheet list`
    * `cheatsheet search git`
  * Conclusion:
    * **Accepted** since it clearly separates different actions (add, remove, list, search) and is widely used in CLI applications.
    * **Notes**
      * In future we may consider positional arguments in addition to the named ones. *NOT instead of, to not break backward compatibility.*
* Fuzzy matching commands
  * Like `cheatsheet <arg>`, if `arg` matches command, then it is command, otherwise it is search term.
  * Conclusion:
    * **Discarded** since it may lead to ambiguity and confusion for users.
* Use dedicated binary for search and another for managing cheat sheets
  * Example
    * `cheatsheet <search term>`
    * `cheatsheetctl add ...`
  * Conclusion:
    * **Discarded** since it complicates the user experience and increases maintenance overhead.


## Decision Outcome

- There will be a single binary `cheatsheet` for all operations.
- We will ALWAYS use Subcommand-based interface for the CLI.
  - It is expected that user will create aliases for frequently used commands, e.g. `alias css='cheatsheet search'`.
  - As an option, aliases may be provided out-of-the-box.
- Complex commands (add, remove) (where multiple parameters are required) will use named arguments (flags) to improve clarity.
- Simple and frequent commands (list, search) will use positional arguments for convenience.
- Flags and options MUST NOT change the fundamental behavior of the command, but only modify it (e.g., filtering, formatting, etc).
- The CLI options are documented only in the `--help` section, as a single source of truth.
  - This means that `--help` is the primary, while does not forbid having additional documentation elsewhere (e.g. default cheatsheet config).
- In addition, as a sort of self-documentation, self-advertisement, default cheatsheet config will contain cheatsheet for the cheatsheet CLI itself 😉
  - It is expected that every time CLI is changed, the cheatsheet for it is updated as well.


### Consequences

#### Positive consequences

* TBD
* ...


#### Negative consequences

* TBD
* …
