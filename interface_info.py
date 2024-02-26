import subprocess
import json


def interface_info():
    result = subprocess.Popen(".\\target\\debug\\interface.exe", stdout=subprocess.PIPE)

    out, _ = result.communicate()

    json_object = json.loads(str(out.decode("utf-8")))

    return [interface["name"] for interface in json_object["interface"]]


def main():
    print(interface_info())


if __name__ == "__main__":
    main()
