# Site checker

Web site of the [Site Checker](https://site-checker.code-maven.com/)

* Given a hostname of a website

* check if all the pages are valid HTML
* check if all the pages have (unique?) title
* check the description meta field
* check the keywords meta field


* check if all the internal links are valid
* check if the robots.txt includes link to sitemap.xml
* check if the sitemap.xml lists all the pages?
* check internal link density

## Use

```
git clone git@github.com:szabgab/site-checker.rs.git
cd site-checker
cargo run -- --host https://rust.code-maven.com/
```

This will create a file called `report.json` and another file called `report.html`.


## Development / Contributions

```
git clone git@github.com:szabgab/site-checker.rs.git
cd site-checker
```

Optionally install [pre-commit](https://pre-commit.com/) and then run `pre-commit install` to configure it in the current repository.

