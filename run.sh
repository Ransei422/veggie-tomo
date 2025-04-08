#!/bin/bash

# Install openssl
if ! dpkg -s libssl-dev &> /dev/null; then
    echo "Installing libssl-dev..."
    sudo apt update
    sudo apt install libssl-dev -y
fi


# Install PostgreSQL and start it
if ! command -v psql &> /dev/null; then
    echo "Installing PostgreSQL..."
    sudo apt update
    sudo apt install postgresql postgresql-contrib -y

    echo "Starting PostgreSQL service..."
    sudo systemctl start postgresql.service
fi


# Check if the PostgreSQL role already exists
if ! sudo -u postgres psql -tAc "SELECT 1 FROM pg_roles WHERE rolname='hogehoge'" | grep -q 1; then
    echo "Setting up PostgreSQL user and database..."

    sudo -u postgres psql <<EOF
DO \$\$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'hogehoge') THEN
        CREATE ROLE hogehoge WITH LOGIN PASSWORD 'hogehoge';
    END IF;
END
\$\$;
EOF
fi


# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Installing via rustup..."
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source "$HOME/.cargo/env"
fi


# Check if sqlx-cli is installed and do migrations
if ! command -v sqlx &> /dev/null; then
    echo "Installing sqlx-cli..."
    cargo install sqlx-cli
    sqlx migrate run
fi


# Check if cargo-watch is installed
if ! command -v cargo-watch &> /dev/null; then
    echo "Installing cargo-watch..."
    cargo install cargo-watch
fi

# Run it baby
cargo-watch -x run
