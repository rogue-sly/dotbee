FROM fedora:latest

RUN dnf install -y vim neovim git fastfetch tree bat hostname fish && dnf clean all

# dotsy can't be used by root and I will not support that
RUN useradd -m -s /bin/bash testuser

# Copy example dotfiles
COPY --chown=testuser:testuser example /home/testuser/dotfiles

# initialize dotsy in the dotfiles directory
USER testuser
WORKDIR /home/testuser/dotfiles

CMD ["/bin/fish"]
