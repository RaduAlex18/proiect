# import socket
# import traceback

# # socket de UDP
# udp_send_sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM, proto=socket.IPPROTO_UDP)

# # socket RAW de citire a răspunsurilor ICMP
# icmp_recv_socket = socket.socket(socket.AF_INET, socket.SOCK_RAW, socket.IPPROTO_ICMP)
# # setam timout in cazul in care socketul ICMP la apelul recvfrom nu primeste nimic in buffer
# icmp_recv_socket.settimeout(3)

# def traceroute(ip, port):
#     # setam TTL in headerul de IP pentru socketul de UDP
#     TTL = 64
#     udp_send_sock.setsockopt(socket.IPPROTO_IP, socket.IP_TTL, TTL)

#     # trimite un mesaj UDP catre un tuplu (IP, port)
#     udp_send_sock.sendto(b'salut', (ip, port))

#     # asteapta un mesaj ICMP de tipul ICMP TTL exceeded messages
#     # in cazul nostru nu verificăm tipul de mesaj ICMP
#     # puteti verifica daca primul byte are valoarea Type == 11
#     # https://tools.ietf.org/html/rfc792#page-5
#     # https://en.wikipedia.org/wiki/Internet_Control_Message_Protocol#Header
#     addr = 'done!'
#     try:
#         data, addr = icmp_recv_socket.recvfrom(63535)
#     except Exception as e:
#         print("Socket timeout ", str(e))
#         print(traceback.format_exc())
#     print (addr)
#     return addr

# '''
#  Exercitiu hackney carriage (optional)!
#     e posibil ca ipinfo sa raspunda cu status code 429 Too Many Requests
#     cititi despre campul X-Forwarded-For din antetul HTTP
#         https://www.nginx.com/resources/wiki/start/topics/examples/forwarded/
#     si setati-l o valoare in asa fel incat
#     sa puteti trece peste sistemul care limiteaza numarul de cereri/zi

#     Alternativ, puteti folosi ip-api (documentatie: https://ip-api.com/docs/api:json).
#     Acesta permite trimiterea a 45 de query-uri de geolocare pe minut.
# '''

# # exemplu de request la IP info pentru a
# # obtine informatii despre localizarea unui IP
# fake_HTTP_header = {
#                     'referer': 'https://ipinfo.io/',
#                     'user-agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/67.0.3396.79 Safari/537.36'
#                    }
# # informatiile despre ip-ul 193.226.51.6 pe ipinfo.io
# # https://ipinfo.io/193.226.51.6 e echivalent cu
# raspuns = requests.get('https://ipinfo.io/widget/193.226.51.6', headers=fake_HTTP_header)
# print (raspuns.json())

# # pentru un IP rezervat retelei locale da bogon=True
# raspuns = requests.get('https://ipinfo.io/widget/10.0.0.1', headers=fake_HTTP_header)
# print (raspuns.json())



import socket
import time
import requests
import folium


# ICMP = socket.getprotobyname('icmp')
# UDP = socket.getprotobyname('udp')

m = folium.Map(location=[0,0], zoom_start=2)

def get_ip_info(ip):
    url = f"http://ip-api.com/json/{ip}"
    response = requests.get(url)

    if response.status_code == 200:
        data = response.json()
        return data
    else:
        return {'error': 'Failed to retrieve data.'}



TIMEOUT = 0.2  # seconds
HOPS = 30

# Create raw sockets
receive_sock = socket.socket(socket.AF_INET, socket.SOCK_RAW, socket.IPPROTO_ICMP)
send_sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM, proto=socket.IPPROTO_UDP)

# Set timeout for receiving ICMP packets
receive_sock.settimeout(TIMEOUT)


def traceroute(ip: str, port: int, logFile: str):
    host = None
    try:
        bytes = ip.split(".")
        if not (len(bytes) == 4
        and 0 <= int(bytes[0]) <= 255
        and 0 <= int(bytes[1]) <= 255
        and 0 <= int(bytes[2]) <= 255
        and 0 <= int(bytes[3]) <= 255):
            host = ip
            ip = socket.gethostbyname(host)
    except:
        pass

    ttl = 1
    with open(logFile, "+a") as log:
        if host == None:
            print(f"\nTracerouting... {ip}")
        else:
            print(f"\nTracerouting... {host} ({ip})")

        coords = []

        while True:
            # Set TTL for UDP socket
            send_sock.setsockopt(socket.IPPROTO_IP, socket.IP_TTL, ttl)
            send_sock.sendto(b'hello', (ip, port))

            start_time = time.time()
            no_tries = 3
            success = False
            done = False
            while no_tries > 0:
                try:
                    packet, addr = receive_sock.recvfrom(port)
                    # print(packet)
                    success = True
                except socket.error:
                    no_tries -= 1
                    continue
                if addr[0] == ip:
                    done = True
                    break

            if success:
                end_time = time.time()
                try:
                    name = socket.gethostbyaddr(addr[0])[0]
                except:
                    name = addr[0]  # Use IP address if hostname lookup fails
                
                t = round((end_time - start_time) * 1000, 4)
                print(f"TTL: {ttl} Addr: {name} ({addr[0]}) Time: {t}ms Location: {get_ip_info(name).get('city', 'N/A')}, {get_ip_info(name).get('region', 'N/A')}, {get_ip_info(name).get('country', 'N/A')}, {get_ip_info(name).get('isp', 'N/A')}")
                log.write(f"TTL: {ttl} Addr: {name} ({addr[0]}) Time: {t}ms Location: {get_ip_info(name).get('city', 'N/A')}, {get_ip_info(name).get('region', 'N/A')}, {get_ip_info(name).get('country', 'N/A')}, {get_ip_info(name).get('isp', 'N/A')}\n")

                location_data = get_ip_info(name)
                lat = location_data.get('lat', None)
                lon = location_data.get('lon', None)

                if lat and lon:
                    folium.Marker([lat, lon], popup=f"{name} ({addr[0]})").add_to(m)
                    coords.append([lat, lon])

            else:
                print(f"TTL: {ttl} *  *  * Request timed out")

            ttl += 1
            if ttl > HOPS or done:
                break
                
        if coords:
            folium.PolyLine(coords, color="blue", weight=2.5, opacity=0.5).add_to(m)

        log.write("\n")

    m.save("traceroute_map.html")
    print("Traceroute completed.")



# hosts = ["google.com", "www.amazon.de", "parks.canada.ca", "www.airbnb.co.za", "www.japan.go.jp", "www.news.com.au"]
hosts = ["google.com"]
def main():
    for host in hosts:
        traceroute(host, 33434, "info_trace.txt")
    

main()




























"""
      ඞඞඞඞඞඞඞඞඞ
   ඞඞඞඞඞඞඞඞඞඞඞඞ
  ඞඞඞඞඞඞ      ඞඞඞ
ඞඞඞඞඞඞ          ඞඞ
ඞඞඞඞඞඞඞ       ඞඞඞ
ඞඞඞඞඞඞඞඞඞඞඞඞඞඞ
  ඞඞඞඞඞඞඞඞඞඞඞඞඞ
   ඞඞඞඞඞඞඞඞඞඞඞඞ
   ඞඞඞඞඞ     ඞඞඞඞ
   ඞඞඞඞඞ     ඞඞඞඞ
"""
