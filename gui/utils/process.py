import subprocess
import json

def process(path):
    result = subprocess.Popen(r"./target/debug/interface", stdout=subprocess.PIPE)
        
    out, _ = result.communicate()
    exit_code = result.wait()
    
    if exit_code == 0:
        return json.loads(str(out.decode("utf-8")))
    return None    