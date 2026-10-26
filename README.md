# CS411 Spring 2027 · Project 1: Multithreaded Rust Raytracer

The supplied sequential raytracer and fixed scene are complete. Implement the TODOs in `src/renderer.rs` and `src/worker.rs` to render exclusive row bands concurrently, then assemble them in image order. Read the included project handout before coding.

```sh
cargo test --release
cargo run --release
```

The unchanged starter compiles, but parallel tests fail at intentional TODOs. The completed binary checks each image against the sequential reference, prints median timings for 1/2/4/8 workers, and writes `render.png`.

## Work in your own GitHub repository

The instructor starter is an upstream source. Keep your completed work in a repository under your own GitHub account, then submit the repository URL and the evidence listed in the assignment handout through Blackboard.

If Blackboard links a template repository, choose **Use this template**, create the new repository under your account, and clone that new repository. Check `git remote -v` before pushing; `origin` must be your repository.

For a direct clone, first create an empty repository under your account. Replace the placeholders below with the starter link supplied in Blackboard and your own empty repository URL:

```sh
git clone <instructor-starter-url>
cd <cloned-directory>
git remote rename origin upstream
git remote add origin <your-empty-repository-url>
git remote -v
git push -u origin HEAD
```

Commit and push your completed work to your own `origin`. Do not push to the instructor's upstream repository. Use the exact submission files and verification commands in the assignment handout.
