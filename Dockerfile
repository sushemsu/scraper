from rust:slim
workdir /usr/src/scraper
copy . .
run apt-get update -yqq && apt-get install openssl libssl-dev pkg-config -yqq
run cargo install --path .

from debian:latest
workdir /app
copy --from=0 /usr/src/scraper/target/release/scraper /app/scraper
cmd ["bash", "-c", "while $(sleep 1); do ./scraper; done"]
