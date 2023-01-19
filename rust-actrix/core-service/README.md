```cmd
docker build -t core-service .
docker run -p 8080:8080 core-service


docker build -t core-service . -f Dockerfile.dev

```

### Status

- [] CI/CD: Connect Github Action && Deploy to ECS
- [] Fix: Dockerfile (Hot load in Development)
- [] Refactor: Group the core-service DB and Server into one project instead of Rust-workspace
- [] Feat: Connect each docker containers
