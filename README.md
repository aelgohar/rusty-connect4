# Rusty Connect 4 Web App
Rusty Connect 4 is a full-stack project written completely in Rust.
It uses:
- `rocket` on the backend
- [`yew`](https://yew.rs/) for creating front-end webapps with `WebAssembly` (yew is a great component-based framework!) 
- [`yew-router`](https://github.com/yewstack/yew_router) for routing 
- [`stdweb`]( https://github.com/koute/stdweb) to provide Rust bindings for Web APIs

The backend is only used for requests and saving your progress. You can still play the game with just the frontend. 

![Gameplay](img/connect4.png)

## Dependencies

### cargo-web

The frontend is built using cargo-web, which can be installed with the following command.

```bash
cargo install cargo-web
```

### MongoDB

The backend uses MongoDB for storing game history. Instructions for how to install MongoDB can be found [here.](https://docs.mongodb.com/manual/installation/)

### nightly
[Rocket](https://rocket.rs/) requires the latest version of Rust nightly ([see here](https://rocket.rs/v0.4/guide/getting-started/)).
From the base directory of the project, run:
```bash
rustup override set nightly
```

## Run

The project can be built and ran from by executing the following commands from the base directory of the project.

To compile the frontend and generate the static files for the webpage:
```bash
(cd frontend && cargo web deploy)
```

To build and run the backend:
```bash
cargo run -p backend
```

The game should now be up and running and can be accessed by going to [localhost:8000](http://localhost:8000) in any web browser.

