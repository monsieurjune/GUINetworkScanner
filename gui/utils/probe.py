import subprocess
import json

def get_ip_subset(interface_info, interface_name, subset_no):
    cmd = [r"./target/debug/ifsubset", json.dumps(interface_info), interface_name, str(subset_no)]
    
    result = subprocess.Popen(cmd, stdout=subprocess.PIPE)
    out, _ = result.communicate()
    exit_code = result.wait()
    
    if exit_code == 0:
        return json.loads(str(out.decode("utf-8")))
    return None

def probe_subset(set, i):
    subset = set[i]
    cmd = [r"./target/debug/probe", json.dumps(subset)]
    
    result = subprocess.Popen(cmd, stdout=subprocess.PIPE)
    out, _ = result.communicate()
    exit_code = result.wait()
    
    if exit_code == 0:
        return out
    return None

# def probe(interface_info, interface_name):
#     name = ""
    
#     for interface in interface_info:
#         if interface["name"] == interface_name:
#             name = interface["name"]
    
#     subset = get_ip_subset(interface_info, name, 256)
#     if (subset == None):
#         return None
    
#     # for set in subset:
        

def main():
    str1 = interface_info()
    # print(str1)
    print(get_ip_subset(str1, "wlan0", 256))
    probe(str1, "wlan0")


if __name__ == "__main__":
    main()
