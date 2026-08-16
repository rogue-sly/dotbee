FROM fedora:latest

# Install some packages
RUN dnf install -y neovim git tree bat fish tmux

# Create a mock host and add a test user and
RUN groupadd laptop
RUN useradd -m -s /bin/bash test
USER test
WORKDIR /home/test/dotfiles

# Create dotfiles and profile directories
RUN mkdir -p profiles/laptop profiles/termux

# Create config files for each profile

# Laptop Profile
# kitty
RUN mkdir profiles/laptop/kitty
RUN touch profiles/laptop/kitty/kitty.conf
# neovim
RUN mkdir profiles/laptop/nvim
RUN <<EOF cat > profiles/laptop/nvim/init.lua
vim.o.number = true
vim.o.relativenumber = true
vim.cmd.colorscheme('retrobox')

vim.notify('hello from laptop')
EOF

# Termux Profile
# tmux
RUN mkdir profiles/termux/tmux
RUN touch profiles/termux/tmux/tmux.conf
# neovim
RUN mkdir profiles/termux/nvim
RUN <<EOF cat > profiles/termux/nvim/init.lua
vim.o.number = true
vim.o.relativenumber = true
vim.cmd.colorscheme('habamax')

vim.notify('hello from termux')
EOF

# Global Profile (applies regardless of active profile)
RUN mkdir -p global/git
RUN <<EOF cat > global/git/gitconfig
[user]
    name = test
    email = test@localhost
EOF

# Create dotbee.toml
RUN <<EOF cat > dotbee.toml
[settings]

[vars]
config = "~/.config"

[profiles.global.links]
gitconfig = { src = "global/git/gitconfig", dst = "~/.gitconfig" }

[profiles.laptop.links]
nvim = { src = "profiles/laptop/nvim", dst = "{config}/nvim" }
kitty = { src = "profiles/laptop/kitty", dst = "{config}/kitty" }

[profiles.termux.links]
nvim = { src = "profiles/termux/nvim", dst = "{config}/nvim" }
tmux = { src = "profiles/termux/tmux", dst = "{config}/tmux" }
EOF

CMD ["/bin/fish"]
