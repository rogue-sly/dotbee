FROM fedora:latest

RUN dnf install -y vim git tmux tree bat && dnf clean all

# dotsy can't be used by root and I will not support that
RUN useradd -m -s /bin/bash testuser

# copy the locally built binary into the path
COPY --chown=testuser:testuser --chmod=755 target/debug/dotsy /usr/local/bin/dotsy

USER testuser
ENV PATH="/usr/local/bin:${PATH}"
WORKDIR /home/testuser

# create some dummy dotfiles to link
RUN mkdir -p dotfiles/bash dotfiles/git dotfiles/linux dotfiles/server dotfiles/termux
RUN touch dotfiles/bash/bashrc
RUN touch dotfiles/git/gitconfig
RUN touch dotfiles/linux/i3_config
RUN touch dotfiles/linux/polybar_config
RUN touch dotfiles/server/tmux.conf
RUN touch dotfiles/termux/.termux

# initialize dotsy in the dotfiles directory
WORKDIR /home/testuser/dotfiles

CMD ["/bin/bash"]
