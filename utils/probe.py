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


def probe_subset(subset):
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
    str0 = '{"length":2,"interface":[{"index":0,"name":"wlan0","addr":"10.18.7.186","broadcast":"10.18.15.255","netmask":"255.255.240.0","mac":"A4:6B:B6:9C:22:2D"},{"index":1,"name":"docker0","addr":"172.17.0.1","broadcast":"172.17.255.255","netmask":"255.255.0.0","mac":"02:42:D8:91:83:1F"}]}'
    str00 = json.loads(str0)
    str1 = get_ip_subset(str00, "wlan0", 16)

    print(str1)
    # print(get_ip_subset(interface_info=str1, interface_name="wlan0", subset_no=256))
    # print(probe_subset(set=str1, i="wlan0")


if __name__ == "__main__":
    main()
