#!/usr/bin/env bash

# Everything CI checks in this repository, in one command.
#
#   ./build-and-test.sh          format, lint, test, and every documented snippet
#   ./build-and-test.sh check    the same thing; the name CI uses
#   ./build-and-test.sh fix      format Rust and C++ in place first
#
# **Half of what runs is in `bin/gate-common.sh`**, of which every repository
# in the organisation carries a byte-identical copy. This file is what this
# repository configures, what only it checks, and the order they run in.
# `xpui-dev` compares the nine copies and runs all nine gates.

set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${PROJECT_DIR}"

# Four crates, one per vendor over a shared core.
SOURCE_ROOTS=(core pimoroni seeed xteink)

# Nothing here needs a feature: a board is data, and the tests read it.
TEST_FEATURES=""

# Board data is compiled into every firmware, so it is linted for both
# bare-metal architectures. Neither has atomic compare-and-swap; the second is
# not the stricter run, it is the second architecture. The `?` says Cortex-M0+
# may skip when the target is not installed rather than failing a gate somebody
# cannot fix without a download.
HOST_WORKSPACE=1
LINT_TARGETS=("riscv32imc-unknown-none-elf" "thumbv6m-none-eabi?")
LINT_TARGET_CRATES=(-p xpui-boards-core -p xpui-boards-pimoroni
                    -p xpui-boards-seeed -p xpui-boards-xteink)

. bin/gate-common.sh

gates() {
  file_sizes
  every_check_runs
  readmes_warn
  prose_is_compiled
  doc_paths
  cpp_snippets_compile
  lint
  test_suite
  doc_tests
  doc_links
}

case "${1:-check}" in
  check)
    run_all "${FORMAT_CHECK[@]}"
    gates
    printf '\nChecks passed.\n'
    ;;
  fix)
    run_all "${FORMAT_FIX[@]}"
    gates
    printf '\nFormatted and checked.\n'
    ;;
  *)
    echo "usage: ./build-and-test.sh [check|fix]" >&2
    exit 2
    ;;
esac
