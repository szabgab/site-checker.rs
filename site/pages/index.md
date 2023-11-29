---
title: Site Checker
timestamp: 2023-11-26T09:10:01
description: Checking web sites
---


Given a hostname of a website

* Check if there is a robots.txt file
* Check the title and the description of the main page.

## Planned features:

* check if all the pages are valid HTML
* check if all the pages have (unique?) title
* check the description meta field
* check the keywords meta field


* check if all the internal links are valid
* check if the robots.txt includes link to sitemap.xml
* check if the sitemap.xml lists all the pages?
* check internal link density


## Install and Run

### Run from cloned repository (assuming you have Rust installed)

```
git clone git@github.com:szabgab/site-checker.rs.git
cd site-checker
cargo run -- --pages 5 --host https://rust.code-maven.com/
```

This will create a file called `report.json` and another file called `report.html`.


### Install (assuming you have Rust installed)

```
cargo install --git https://github.com/szabgab/site-checker.rs
site-checker --host https://rust.code-maven.com/
```

## Install if you don't have Rust installed

We don't yet have binary distribution, but let us know which Operating system you use
by opening an [issue](https://github.com/szabgab/site-checker.rs/issues) and we'll
create the binary.


## Reports of a few small sites:

* [izrael.szabgab.com](/izrael.szabgab.com.html)
* [site-checker.code-maven.com](/site-checker.code-maven.com.html)
* [ssg.code-maven.com](/ssg.code-maven.com.html)

