import json
import subprocess


def ip_address():
    result = subprocess.Popen(
        ".\\target\\debug\\port_scanner.exe 127.0.0.1,fast,test", stdout=subprocess.PIPE
    )

    out, _ = result.communicate()
    return str(out.decode("utf-8"))


def main():
    print(ip_address())


if __name__ == "__main__":
    main()
