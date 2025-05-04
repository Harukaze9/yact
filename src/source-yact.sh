# =============== load script paths ==================== #
# Use script's location as the root directory
if [ -n "$BASH_VERSION" ]; then
  __yact_src_dir=$(cd $(dirname ${BASH_SOURCE:-$0}); pwd) # for bash
elif [ -n "$ZSH_VERSION" ]; then
  __yact_src_dir=$(dirname ${0}) # for zsh
else
  # path retrieval may fail (depending on the shell's specifications)
  __yact_src_dir=$(dirname ${0})
fi

export __yact_root_dir=$(dirname $__yact_src_dir)
export __yact_config_path="${__yact_root_dir}/yact_config"

export YACT_LOGGER_PATH="${__yact_src_dir}/yact-debug-logger.sh"
case "$(uname)" in
    Darwin)  # macOS
        YACT_CORE_PATH="${__yact_root_dir}/bin/yact_core_macos"
        YACT_COLORLIZER_PATH="${__yact_root_dir}/bin/yact_colorlizer_macos"
        ;;
    Linux)   # Linux
        YACT_CORE_PATH="${__yact_root_dir}/bin/yact_core_linux"
        YACT_COLORLIZER_PATH="${__yact_root_dir}/bin/yact_colorlizer_linux"
        ;;
esac
export YACT_CORE_PATH;
export YACT_COLORLIZER_PATH;
if [[ ! -x "$YACT_CORE_PATH" ]]; then
    echo "WARNING: No yact_core executable found at $YACT_CORE_PATH. To use YACT, please resolve this issue."
fi

__yact_generated_dir="${__yact_root_dir}/.generated"
__yact_commands_dir="${__yact_root_dir}/commands"

# =========== check dependency =================== #
if ! command -v awk >/dev/null 2>&1 ; then
  echo "YACT Warning: awk is not installed. please install awk to use YACT"
fi

# ============  load commands from YAML ========================== #
__yact_source_each_yaml_commands() {
  find "${__yact_generated_dir}" -name "*.sh" -type f -exec rm -f {} \;
  for source_yaml in `find "${__yact_commands_dir}" -maxdepth 1 \( -type f -o -type l \) -name "*.yaml" `
  do
    local basename=$(basename "${source_yaml}" .yaml)
    local temp_filename="${__yact_generated_dir}/yact_${basename}.sh"
    sed -e "s/%__yact_function_name%/${basename}/g" -e "s#%__yact_source_yaml_path%#${source_yaml}#g" ${__yact_src_dir}/yact_template.sh  > ${temp_filename}
    source ${temp_filename}
  done
}

__yact_source_each_yaml_commands

# =============== source scripts =======================
for source_script in `find "${__yact_commands_dir}" -maxdepth 1 \( -type f -o -type l \) -name "*.sh" `
do
source "${source_script}"
done
# ==============================================================