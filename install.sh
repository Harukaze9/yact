#!/bin/bash

user_shell=$(basename $SHELL)

case $user_shell in
    bash)
        config_file="$HOME/.bashrc"
        ;;
    zsh)
        config_file="$HOME/.zshrc"
        ;;
    *)
        echo "Installation Error: Unsupported shell type: $user_shell"
        exit 1
        ;;
esac

if grep -q "# >>> YACT added lines >>>" "$config_file"; then
    echo "The script has already been integrated into $config_file."
else
    script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    script_path="$script_dir/src/source-yact.sh"

    echo "This installation script intends to append the following lines to $config_file:"
    echo ""
    echo "source $script_path"
    echo ""
    echo "If you prefer to make these modifications manually, you can do so following the prompts."
    echo -n "Would you like to apply these changes automatically? (y/n) "
    read answer

    if [ "$answer" == "y" ]; then
        echo "# >>> YACT added lines >>>" >> "$config_file"
        echo "source $script_path" >> "$config_file"
        echo "# <<< YACT added lines <<<" >> "$config_file"
        echo "Configuration has been successfully appended to $config_file."

        echo ""
        echo "To complete the installation, either restart your shell or run:"
        echo "source $config_file"
        echo "Then, execute 'yact' command to ensure that the installation was successful."

    else
        echo ""
        echo "Automatic modifications were not made. To install YACT manually, please add the following lines to your shell's configuration file:"
        echo "# >>> YACT added lines >>>"
        echo "\"source $script_path\""
        echo "# <<< YACT added lines <<<"
        echo "After adding, remember to either restart your shell or source the configuration file, and then test the installation with the 'yact' command."
    fi
fi
