---
title: Starting the Site Checker project
timestamp: 2023-11-27T08:45:01
description: The Site Checker will help me improve the SEO of the other site I build with the Static Site Generator.
---

As a website owner looking at the numbers on the [Google Search console](https://search.google.com/search-console) if you turn on the "Average CTR" and the "Average position"
you can see that the first few positions on every Google search get the bulk of the clicks. Some numbers for various search terms:

| Average position | Average CTR |
| ---------------- | ----------- |
| 1.0              | 80.3%       |
| 1.0              | 64.7%       |
| 1.1              | 58.1%       |
| 1.3              | 55.4%       |
| 2.0              | 44.4%       |
| 2.7              | 19.9%       |
| 2.7              | 25.4%       |
| 3.8              |  8.9%       |
| 4.2              | 11.9%       |
| 7.2              |  3.5%       |
| 9.7              |  2.0%       |


So it is quite obvious that being in the first few positions and especially in the first position has a huge benefit in terms of visitors.

Search Engine Optimization (SEO) is the title of making your site and the specific pages rank higher on the search engines.

There are both internal and external factors that will impact the ranking of your site and the specific pages.
To some extent you can improve the external factors as well (e.g. guest-post on some other site and link to your site),
but you have full control over the internal factors.

As I am learning more on how to improve the [Code Maven Static Site Generator](https://ssg.code-maven.com/)
to make the sites better at SEO, I set out to create a tool that can measure certain aspects of a web site.

There are plenty of tools that can help, and I am sure some of them are free and open source,
but I like programming and building this tool will create many opportunities to learn new techniques,
write about them (creating more content for the [Rust Maven site](https://rust.code-maven.com/)) and
then to [teach them](https://szabgab.com/).

The first version of the project is already working though as of now it only checks the main page of a website.

Visit the [main page](/) from where you'll see reports created for a number of my web sites.
