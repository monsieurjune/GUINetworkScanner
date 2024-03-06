import json
import subprocess


def get_ip_data(ip_address, mode):
    result = subprocess.run(
        ["./target/release/port_scanner.exe", f"{ip_address},{mode},test"],
        capture_output=True,
        text=True,
    )
    return result.stdout


def get_result():
    ip_addressed: list[str] = []
    return json.loads(get_ip_data("127.0.0.1", "fast"))


def main():
    result = get_result()
    print(result)
    with open("result.txt", "w") as f:
        json.dump(result, f, indent=4)


if __name__ == "__main__":
    main()
