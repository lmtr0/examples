
FROM gitpod/workspace-full

RUN sudo apt-get update -y && sudo apt-get install curl -y
RUN curl -L https://dot.net/v1/dotnet-install.sh | bash
