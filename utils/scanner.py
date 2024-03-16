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
    pass


if __name__ == "__main__":
    main()
