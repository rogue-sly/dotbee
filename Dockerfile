FROM fedora:latest

RUN dnf install -y vim neovim git fastfetch tree bat hostname fish && dnf clean all

ARG USER_ID=1000
ARG GROUP_ID=1000

RUN groupadd -g $GROUP_ID testuser 2>/dev/null || groupadd testuser
RUN useradd -m -u $USER_ID -g $GROUP_ID -s /bin/bash testuser 2>/dev/null || useradd -m -s /bin/bash testuser

# Copy example dotfiles
COPY --chown=testuser:testuser example /home/testuser/dotfiles

# initialize dotbee in the dotfiles directory
USER testuser
WORKDIR /home/testuser/dotfiles

CMD ["/bin/fish"]
