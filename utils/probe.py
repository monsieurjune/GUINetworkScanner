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


def probe_subset(subset, inter_addr, passwd):
    cmd1 = [
        "echo",
        passwd,
    ]
    cmd2 = [
        "sudo",
        "-S",
        r"./target/release/probe",
        json.dumps(obj=subset),
        inter_addr
    ]

    print("Start : ", subset)
    result1 = subprocess.Popen(args=cmd1, stdout=subprocess.PIPE)
    output = subprocess.check_output(cmd2, stdin=result1.stdout)
    print("End : ", output)
    exit_code = result1.wait()

    # print(out)
    return (
        json.loads(str(object=output.decode(encoding="utf-8"))) if exit_code == 0 else None
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
    pass


if __name__ == "__main__":
    main()
