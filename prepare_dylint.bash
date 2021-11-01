#!/usr/bin/env bash
set -o xtrace -o pipefail -o errexit -o nounset

function main() {
    local crate_name
    crate_name=${1:?"Must specify library name"}

    local library_path
    library_path=$(fd --no-ignore \
        --type executable \
        "${crate_name}" \
        | most-recently created
    )

    local toolchain
    toolchain=$(rustup show active-toolchain | awk '{print $1}')

    cp "${library_path}" "lib$crate_name@$toolchain.so"
}

main "$@"
