### Commit Messages

I use a [conventional](https://github.com/ajoslin/conventional-changelog/blob/a5505865ff3dd710cf757f50530e73ef0ca641da/conventions/angular.md) changelog format so I can update my changelog automatically using [clog](https://github.com/clog-tool/clog-cli)

 * Please format your commit subject line using the following format: `TYPE(COMPONENT): MESSAGE` where `TYPE` is one of the following:
    - `feat` - A new feature of an existing API
    - `perf` - A performance improvement
    - `fix` - A bug fix
    - `docs` - Changes to documentation only
    - `test` - Changes to the testing framework or tests only
    - `refactor` - Code functionality doesn't change, but underlying structure may
    - `style` - Stylistic changes only, no functionality changes
    - `wip` - A work in progress commit (Should typically be `git rebase`'ed away)
    - `chore` - Catch all or things that have to do with the build system, etc
 * The `COMPONENT` is optional, and may be a single file, directory, or logical component. Parenthesis can be omitted if you are opting not to use the `COMPONENT`.
