#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
# Clone private UMST siblings next to the runner workspace (../<repo>).
set -euo pipefail
if [ -z "${UMST_PRIVATE_CHECKOUT:-}" ]; then
  echo "::error::UMST_PRIVATE_CHECKOUT secret missing. Operator: gh secret set UMST_PRIVATE_CHECKOUT -R tytolabs/REPO --body YOUR_PAT (see workspace/ops/G14_CI.json OPERATOR_ACTION)"
  exit 1
fi
PARENT="$(dirname "${GITHUB_WORKSPACE:?GITHUB_WORKSPACE required}")"
clone_private() {
  local name="$1"
  local sha="${2:-}"
  local dest="${PARENT}/${name}"
  local url="https://x-access-token:${UMST_PRIVATE_CHECKOUT}@github.com/tytolabs/${name}.git"
  if [ -d "${dest}/.git" ]; then
    git -C "${dest}" fetch --depth 1 origin "${sha:-HEAD}"
    git -C "${dest}" checkout "${sha:-FETCH_HEAD}"
  else
    if [ -n "${sha}" ]; then
      git clone --depth 1 "${url}" "${dest}"
      git -C "${dest}" fetch --depth 1 origin "${sha}"
      git -C "${dest}" checkout "${sha}"
    else
      git clone --depth 1 "${url}" "${dest}"
    fi
  fi
  echo "cloned ${name} @ $(git -C "${dest}" rev-parse --short HEAD)"
}
for spec in "$@"; do
  name="${spec%%@*}"
  sha=""
  if [[ "${spec}" == *"@"* ]]; then sha="${spec#*@}"; fi
  clone_private "${name}" "${sha}"
done
