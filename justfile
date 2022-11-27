alias b := build
alias r := run
alias s := serve

# build main
build:
  tailwindcss -o ./tailwind.css
  cargo build

# run it
run:
  pnpx tailwindcss -o ./tailwind.css
  cargo run

# serve a dev server with hmr
serve:
  pnpx tailwindcss -o ./tailwind.css
  trunk serve
