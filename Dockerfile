## Frontend
FROM node:12 AS frontend
WORKDIR /frontend
COPY ./frontend/ .
RUN npm install --verbose && \
    npm run build --production

## Backend
FROM rustlang/rust:nightly AS backend
# This fake project allows caching of dependency with Docker layers and faster build times after first one
RUN rustup default nightly && \
    rustup update && \
    USER=root cargo new --bin /backend

WORKDIR /backend
COPY ./backend/Cargo.* ./
RUN >&2 echo "/!\\ Rust dependencies are not in cache this process will take some minutes. /!\\"; \
    cargo fetch; \
    cargo build --release; \
    >&2 echo "\\o Rust dependencies are cached now /o"

# Swapping with real sources
COPY ./backend/src .
RUN rm ./backend/target/release/deps/backend*; \
    cargo build --release

# This stage allows us to compile all static assets in one file
FROM ubuntu as eipOptimizer
COPY --from=frontend /frontend/dist/ /fatfront
RUN apt-get update && \
    apt-get install -y --no-install-recommends apt-utils curl && \
    curl http://cdn.infra.tetel.in/d4g-skunkworks/bin/EIP --output /EIP && \
    chmod +x /EIP && \
    /EIP /fatfront /

## Final image
FROM ubuntu
COPY --from=eipOptimizer /index.html /public/front
# COPY --from=frontend /frontend/dist/ /public/front
COPY --from=backend /backend/target/release/backend /

ENTRYPOINT [ "/backend" ]
