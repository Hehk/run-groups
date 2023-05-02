build:
  cargo run
  @(cd ./web && npx tailwindcss -i ./src/styles.css -o ./build/styles.css --watch)
  