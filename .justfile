build:
    #!/usr/bin/env bash
    set -euxo pipefail
    name=$(grep -oP '^name\s*=\s*"\K([^"]*)' Cargo.toml)
    version=$(grep -oP '^version\s*=\s*"\K([^"]*)' Cargo.toml)
    docker build -t "atareao/${name}:v${version}" .

push:
    #!/usr/bin/env bash
    set -euxo pipefail
    name=$(grep -oP '^name\s*=\s*"\K([^"]*)' Cargo.toml)
    version=$(grep -oP '^version\s*=\s*"\K([^"]*)' Cargo.toml)
    docker push "atareao/$name:v${version}"

run:
    #!/usr/bin/env bash
    set -euxo pipefail
    name=$(grep -oP '^name\s*=\s*"\K([^"]*)' Cargo.toml)
    version=$(grep -oP '^version\s*=\s*"\K([^"]*)' Cargo.toml)
    docker run -it --rm --init --env-file .env --name "${name}" "atareao/${name}:v${version}"
