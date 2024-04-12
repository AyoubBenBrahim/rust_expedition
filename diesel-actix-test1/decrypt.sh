#!/bin/bash

SECRET_KEY=${SECRET_KEY}

if [[ -z "$SECRET_KEY" ]]; then
  echo "Error: Missing SECRET_KEY environment variable!"
  exit 1
fi

IN_FILE="encrypted.env"
OUT_FILE="env"

openssl enc -d -aes-256-cbc -pbkdf2 -k "$SECRET_KEY" -in "$IN_FILE" -out "$OUT_FILE"

if [[ $? -ne 0 ]]; then
  echo "Error: Decryption failed!"
  exit 1
fi

echo "Decrypted .env file successfully!"
