# %__yact_function_name% and %__yact_source_yaml_path% will be replaced when generating temporary scripts.

# main func
%__yact_function_name%() {
    local source_yaml_path=%__yact_source_yaml_path%
    local params=("$@")

    # Handle "-e" option
    for param in "${params[@]}"; do
        if [[ "$param" == "-e" ]]; then
            local editor_command=`yact _config get editor`
            if [[ -z "$editor_command" ]]; then
                echo "Missing YACT editor config. Please input your favorite editor command below (e.g., vim, code, emacs)."
                read editor_command;
                if ! command -v "$editor_command" > /dev/null; then
                    echo "$editor_command is not a valid editor command."
                    return
                fi
                yact _config set editor $editor_command
                echo "YACT editor is now: [`yact _config get editor`]"
            fi
            eval "${editor_command} ${source_yaml_path}"
            return
        fi
    done

    # Handle "-a" option with no arguments
    if [ ${#params[@]} -gt 0 ] && [ "${params[${#params[@]}]}" = "-a" ]; then
        local last_command=$(fc -ln -1 | sed 's/^[[:space:]]*//')

        if [[ -z "$last_command" ]]; then
            echo "Error: No command was found to add, and no previous command was executed."
            return 1
        fi
        params+=("$last_command")
        echo "No specific command was provided to add; YACT will instead use the last executed command: [$last_command]"
    fi

    # call yact_core
    local output=$(${YACT_CORE_PATH} execution ${source_yaml_path} "${params[@]}")
    
    local status_output=$(echo "$output" | awk 'NR==1' | base64 -d)
    local result=$(echo "$output" | awk 'NR==2' | base64 -d)
    local message=$(echo "$output" | awk 'NR==3' | base64 -d)


    echo "Status is: [$status_output]" | ${YACT_LOGGER_PATH}
    echo "Result is: [$result]" | ${YACT_LOGGER_PATH}
    echo "Message is: [$message]" | ${YACT_LOGGER_PATH}

    if [ "$status_output" = "command" ]; then
        eval "${result}"
    else
        if [ "$result" = "colorlize" ]; then
            echo "${message}" | ${YACT_COLORLIZER_PATH}
        else
            echo "${message}"
        fi
    fi
}

# completion func
__completion_%__yact_function_name%()
{
    local source_yaml_path=%__yact_source_yaml_path%
    COMPREPLY=()

    local output=$(${YACT_CORE_PATH} completion ${source_yaml_path} "${COMP_WORDS[@]:1:(COMP_CWORD-1)}")
    
    local status_output=$(echo "$output" | awk 'NR==1' | base64 -d)
    local result=$(echo "$output" | awk 'NR==2' | base64 -d)
    local message=$(echo "$output" | awk 'NR==3' | base64 -d)

    echo "(completion) Status is: [$status_output]" | ${YACT_LOGGER_PATH}
    echo "(completion) Result is: [$result]" | ${YACT_LOGGER_PATH}
    echo "(completion) Message is: [$message]" | ${YACT_LOGGER_PATH}


    if [[ "$status_output" == "completion_command" ]]; then
        completion_list=$(eval "$result")
    elif [[ "$status_output" == "completion_default" ]]; then
        # emulates default completion of shell
        local cur="${COMP_WORDS[COMP_CWORD]}"
        COMPITEMS=()
        for comp in $(compgen -f -- "$cur"); do
            if [ -d "$comp" ]; then
                comp="$comp/"
            fi
            COMPITEMS+=("$comp")
        done
        if [ ${#COMPITEMS[@]} -eq 1 ] && [ "${COMPITEMS[0]%/}" != "${COMPITEMS[0]}" ]; then
            if [ -n "$BASH_VERSION" ]; then
                compopt -o nospace
            elif [[ -n "${ZSH_VERSION}" ]]; then
                COMPITEMS=$(bash -c "compgen -f -- ${COMPITEMS[0]}")
            fi
        fi
        completion_list="${COMPITEMS[@]}"
        echo "(completion) emulated shell completion: [${completion_list}]" | ${YACT_LOGGER_PATH}
    else
        completion_list="$result"
    fi

    # if option is given
    if [[ "${COMP_WORDS[COMP_CWORD]}" == -* ]]; then
        completion_list="${result} ${message}"
    fi

    COMPREPLY=( `compgen -W "${completion_list}" -- ${COMP_WORDS[COMP_CWORD]}` );
}

# bind completion function
complete -F __completion_%__yact_function_name% %__yact_function_name%