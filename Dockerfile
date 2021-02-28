FROM rust:1.50-slim

RUN echo "-----> Installing tools"
RUN apt update && apt install -y libwebkit2gtk-4.0-dev \
    build-essential \
    wget \
    libssl-dev \
    appmenu-gtk3-module \
    libgtk-3-dev \
    squashfs-tools
RUN apt-get install -y curl unzip \
        sqlite3 node \
    && apt-get clean && rm -rf /var/cache/apt/* && rm -rf /var/lib/apt/lists/* && rm -rf /tmp/*

# RUN echo "-----> Installing Rust channel stable"
# RUN curl https://sh.rustup.rs -sSf > rustup.sh
# RUN chmod u+x rustup.sh
# RUN ./rustup.sh -y --default-toolchain stable
# RUN rm rustup.sh

# ENV PATH="$HOME/.cargo/bin:$PATH"
RUN cargo install diesel_cli --no-default-features --features sqlite --force
RUN cargo install tauri-bundler --force

RUN echo "-----> Installing yarn"
RUN npm install -g yarn

WORKDIR /app

COPY package*.json ./

# install project dependencies
RUN yarn install

# copy project files and folders to the current working directory (i.e. 'app' folder)
COPY . .

# init db migration
RUN diesel setup
RUN diesel migratin run

RUN yarn tauri:serve