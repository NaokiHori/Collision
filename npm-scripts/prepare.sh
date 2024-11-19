#!/bin/bash

# setup husky to configure on-commit hooks

set -e

echo "--- trying to initialize husky ---"

to_root=$(git rev-parse --show-cdup)
from_root=$(git rev-parse --show-prefix)

if [ -z "${to_root}" ]; then
  to_root="."
fi
if [ -z "${from_root}" ]; then
  from_root="."
fi

echo "  > to_root:   ${to_root}"
echo "  > from_root: ${from_root}"

cd ${to_root} && husky ${from_root}/.husky && cd ${from_root}

echo "  > .husky directory is initialized under ${from_root}"

if [ ! -e .husky ]; then
  mkdir .husky
fi
echo "cd ${from_root} && npx lint-staged" > .husky/pre-commit

echo "  > .husky/pre-commit is written"

echo "--- husky is successfully initialized ---"
