# Rusty Connect 4 Web App

## Dependencies

### cargo-web

The frontend is built using cargo-web, which can be installed with the following command.

```bash
cargo install cargo-web
```

### MongoDB

The backend uses MongoDB for storing game history. Instructions for how to install MongoDB can be found [here.](https://docs.mongodb.com/manual/installation/)

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

