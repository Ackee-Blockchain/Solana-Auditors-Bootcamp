![Solana Auditors Bootcamp](https://github.com/Solana-Auditors-Bootcamp/.github/blob/main/.banner/Solana%20Auditors%20Bootcamp.png?raw=true)

# Task 1: Fuzz a Solana Program

Lesson 3 introduces you to Fuzzing with Trident. The goal of this task is to familiarize yourself with writing fuzz tests for Solana programs written in Anchor.

## Task details

This task involves writing a fuzz test for an open-source Solana project written with Anchor using the Trident Fuzz Testing Framework. Fuzz testing is a technique where you automatically generate a wide range of random inputs to test your program and catch edge cases or potential security vulnerabilities that might not be covered by typical unit or integration tests.

**The details:**

1. **Select a Project:** Choose an open-source Solana project that you want to fuzz test. The project could be one you are already familiar with, or select any other project with open-source code available on GitHub.

2. **Integrate Trident:** Set up the Trident framework within the chosen project. The setup involves initializing Trident and configuring it appropriately.

3. **Identify Critical Functions:** Identify key functions or processes within the Solana program that are critical to its operation. These are the functions where fuzz testing will be most beneficial.

4. **Write Fuzz Tests:** Implement fuzz tests for the selected program. The goal is to generate a variety of random inputs to test the program's response to unexpected, invalid, or even valid inputs.

5. **Run and Analyze:** Run the fuzz tests using Trident and analyze the results. Identify any crashes, panics, or unexpected behaviors that indicate vulnerabilities or bugs.

6. **Document Your Findings:** Provide a report detailing the fuzz testing process, and write a short README.md about the project you fuzzed. It is also important to include the implemented Fuzz Tests and CrashFile/s (if any)!

## Bug Bounties & Responsible Bug Disclosures

> [!tip]
> We recommend you explore whether the project you want to fuzz has an active bug bounty. This not only can serve as an incentive to secure the project but can also help you build up a portfolio of detected and fixed bugs.

> [!caution]
> If you detected a bug in the project you fuzzed, which could lead to loss of TVL (total value locked) or any other issues that could harm the project. In that case, we encourage you to follow white hat hacker principles, ethics, and responsible bug disclosure practices. Do not push a detected bug onto GitHub if the project has not resolved the bug. You can communicate this to us through a private support ticket, and we will extend your deadline until the project has resolved the issue.

## Submission Process

> [!important]
> Within the `project-fuzzing` folder, include the project you were fuzzing with a short README.md detailing the found crashes or anything else you think is important. Do not forget to also include the Fuzz Tests within the folder!

## Contributing to Trident Process

If you find an issue with Trident while fuzzing please open an issue on GitHub and follow the CONTRIBUTING.md guidelines.

## Deadline

The deadline for this task is Tuesday, September 10th, at 12:00 UTC.

## Hints and Useful Links

- [Lesson 3 Materials](https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/master/Lesson-3)
- [Trident Documentation](https://ackee.xyz/trident/docs/latest/)
- [Trident Examples](https://ackee.xyz/trident/docs/latest/fuzzing/extra/examples/)
- [Youtube Lesson](https://youtu.be/5JRVnxGW8kc?si=bx3dnq7kGQV7_uYk)

-----

### Need help?

If you have any questions, please reach out to us on [Discord](https://discord.gg/z3JVuZyFnp).
