import json
import subprocess

# port tcp fast 192.168.1.1 interface 

def tcp_scan(ipaddr, inter_addr, tcp, mode, passwd):
    cmd1 = [
        "echo",
        passwd
    ]
    cmd2 = [
        "sudo",
        "-S",
        r"./target/release/port",
        tcp,
        mode,
        ipaddr,
        inter_addr
    ]

    result1 = subprocess.Popen(args=cmd1, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
    output = subprocess.check_output(cmd2, stdin=result1.stdout, stderr=subprocess.DEVNULL)
    exit_code = result1.wait()

    return (
        json.loads(str(object=output.decode(encoding="utf-8"))) if exit_code == 0 else None
    )


def main():
    # result = get_result()
    # print(result)
    # with open("result.txt", "w") as f:
    #     json.dump(result, f, indent=4)
    pass


if __name__ == "__main__":
    main()
