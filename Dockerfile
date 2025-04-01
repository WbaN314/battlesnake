FROM rustlang/rust:nightly

COPY . /usr/app
WORKDIR /usr/app

RUN cargo install --path .

CMD ["battlesnake_game_of_chicken"]
