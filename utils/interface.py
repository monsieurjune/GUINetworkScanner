import subprocess
import json


def interface_info():
    result = subprocess.Popen(r"./target/release/interface", stdout=subprocess.PIPE)

    out, _ = result.communicate()
    exit_code = result.wait()

    return json.loads(str(out.decode("utf-8"))) if exit_code == 0 else None


def get_interfaces_name(info):
    return [interface["name"] for interface in info["interface"]]


def main():
    print(get_interfaces_name(interface_info))


if __name__ == "__main__":
    main()
