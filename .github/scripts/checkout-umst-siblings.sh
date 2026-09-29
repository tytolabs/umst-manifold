#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
# Clone UMST sibling repositories next to the runner workspace (../<repo>) without credentials.
# Public siblings are cloned anonymously at the requested SHA. A private sibling cannot be cloned without
# credentials, and this workflow uses none: the job stops and names the public/private boundary (W-63).
set -euo pipefail
PARENT="$(dirname "${GITHUB_WORKSPACE:?GITHUB_WORKSPACE required}")"
clone_public() {
  local name="$1" sha="${2:-}" dest="${PARENT}/$1"
  local url="https://github.com/tytolabs/${name}.git"
  if ! git ls-remote --exit-code "${url}" HEAD >/dev/null 2>&1; then
    echo "::error::${name} is private: this public repository depends on it by path and cannot build from a fresh clone. Resolve through W-63 (public ladder or a public-only dependency graph)."
    return 2
  fi
  if [ ! -d "${dest}/.git" ]; then git clone --depth 1 "${url}" "${dest}"; fi
  if [ -n "${sha}" ]; then
    git -C "${dest}" fetch --depth 1 origin "${sha}"
    git -C "${dest}" checkout "${sha}"
  fi
  echo "cloned ${name} @ $(git -C "${dest}" rev-parse --short HEAD)"
}
status=0
for spec in "$@"; do
  name="${spec%%@*}"; sha=""
  if [[ "${spec}" == *"@"* ]]; then sha="${spec#*@}"; fi
  clone_public "${name}" "${sha}" || status=$?
done
exit "${status}"
