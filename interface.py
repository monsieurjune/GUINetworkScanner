import subprocess
import json

def interface_info():
    result = subprocess.Popen(r"./target/debug/interface", stdout=subprocess.PIPE)
        
    out, _ = result.communicate()
    exit_code = result.wait()
    
    if exit_code == 0:
        return json.loads(str(out.decode("utf-8")))
    return None    

def get_interfaces_name(info):
    return [interface["name"] for interface in info["interface"]]

def main():
    print(interface_info())

if __name__ == "__main__":
    main()
