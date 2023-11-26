mkdir _site

cargo run -- --host https://site-checker.code-maven.com/
mv report.html _site/site-checker.code-maven.com.html

cargo run -- --host https://ssg.code-maven.com/
mv report.html _site/ssg.code-maven.com.html

cargo run -- --host https://izrael.szabgab.com/
mv report.html _site/izrael.szabgab.com.html
