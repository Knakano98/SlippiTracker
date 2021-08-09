FROM rust:latest as rust-build

RUN mkdir build
COPY rust/src ./build/src
COPY rust/Cargo.toml ./build/Cargo.toml

WORKDIR /build

RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --release --target x86_64-unknown-linux-gnu


FROM python:3.8-slim-buster


RUN mkdir app

COPY --from=rust-build /build/target/x86_64-unknown-linux-gnu/release/libmylib.so /app/mylib.so

WORKDIR /app
ENV PYTHONUNBUFFERED=1
COPY requirements.txt requirements.txt
RUN pip3 install -r requirements.txt

COPY . .

#CMD ["python3" , "manage.py", "runserver","0.0.0.0:8000"]
