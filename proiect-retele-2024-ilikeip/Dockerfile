FROM snisioi/retele:2021

RUN apt-get update && apt-get install -y iptables iproute2 libnetfilter-queue-dev
RUN apt-get update && apt-get install -y sudo
RUN pip3 install --upgrade python-iptables cython
RUN pip3 install plotly pandas requests 

RUN mkdir -p /app
# WORKDIR /app

COPY . ./app

# RUN mkdir -p /scripts
# COPY src/*.sh /scripts/

ENV PORT = 53
EXPOSE 53

# RUN sudo python3 ./app/src/traceroute.py
# CMD ["sudo", "python3", "./app/src/traceroute.py"]