# You might be wondering, why dockerfile? Well... I don't want to accidentally
# destroy my home directory xD

FROM rust:latest

WORKDIR /app

RUN apt-get update && apt-get install -y vim git tmux tree

# dotsy can't be used by root and I will not support that
RUN useradd -m -s /bin/bash testuser

# copy && build project
COPY . .
RUN cargo build
USER testuser
WORKDIR /home/testuser

# create some dummy dotfiles to link
RUN mkdir -p dotfiles/bash dotfiles/git dotfiles/linux
RUN touch dotfiles/bash/bashrc
RUN touch dotfiles/git/gitconfig
RUN touch dotfiles/linux/i3_config

# initialize dotsy in the dotfiles directory
WORKDIR /home/testuser/dotfiles
RUN /app/target/debug/dotsy init

# modify the generated config to point to our dummy files
RUN sed -i 's|# \"~/.bashrc\"|\"~/.bashrc\"|' dotsy.toml
RUN sed -i 's|# \"~/.gitconfig\"|\"~/.gitconfig\"|' dotsy.toml
RUN sed -i 's|# \"~/.config/i3/config\"|\"~/.config/i3/config\"|' dotsy.toml

CMD ["/bin/bash"]
