local rust = "rust:1.36.0";

[
    {
        kind: "pipeline",
        name: "main",
        steps: [
            {
                name: "fmt",
                image: rust,
                commands: [
                    "rustup component add rustfmt --toolchain " + rust + "-x86_64-unknown-linux-gnu"
                ],
            },
            {
                name: "tests",
                image: rust,
                commands: [
                    "cargo test"
                ],
            },
        ],
    },
]