# README

## Goals

**Have fun!**

### Features

- Create list of items with custom fields
- User accounts
- Groups
- Share lists with groups
- Barcode scanner
- Notifications
- Recipes
- Suggestions based on current stock with AI

### Tech

- WebAPI with Swagger **Tokio**
  - [] Create a WebAPI project
  - [] Add Swagger to the project
  - [] Add a controller with a GET method
- Database **Postgres**
  - [] Create a database
  - [] SQLx or Diesel
- WebApp **Yew**
  - [] Create a WebApp project
  - [] Add a page with a button that calls the WebAPI
- Docker
  - [] Create a Dockerfile for the WebAPI
  - [] Create a Dockerfile for the WebApp
  - [] Create a Dockerfile for the Database
  - [] Create a docker-compose file
  - For running the project
- OpenTofu
  - [] Add OpenTofu to the WebAPI
  - [] Add OpenTofu to the WebApp
  - [] Add OpenTofu to the Database
  - For managing secrets and configuration
- CI/CD
  - [] GitHub Actions
- Testing
  - [] Unit tests
  - [] Integration tests
  - [] End-to-end tests
  - For the WebAPI and WebApp
- Monitoring and Logging **OpenTelemetry**
  - [] Prometheus
  - [] Grafana
  - [] Loki
  - [] Jaeger
  - [] Add OpenTelemetry to the project
  - For metrics, logs, and traces
- SSO **Keycloak**
  - [] Add Keycloak to the project
  - For user accounts
- Feature Flags **LaunchDarkly**
  - [] Add LaunchDarkly to the project
  - For A/B testing and feature flags
- Caching **Redis**
  - [] Add Redis to the project
  - For caching lists
- Messaging **Kafka**
  - [] Add Kafka to the project
  - For notifications
