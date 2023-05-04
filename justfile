build:
  (cd ./web && cargo run && npx tailwindcss -i ./src/styles.css -o ./build/styles.css)
  
