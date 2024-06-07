import base64
from dnslib import DNSRecord, RR, QTYPE
from dnslib.server import DNSServer, DNSHandler, BaseResolver

# Decode data from the query
def decode_data(encoded_data):
    return base64.urlsafe_b64decode(encoded_data.encode()).decode()

class DNSQueryResolver(BaseResolver):
    def resolve(self, request, handler):
        query_name = str(request.q.qname)
        subdomain = query_name.split('.')[0]
        
        try:
            data = decode_data(subdomain)
            print(f"Received data: {data}")
        except Exception as e:
            print(f"Failed to decode data: {e}")
            data = "Error"

        reply = request.reply()
        reply.add_answer(RR(query_name, QTYPE.A, rdata=DNSRecord.A("127.0.0.1"), ttl=60))
        return reply

def main():
    resolver = DNSQueryResolver()
    server = DNSServer(resolver, port=53, address="104.248.36.68", tcp=True)
    server.start_thread()

    while True:
        pass

if __name__ == "__main__":
    main()
