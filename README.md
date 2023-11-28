# Site checker

See the web site of the [Site Checker project](https://site-checker.code-maven.com/)

## Development / Contributions

```
git clone git@github.com:szabgab/site-checker.rs.git
cd site-checker
```

Optionally install [pre-commit](https://pre-commit.com/) and then run `pre-commit install` to configure it in the current repository.


## Release and publish

* Update version number in Cargo.toml
* `git commit`
* `cargo publish`
* git tag using the same version number:   (`git tag -a v0.2.2 -m "publish version v0.2.2"`)
* `git push --tags`