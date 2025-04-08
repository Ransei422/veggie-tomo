#!/bin/bash

# Install PostgreSQL and start the service
if ! command -v psql &> /dev/null; then
    echo "Installing PostgreSQL..."
    sudo apt update
    sudo apt install postgresql postgresql-contrib -y

    echo "Starting PostgreSQL service..."
    sudo systemctl start postgresql.service

    # Create user and database
    echo "Setting up PostgreSQL user and database..."

    sudo -u postgres psql <<EOF
    DO \$\$
    BEGIN
        IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'hogehoge') THEN
            CREATE ROLE hogehoge WITH LOGIN PASSWORD 'hogehoge';
        END IF;
    END
    \$\$;

    DO \$\$
    BEGIN
        IF NOT EXISTS (SELECT FROM pg_database WHERE datname = 'veggie-tomo') THEN
            CREATE DATABASE "veggie-tomo" OWNER hogehoge;
        END IF;
    END
    \$\$;
EOF
fi

# Check if rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Installing via rustup..."
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Ensure sqlx-cli is installed
if ! command -v sqlx &> /dev/null; then
    echo "Installing sqlx-cli..."
    cargo install sqlx-cli
    sqlx migrate run
fi

# Ensure cargo-watch is installed
if ! command -v cargo-watch &> /dev/null; then
    echo "Installing cargo-watch..."
    cargo install cargo-watch
fi

# Run app
cargo-watch -x run