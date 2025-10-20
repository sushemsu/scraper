
# Pulse Scraper

Pulse scraper is a rust program that will query a pulse api for /charts, format the data and post into a postgres database.

There current;y isn't a daemonmod, so to keep inserting data just run in a loop. I'll add this functionality later.

## Features

- Scrape charts from pulse api
- Store data in a postgres database
- Retain 30 days worth of metrics


## Usage/Examples

```bash
#!/usr/bin/env bash
while $(sleep 1); do
    ./scraper
done

```


## Environment Variables

To run this project, you will need to add the following environment variables to your .env file

- `POSTGRES_USER`
- `POSTGRES_PASSWORD`
- `POSTGRES_HOST`
- `PULSE_TOKEN`
- `PULSE_ADDRESS` where addr is "http://host:port/api/



## License

[GPL3](https://www.gnu.org/licenses/gpl-3.0.en.html#license-text)


## TODO
- Support Daemon Mode
- Support env vars rather than just .env file
- Support specifying an override for retention days
- Clean up ()? for error handling
