# gh-bell

Minimal app to check your unread Github notifications

## Building:

Just plain old `cargo build` or `cargo run` does the trick, the crate is not released or anything fancy.

## Usage:

The application takes the Github Personal Access Token from the `GH_BELL_PAT` environment variable. You can
get your PAT from this URL: [https://github.com/settings/tokens](https://github.com/settings/tokens).
It only requires the notification scope.

Note: The notification endpoint authorization only works with the classic tokens as of yet (2023-01-05).

### Option 1

```
$ export GH_BELL_PAT=<token>
$ cargo run
```

### Option 2

```
$ GH_BELL_PAT=<token> cargo run
```
