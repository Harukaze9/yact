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
        echo "Uninstallation Error: Unsupported shell type: $user_shell"
        exit 1
        ;;
esac

if grep -q "# >>> YACT added lines >>>" "$config_file"; then
    echo "Removing YACT configuration from $config_file..."
    
    if [[ "$(uname)" == "Darwin" ]]; then
        # for mac OS
        sed -i '' '/# >>> YACT added lines >>>/,/# <<< YACT added lines <<</d' "$config_file"
    else
        sed -i '/# >>> YACT added lines >>>/,/# <<< YACT added lines <<</d' "$config_file"
    fi
    
    echo "YACT configuration has been successfully removed from $config_file."
else
    echo "YACT configuration not found in $config_file."
fi
