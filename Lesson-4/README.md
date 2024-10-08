# Lesson 4 - Fuzzing with Trident II


> [!IMPORTANT]
> For more details about the Trident, check the Trident documentation [Tridetn docs](https://ackee.xyz/trident/docs/latest/)

> [!IMPORTANT]
> Check the Part I of Fuzzing with Trident for more better introduction into Fuzzing [Fuzzing with Trident I](../Lesson-3/)

## Table of Contents
- [Invariant Checks](#invariant-checks)
- [Instruction Sequences](#instruction-sequqnces)
- [Custom Error Handler](#custom-error-handler)
- [Well Structure Data](#well-structured-data)
- [Client Methods](#client-methods)
- [Trident Manifest](#trident-manifest)

---

> [!IMPORTANT]
> For all of the following features implemented in the Fuzz Test, check the [trident-lesson-part-ii](./trident-lesson-part-ii/)

## Invariant Checks

Trident provides ability to define Invariant checks, this add opportunity to check how the Accounts were updated within the corresponding Instruction and if the updates were performed as expected.

## Instruction Sequqnces

Trident allows you to specify desired Instruction Sequences. You can specify

1. Expected Instruction/s executed at the start of Fuzzing iteration, In the Middle, or in the End.
2. Whole Instruction Sequence that you want the Fuzzer should execute within each fuzzing iteration.

## Custom Error Handler

In case of Instruction failure, Trident allows you to implement Custom Error Handler to check (and compare) or even intersect the Instruction Sequence and define behavior in case of specific error occured.

## Well Structured Data

Using the Arbitrary crate, Trident allows you to give better structure to the random data generated by the underlying Fuzzing Engine. In this case Instruction parameters can be well structured with some dependencies instead of completely random and independant instruction inputs.

## Client Methods

Trident specifies multiple methods to communicate with the Testing Environment. For this purpose it is possible to for example set completely custom account manually instead of calling specific Initialize instruction.

## Trident Manifest

Initialization of Trident generates also the Trident.toml. Parameters specified in this file can be use to increase the Fuzzing Experience,
