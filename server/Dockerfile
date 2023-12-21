FROM rust:latest
WORKDIR /usr/src/your-story
COPY . .
ENV LISTENER=0.0.0.0:3000
RUN cargo install --path .
CMD ["target/release/server"]
EXPOSE 3000