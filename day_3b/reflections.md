# Day 3 Part 2

## Thought Process

A little more complicated than part 1. Instead of directly finding `do() xxxxx don't()` patterns, I instead opted to find `don't() xxxxx do()` patterns and then subtract the `mul()` commands inside these patterns from the answer we obtained in part 1.

Then extracting the `mul()` patterns is the same as in part 1:
> Since we are only extracting out one pattern, it seems quite straightforward to just use regex to extract out any parts of the input text that has the required pattern `mul(x,y)` where `x` and `y` are both integers.

The final answer is `107862689`.

## Reflections

- Had the most problems trying to figure out what regex string to use? first i tried to find `do() xxx don't()` patterns and calculate directly but it didn't really work very well so i changed my approached (which worked better)

- had to factor in the `\n\r` (ie. newline) shit cuz the don't() commands don't carry over, so making everything stay in one line eventually fixed this problem. (credits to the subreddit though i saw people having the same problem then i realised it was an actual issue)
