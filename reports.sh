set -e

cargo run --bin seo-site-checker -- --host https://site-checker.code-maven.com/  --json site/reports/site-checker.code-maven.com.json --html site/reports/site-checker.code-maven.com.html

cargo run --bin seo-site-checker -- --pages 1 --host https://ssg.code-maven.com/ --json site/reports/ssg.code-maven.com.json --html site/reports/ssg.code-maven.com.html

cargo run --bin seo-site-checker -- --host https://izrael.szabgab.com/ --json site/reports/izrael.szabgab.com.json --html site/reports/izrael.szabgab.com.html

cargo run --bin seo-site-checker -- --host https://rust.code-maven.com/ --json site/reports/rust.code-maven.com.json --html site/reports/rust.code-maven.com.html
