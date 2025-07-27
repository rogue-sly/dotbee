# Set Zsh options
setopt autocd           # Auto-change to directory if command is a directory
setopt no_beep          # Disable system bell
setopt correct          # Correct command spelling

# History settings
HISTFILE=~/.zsh_history
HISTSIZE=10000
SAVEHIST=10000
setopt hist_ignore_all_dups
setopt share_history

# Prompt customization (basic)
autoload -Uz colors && colors
PROMPT='%F{cyan}%n@%m%f %F{yellow}%~%f %# '

# Aliases
alias ll='ls -lah'
alias gs='git status'
alias gp='git pull'
alias ..='cd ..'
alias ...='cd ../..'

# Environment variables
export EDITOR=nvim
export PATH="$HOME/.local/bin:$PATH"

# Plugins (if using a plugin manager like zinit, oh-my-zsh, etc.)
# Example: source ~/.zinit/bin/zinit.zsh

# Source custom scripts
[[ -f ~/.zsh_aliases ]] && source ~/.zsh_aliases
[[ -f ~/.zsh_exports ]] && source ~/.zsh_exports
