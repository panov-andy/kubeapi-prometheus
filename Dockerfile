FROM debian:buster-slim

ADD target/release/kubeapi-prometheus /app/
RUN ls -la /app

CMD ["/app/kubeapi-prometheus"]
