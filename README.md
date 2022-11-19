

### setup
    
    sudo apt-get install libpcap-dev


### minor

    k port-forward service/docker-registry 5000:5000


http://34.77.71.83:3000/metrics

prometheus:
    
    make port-forward-prom
    http://localhost:9090/