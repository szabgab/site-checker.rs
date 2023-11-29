
cargo run -- --pages 10 --host https://site-checker.code-maven.com/
mv report.html site/site-checker.code-maven.com.html

cargo run -- --pages --host https://ssg.code-maven.com/
mv report.html site/ssg.code-maven.com.html

cargo run -- --pages 10 --host https://izrael.szabgab.com/
mv report.html site/izrael.szabgab.com.html
