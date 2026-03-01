FROM rust:latest

ARG TZ
ENV TZ="$TZ"

# Install Node.js 20.x
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash -

# Install basic development tools
RUN apt-get update && apt-get install -y \
  less \
  git \
  procps \
  sudo \
  fzf \
  fish \
  man-db \
  unzip \
  gnupg2 \
  curl \
  build-essential \
  nodejs \
  vim \
  && rm -rf /var/lib/apt/lists/*

# Create vscode user with sudo privileges
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
  && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME -s /usr/bin/fish \
  && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
  && chmod 0440 /etc/sudoers.d/$USERNAME

# Ensure vscode user has access to npm global
RUN mkdir -p /usr/local/share/npm-global && \
  chown -R $USERNAME:$USERNAME /usr/local/share

# Persist bash history
RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/commandhistory/.bash_history" \
  && mkdir /commandhistory \
  && touch /commandhistory/.bash_history \
  && chown -R $USERNAME /commandhistory

# Set `DEVCONTAINER` environment variable to help with orientation
ENV DEVCONTAINER=true

# Create workspace directory and set permissions
RUN mkdir -p /workspace /home/$USERNAME/.claude && \
  chown -R $USERNAME:$USERNAME /workspace /home/$USERNAME/.claude

WORKDIR /workspace

# Install git-delta for better git diffs
RUN ARCH=$(dpkg --print-architecture) && \
  wget "https://github.com/dandavison/delta/releases/download/0.18.2/git-delta_0.18.2_${ARCH}.deb" && \
  dpkg -i "git-delta_0.18.2_${ARCH}.deb" && \
  rm "git-delta_0.18.2_${ARCH}.deb"

# Switch to vscode user
USER $USERNAME

# Install Rust components
RUN rustup component add clippy rustfmt rust-analyzer

# Set up npm global prefix
ENV NPM_CONFIG_PREFIX=/usr/local/share/npm-global
ENV PATH=$PATH:/usr/local/share/npm-global/bin

# Install Claude Code and Gemini CLI
RUN npm install -g @anthropic-ai/claude-code @google/gemini-cli @openai/codex

# Set the default shell to fish
ENV SHELL=/usr/bin/fish

# Set up fish shell configuration
RUN mkdir -p /home/$USERNAME/.config/fish && \
  echo "# Add cargo bin to PATH" >> /home/$USERNAME/.config/fish/config.fish && \
  echo "set -gx PATH /home/$USERNAME/.cargo/bin \$PATH" >> /home/$USERNAME/.config/fish/config.fish && \
  echo "# Set up fzf key bindings" >> /home/$USERNAME/.config/fish/config.fish && \
  echo "source /usr/share/doc/fzf/examples/key-bindings.fish 2>/dev/null; or true" >> /home/$USERNAME/.config/fish/config.fish && \
  chown -R $USERNAME:$USERNAME /home/$USERNAME/.config

# Add cargo bin to PATH
ENV PATH="/home/$USERNAME/.cargo/bin:${PATH}"
