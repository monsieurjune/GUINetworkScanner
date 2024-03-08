import subprocess

def check_password(check_passwd):
    cmd = [
        "sudo",
        "-S",
        "ls"
    ]

    try:
        result1 = subprocess.Popen(args=cmd1, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
        output = subprocess.check_output(cmd2, stdin=result1.stdout, stderr=subprocess.DEVNULL)
        exit_code = result1.wait()
        return exit_code
    except:
        return 255