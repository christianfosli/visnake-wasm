FROM mcr.microsoft.com/dotnet/core/sdk:3.1 AS installer-env
ARG CRON_CLEANUP_SCHEDULE
ENV CRON_CLEANUP_SCHEDULE $CRON_CLEANUP_SCHEDULE

COPY . /src/dotnet-function-app
RUN cd /src/dotnet-function-app && \
    mkdir -p /home/site/wwwroot && \
    dotnet restore *.fsproj && \
    dotnet publish *.fsproj --output /home/site/wwwroot

# To enable ssh & remote debugging on app service change the base image to the one below
# FROM mcr.microsoft.com/azure-functions/dotnet:3.0-appservice
FROM mcr.microsoft.com/azure-functions/dotnet:3.0
ENV AzureWebJobsScriptRoot=/home/site/wwwroot \
    AzureFunctionsJobHost__Logging__Console__IsEnabled=true

COPY --from=installer-env ["/home/site/wwwroot", "/home/site/wwwroot"]
