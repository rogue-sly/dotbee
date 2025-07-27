# Prompt customization
let-env PROMPT_COMMAND = {|| 
  let path = (pwd)
  $"(ansi cyan_bold)($env.USER)@($env.HOST)(ansi reset): (ansi green)($path)(ansi reset) > "
}

# History settings
let-env config = {
  history: {
    file: "~/.config/nushell/history.txt"
    max_size: 10000
  }

  edit_mode: "emacs"  # or "vi"
}

# Aliases
alias ll = ls -l
alias gs = git status
alias .. = cd ..
alias ... = cd ../..

# Add custom binary paths
let-env PATH = ($env.PATH | append "~/.local/bin")

# Environment variables
let-env EDITOR = "nvim"
