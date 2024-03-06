import subprocess
import json


def get_ip_subset(interface_info, interface_name, subset_no):
    cmd = [
        r"./target/release/ifsubset",
        json.dumps(interface_info),
        interface_name,
        str(subset_no),
    ]

    result = subprocess.Popen(cmd, stdout=subprocess.PIPE)
    out, _ = result.communicate()
    exit_code = result.wait()

    return (
        json.loads(str(object=out.decode(encoding="utf-8"))) if exit_code == 0 else None
    )


def probe_subset(set, i):
    subset = set["subset"][i]
    print(subset)
    cmd = [r"./target/release/probe", json.dumps(obj=subset)]

    result = subprocess.Popen(args=cmd, stdout=subprocess.PIPE)
    out, _ = result.communicate()
    exit_code = result.wait()

    return (
        json.loads(str(object=out.decode(encoding="utf-8"))) if exit_code == 0 else None
    )


# def probe(interface_info, interface_name):
#     name = ""

#     for interface in interface_info:
#         if interface["name"] == interface_name:
#             name = interface["name"]

#     subset = get_ip_subset(interface_info, name, 256)
#     if (subset == None):
#         return None

#     # for set in subset:


def main() -> None:
    str1 = get_ip_subset()

    # print(get_ip_subset(interface_info=str1, interface_name="wlan0", subset_no=256))
    probe_subset(set=str1, i="wlan0")


if __name__ == "__main__":
    main()
