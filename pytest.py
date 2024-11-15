import hashlib
from datetime import datetime, timedelta

def generate_token(username: str, timestamp: int) -> str:
    """
    Generates a SHA-256 token based on the username and a given timestamp.
    """
    data = f"{username}{timestamp}"
    sha256_hash = hashlib.sha256(data.encode()).hexdigest()
    return sha256_hash

def get_unix_timestamp(date_str: str) -> int:
    """
    Converts a date string to a Unix timestamp.
    """
    dt = datetime.strptime(date_str, "%a, %d %b %Y %H:%M:%S %Z")
    return int(dt.timestamp())

def main():
    # Given UTC time: Fri, 15 Nov 2024 20:25:25 GMT
    reference_timestamp = get_unix_timestamp("Fri, 15 Nov 2024 20:25:25 GMT")
    username = "user123"
    
    print(f"Tokens for username '{username}' in the range of ±5 seconds:")

    # Generate tokens for all timestamps within ±5 seconds
    for offset in range(-5, 6):
        current_timestamp = reference_timestamp + offset
        token = generate_token(username, current_timestamp)

        # Convert the timestamp back to a human-readable format
        timestamp_str = datetime.utcfromtimestamp(current_timestamp).strftime("%Y-%m-%d %H:%M:%S UTC")
        print(f"Time: {timestamp_str}, Token: {token}")

if __name__ == "__main__":
    main()
