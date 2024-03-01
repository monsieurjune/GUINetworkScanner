import json
import subprocess


def get_ip_data():
    result = subprocess.Popen(
        ".\\target\\release\\port_scanner.exe 127.0.0.1,fast,test",
        stdout=subprocess.PIPE,
    )

    out, _ = result.communicate()
    return str(out.decode("utf-8"))


def get_result():
    return json.loads(get_ip_data())


def main():
    print(get_result())


if __name__ == "__main__":
    main()
