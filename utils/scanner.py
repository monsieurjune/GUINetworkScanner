import json
import subprocess

def tcp_scan(ipaddr, mode):
    cmd = [
        r"./target/release/port",
        f"{ipaddr},{mode},No"
    ]

    result = subprocess.Popen(cmd, stdout=subprocess.PIPE)
    out, _ = result.communicate()
    exit_code = result.wait()

    return (
        json.loads(str(object=out.decode(encoding="utf-8"))) if exit_code == 0 else None
    )


def main():
    # result = get_result()
    # print(result)
    # with open("result.txt", "w") as f:
    #     json.dump(result, f, indent=4)
    pass


if __name__ == "__main__":
    main()
