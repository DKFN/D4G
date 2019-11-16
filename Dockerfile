## Frontend
FROM node:12 AS frontend
COPY ./frontend/package.json ./frontend/package.json
COPY ./frontend/package-lock.json ./frontend/package-lock.json
RUN cd ./frontend && npm install --verbose
COPY ./frontend/src ./frontend/src
RUN cd ./frontend && npm run build --production

## Backend
FROM rustlang/rust:nightly AS backend
# This fake project allows caching of dependency with Docker layers and faster build times after first one
RUN rustup default nightly && rustup update
RUN USER=root cargo new --bin ./backend
COPY ./backend/Cargo.toml ./backend/
COPY ./backend/Cargo.lock ./backend/
RUN >&2 echo "/!\\ Rust dependencies are not in cache this process will take some minutes. /!\\"
RUN cd ./backend && cargo fetch
RUN cd ./backend && cargo build --release
RUN >&2 echo "\\o Rust dependencies are cached now /o"

# Swapping with real sources
COPY ./backend/src ./backend/src
RUN rm ./backend/target/release/deps/backend*
RUN cd backend && cargo build --release

# This stage allows us to compile all static assets in one file
FROM ubuntu as eipOptimizer
RUN apt-get update
RUN apt-get install -y --no-install-recommends apt-utils curl
RUN curl http://cdn.infra.tetel.in/d4g-skunkworks/bin/EIP --output /EIP
RUN chmod +x /EIP
COPY --from=frontend ./frontend/dist/ /fatfront
RUN /EIP /fatfront /

## Final image
FROM ubuntu
RUN mkdir /public
RUN mkdir /public/front
COPY --from=backend ./backend/target/release/backend /
COPY --from=eipOptimizer /index.html /public/front
# COPY --from=frontend ./frontend/dist/ /public/front
CMD ["/backend"]
