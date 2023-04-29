FROM rust:1.69

WORKDIR /usr/src

# We want dependencies cached, so copy those first.
COPY . .

# This is the actual application build.
RUN cargo build --release

RUN cp /usr/src/target/release/tweet_bot /usr/local/bin

EXPOSE 3030

# Run the application
CMD ["/usr/local/bin/tweet_bot"]