defmodule KladolovBack.Repo do
  use Ecto.Repo,
    otp_app: :kladolov_back,
    adapter: Ecto.Adapters.Postgres
end
