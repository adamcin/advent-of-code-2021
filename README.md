# Advent of Code 2021 solutions

Started over vacation in NC when I only had my iPad with me. Thus, my primary constraint for this year's solutions will continue to be using only my iPad, by running GitHub CodeSpaces in this repo.

My day 01 solution started in Java with Gradle because I started later in the day and wanted to get my solution in before I invested more time in figuring out how to not go insane with iPad magic keyboard mappings.

## Key Learnings:

* Change the Caps Lock Key mapping to Escape in `Settings` -> `General` -> `Keyboard` -> `Hardware Keyboard` -> `Modifier Keys`

* Avoid using Ctrl+Space to Trigger Suggest in VS Code, which will always bring up the Emoji Keyboard. Instead, use an alternate binding like Cmd+I, which is also allowed by default.

* Java project complexity and verbosity is increasingly unwieldy on a cramped screen with inconsistent scrolling and selection behavior. Not to mention that my extreme familiarity with Intellij leads to constant frustration with things not working the way I expect them to, or even not at all. I knew I was going to be trying a different language even as I was completing the first day's solution.

* Maximum efficiency in VS Code in a browser on an iPad appears to come from avoiding project explorer GUI operations like the plague. Use a language that allows for all solution code to live in one file, so you can swipe to navigate between functions, modules, etc., instead of fat-fingering your way around the source tree in the tiny column. Use Terminal as much as possible for Git operations.

* Cmd+F binding is totally useless and frustrating in Firefox on iPad. Going to see if my copy-paste issues are also a Firefox-specific issue and not an issue on Safari.

* Safari seems to reach a point where it crashes more and more often while scrolling in a complex source file.

## This is now a Rust project

I needed a reason to try Rust, so I dropped Java after completing the day 01 solution and reimplemented it in rust. I tried generating tests using the VS Code features expecting the Rust extension to create something in the right place. That didn't work, so I copied the hello world example from some tutorial that had a test module right in main.rs, and I'm continuing to add 2 unit tests for each day without adding any non-test code. 

## My Process

* Ran `cargo init` in project folder which added `src/main.rs` and Cargo files.

* Implement a part1 or part2 test fn in main.rs with an `assert_eq!` expression to force printing of the result in `cargo test`. If I believe the result, I submit it to AoC to see if it works, and if so, update the expected arg in the assertion.

