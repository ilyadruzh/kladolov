# This file is responsible for configuring your application
# and its dependencies with the aid of the Mix.Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
use Mix.Config

config :kladolov_back,
  ecto_repos: [KladolovBack.Repo],
  generators: [binary_id: true]

# Configures the endpoint
config :kladolov_back, KladolovBackWeb.Endpoint,
  url: [host: "localhost"],
  secret_key_base: "ymwpbFywzuKw0KM6B78J3BedrzkwIJkUMQpwy5PL0iUtPYyal2lNa7hQ6RebbTg1",
  render_errors: [view: KladolovBackWeb.ErrorView, accepts: ~w(json), layout: false],
  pubsub_server: KladolovBack.PubSub,
  live_view: [signing_salt: "f1FYJqDc"]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{Mix.env()}.exs"
