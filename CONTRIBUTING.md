# Contributing Guidelines

This document outlines the expectations for any contributions to Fuji.  It is recommended you read this first as to avoid later confusion.

## General

### Use of "AI" / Large Language Models (LLMs)

> TL;DR: "AI" / LLMs are strictly prohibited from being used in contributions to Fuji.

Contributions cannot be made with the use of "AI" / Large Language Models (LLMs).<br>
You may not use "AI" / LLMs for discussions, commits, reviewing or creating code, or any contributions whatsoever (even those not listed explicitly here).<br>
This also extends to issues, comments on issues, comments on pull requests, etc.<br>
If your IDE / editor uses "AI" / LLMs or has them enabled as optional functionality, either switch IDE / editor, or disable the features if possible.<br>
If it is not possible for your IDE / editor to disable "AI" / LLM features, and you cannot switch IDE / editor, then **do not contribute**.<br>
If you have some objection to this policy: I don't care, you're not welcome here.<br>
The above prohibitions also apply to translations.

## Style

- When comments or documentation features technical terms, use [Markdown](https://commonmark.org/) links to point to information about the discussed topic.
- Use `en-GB` (British English) or `en-AU` (Australian English) for contributions.
  - Favour `en-GB` where `en-AU` uses `en-US` (U.S. English) spellings & grammar.
  - If an external library depends on uses `en-US` spelling for a struct / function / etc., use an [aliased import](https://doc.rust-lang.org/reference/items/use-declarations.html#as-renames) and rename the item to use `en-GB` or `en-AU` (where possible).
  - No, Oxford English doesn't count!  Keep your filthy `z`s out of words!
- Proofread!
  - Proofread your documentation and comments!
  - Proofread your code!
  - Proofread everything!
    - Seriously, just give what you've written a once-over before you commit or make a pull request, it takes very little time and can catch bugs, typos, errors, etc.