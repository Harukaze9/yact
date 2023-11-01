#!/bin/bash
set -euo pipefail

os_name=""

case "$(uname)" in
    Darwin)  # macOS
        os_name="macos"
        ;;
    Linux)   # Linux
        os_name="linux"
        ;;
    *)
        os_name=""
        ;;
esac

if [ -z "$os_name" ]; then
    echo "Unable to determine a valid OS."
    exit 1
fi

echo "[build.sh] Starting build for yact_core and update."
echo "[build.sh]   Building for target OS: [$os_name]"

cargo build --release --manifest-path "${__yact_root_dir}/yact_core/Cargo.toml"
echo "[build.sh] Finished building yact_core."
cp "${__yact_root_dir}/yact_core/target/release/yact_core" "${__yact_root_dir}/bin/yact_core_${os_name}"
echo "[build.sh] Copied: ${__yact_root_dir}/yact_core/target/release/yact_core => ${__yact_root_dir}/bin/yact_core_${os_name}"
