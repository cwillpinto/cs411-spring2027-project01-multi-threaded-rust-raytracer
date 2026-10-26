# Project 1: Multi threaded Rust Raytracer

**CS 411: Computing Paradigms - Spring 2027**  
**Week 5: February 8-14, 2027**  
**Due:** February 14, 2027  
**Value:** 100 points

**Starter repository:** [Open the Spring 2027 starter](https://github.com/cwillpinto/cs411-spring2027-project01-multi-threaded-rust-raytracer)

## Purpose

Turn the sequential renderer into a deterministic multi-threaded Rust application. Share immutable scene data, assign each output row to one worker, and prove that the parallel image matches the sequential reference before reporting speedup.

## Learning objectives

- Partition image work safely
- Use Rust ownership to prevent shared mutable output
- Verify determinism and performance

## Before you begin

1. Review the Week 5 slide deck and assigned VoiceThread or reading.
2. Create your own repository from the Spring 2027 template linked in Blackboard. The fixed scene, pixel shader, and sequential renderer are complete. Your work is the concurrent row-band renderer in `src/renderer.rs` and `src/worker.rs`.
3. Run `cargo test --release` before editing. The parallel tests fail at the intentional TODOs. Save the initial result in `Project1_Terminal.txt`.

## Milestone A Architecture and baseline (20 points)

Document the sequential reference.

- Use the supplied sequential renderer as the correctness baseline. Run it through the completed benchmark and record its image hash and elapsed time.
- In `Project1_Report.md` or PDF, draw a row-ownership diagram for an image of 17 rows on four workers. Show nonoverlapping bands, including the uneven final band.
- Explain why the scene is immutable and why each worker should return a private RGB byte buffer.

## Milestone B Parallel renderer (50 points)

Implement a bounded worker design.

- Share the immutable `Scene` through `Arc`; clone the `Arc` for each worker.
- Complete `worker::render_rows` for the half-open interval `[start_row, end_row)` using the supplied `pixel` function. Give each worker its own output vector, with no frame-buffer mutex.
- In `render_parallel`, divide all rows into bounded, nonoverlapping bands; spawn at most the requested worker count; join workers and assemble their returned bytes in top-to-bottom row order.
- Support 1, 2, 4, and 8 workers, including an image with fewer rows than workers.

## Milestone C Verification and report (30 points)

Establish correctness before speed.

- Run `cargo test --release` and `cargo run --release`. The binary checks every parallel image against the sequential bytes, prints a stable image hash, reports the median of three timings for each of 1, 2, 4, and 8 workers, and writes `render.png`.
- Save the printed worker rows as `Project1_Timing.csv` and all commands/output as `Project1_Terminal.txt`. Run the binary a second time and confirm the image hash is unchanged.
- In the report, calculate speedup relative to one worker. Explain where scaling flattens, using evidence from the timing table and the cost of worker creation, work partitioning, or memory traffic.

## Verification

- `cargo test --release` passes the uneven-row and repeated-render checks.
- Sequential and parallel bytes and image hashes match for each worker count.
- Repeated runs produce the same image hash, and `render.png` opens as a 640 × 360 RGB image.

## What to submit

Submit your own repository URL and:

1. Your own repository URL (created from the Spring 2027 template).
2. `src/renderer.rs`, `src/worker.rs`, and `Cargo.toml` in that repository.
3. `render.png`.
4. `Project1_Report.pdf` or `Project1_Report.md`, including the row-ownership diagram.
5. `Project1_Timing.csv` and `Project1_Terminal.txt`.

## Rubric

| Criterion | Points |
| --- | ---: |
| Architecture and ownership plan | 15 |
| Correct parallel renderer | 35 |
| Rust ownership and synchronization | 15 |
| Determinism and image evidence | 15 |
| Performance study | 10 |
| Technical report | 10 |
| **Total** | **100** |

## AI usage and academic integrity

Add an **AI Usage** section to the required report file. Name each tool and list the prompts or interactions that affected the work. If you did not use AI, write `AI Usage: None.` You must be able to explain and defend every submitted design and line of code.

## Late work

Late work may be accepted for up to three calendar days with a 25 percent deduction per day, following the course syllabus.

