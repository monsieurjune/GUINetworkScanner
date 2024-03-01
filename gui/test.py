# import tkinter as tk
# from tkinter import ttk

# from utils import interface
# from utils import probe
# import subprocess
# import json

# # Global Variables

# interface_name = ""
# ip_addresses = []
# output_list = []


# # def populate_table(data_string):
# #     data = json.loads(data_string)

# #     names = [item["name"] for item in data]
# #     variable.set(names[0])

# #     for item in data:
# # ip_addresses.extend(item["addr_set"])


# # def probe_ip():
# #     global interface_name
# #     global output_list
# #     global ip_addresses
# #     inter = interface.interface_info()
# #     sets = probe.get_ip_subset(inter, interface_name, 256)
# #     ip_addresses = []

# #     for i in range(len(sets)):
# #         out = probe.probe_subset(sets, i)
# #         print(f"out : {out}")
# #         ip_addresses.append(out)
# #     print(f"here : {ip_addresses}")
# #     data = ip_addresses
# #     ip_addresses = []

# #     def ip_addresses(data):
# #         for item in data:
# #             ip_addresses.extend(item.get("addr_set", []))

# #     listbox.insert(tk.END, ip_addresses)


# def probe_ip():
#     global interface_name, ip_addresses  # Make variables accessible inside function
#     inter = interface.interface_info()
#     sets = probe.get_ip_subset(inter, interface_name, 256)

#     ip_addresses = []  # Probe and populate ip_addresses
#     for i in range(len(sets)):
#         out = probe.probe_subset(sets, i)
#         print(f"out : {out}")
#         ip_addresses.append(out)
#     print(f"here : {ip_addresses}")

#     update_listbox(ip_addresses)  # Update the listbox with results


# def update_listbox(data):
#     listbox.delete(0, tk.END)  # Clear existing items
#     for item in data:
#         extracted_ips = item.get("addr_set", [])
#         listbox.insert(tk.END, *extracted_ips)  # Insert IPs


# def get_ip_data(ip_addresses):
#     ip_list = ",".join(ip_addresses)
#     result = subprocess.Popen(
#         ".\\target\\release\\port_scanner.exe " + ip_list + ",fast,test",
#         stdout=subprocess.PIPE,
#     )

#     out, _ = result.communicate()
#     return str(out.decode("utf-8"))


# def get_result(ip_addresses):
#     return json.loads(get_ip_data(ip_addresses))


# def add_ip_address():
#     ip_address = ip_entry.get()
#     ip_addresses.append(ip_address)
#     ip_entry.delete(0, tk.END)

#     # Update the listbox
#     listbox.insert(tk.END, ip_address)


# def on_listbox_doubleclick(event):  # sourcery skip: use-named-expression
#     widget = event.widget
#     selection = widget.curselection()
#     if selection:
#         selected_ip = widget.get(selection[0])
#         if selected_ip not in selected_ip_addresses:  # Check for duplicates
#             selected_ip_addresses.append(selected_ip)
#             print("Selected IP Addresses:", selected_ip_addresses)


# def run_scan():
#     result = get_result(selected_ip_addresses)

#     # Display Result in a New Window
#     result_window = tk.Toplevel(window)
#     result_window.title("Scan Results")
#     tk.Label(result_window, text=result).pack()


# # GUI Setup
# window = tk.Tk()
# window.title("Two-Sided GUI")

# left_frame = tk.Frame(window)
# left_frame.pack(side="left", fill="both", expand=True)

# right_frame = tk.Frame(window)
# right_frame.pack(side="right", fill="both", expand=True)

# # --- Left Side ---
# # Input Section
# input_frame = tk.Frame(left_frame)
# input_frame.pack()
# tk.Label(input_frame, text="Enter IP Address:").pack(side="left")  # Added label
# ip_entry = tk.Entry(input_frame)
# ip_entry.pack(side="left")
# tk.Button(input_frame, text="Button 1", command=add_ip_address).pack(side="left")

# # Dropdown Section
# menu_frame = tk.Frame(left_frame)
# menu_frame.pack()
# tk.Label(menu_frame, text="Select Interface:").pack(side="left")  # Added label
# inter = interface.interface_info()
# options = interface.get_interfaces_name(inter)
# # options = []
# variable = tk.StringVar(left_frame)
# variable.set(options[0])
# interface_name = variable.get()
# tk.OptionMenu(menu_frame, variable, *options).pack(side="left")
# tk.Button(menu_frame, text="Button 2", command=probe_ip).pack(side="left")

# selected_ip_addresses = []  # Create the list to store selected IPs

# # Listbox
# listbox_frame = tk.Frame(left_frame)
# listbox_frame.pack()
# tk.Label(listbox_frame, text="IP Addresses:").pack()
# listbox = tk.Listbox(listbox_frame)
# listbox.pack()
# listbox.bind("<Double-Button-1>", on_listbox_doubleclick)

# # Radio Buttons
# radio_frame = tk.Frame(left_frame)
# radio_frame.pack()
# tk.Label(radio_frame, text="Scan Mode:").pack(side="left")
# var = tk.IntVar()
# tk.Radiobutton(radio_frame, text="Fast", variable=var, value="fast").pack(side="left")
# tk.Radiobutton(radio_frame, text="Full", variable=var, value="full").pack(side="left")

# tk.Button(left_frame, text="Button 3").pack()

# # --- Right Side ---
# tk.Label(right_frame, text="Data Table").pack()
# table = ttk.Treeview(right_frame)
# table["columns"] = ("address", "port", "description")  # Updated column names
# table.pack(expand=True, fill="both")

# # Update the headers
# table.heading("address", text="IP Address")
# table.heading("port", text="Port")
# table.heading("description", text="Description")

# tk.Button(right_frame, text="Button").pack()

# window.mainloop()


import tkinter as tk
from tkinter import ttk
from utils import interface
from utils import probe
import subprocess
import json

# Global Variables (Minimize if possible)
interface_name = ""
ip_addresses = []
output_list = []


def probe_ip():
    global interface_name, ip_addresses
    inter = interface.interface_info()
    sets = probe.get_ip_subset(inter, interface_name, 256)

    ip_addresses = []
    for i in range(len(sets)):
        out = probe.probe_subset(sets, i)
        print(f"out : {out}")
        ip_addresses.append(out)
    print(f"here : {ip_addresses}")

    update_listbox(ip_addresses)


def update_listbox(data):
    listbox.delete(0, tk.END)
    for item in data:
        extracted_ips = item.get("addr_set", [])
        listbox.insert(tk.END, *extracted_ips)


def get_ip_data(ip_addresses):
    ip_list = ",".join(ip_addresses)
    result = subprocess.Popen(
        ".\\target\\release\\port_scanner.exe " + ip_list + ",fast,test",
        stdout=subprocess.PIPE,
    )

    out, _ = result.communicate()
    return str(out.decode("utf-8"))


def get_result(ip_addresses):
    return json.loads(get_ip_data(ip_addresses))


def add_ip_address():
    ip_address = ip_entry.get()
    ip_addresses.append(ip_address)
    ip_entry.delete(0, tk.END)
    listbox.insert(tk.END, ip_address)


def on_listbox_doubleclick(event):
    widget = event.widget
    if selection := widget.curselection():
        selected_ip = widget.get(selection[0])
        if selected_ip not in selected_ip_addresses:
            selected_ip_addresses.append(selected_ip)
            print("Selected IP Addresses:", selected_ip_addresses)


def display_scan_results(ip_addresses):
    result = get_ip_data(ip_addresses)
    scan_data = json.loads(result)  # Convert result string to data structure
    clear_table()
    populate_table(scan_data)


def clear_table():
    for item in table.get_children():
        table.delete(item)


def populate_table(scan_results):
    data = json.loads(scan_results)  # Assuming scan_results is already JSON
    for item in data:
        table.insert(
            "", tk.END, values=(item["address"], item["port"], item["description"])
        )


# GUI Setup
window = tk.Tk()
window.title("Two-Sided GUI")

left_frame = tk.Frame(window)
left_frame.pack(side="left", fill="both", expand=True)

right_frame = tk.Frame(window)
right_frame.pack(side="right", fill="both", expand=True)

# --- Left Side ---
# Input Section
input_frame = tk.Frame(left_frame)
input_frame.pack()
tk.Label(input_frame, text="Enter IP Address:").pack(side="left")
ip_entry = tk.Entry(input_frame)
ip_entry.pack(side="left")
tk.Button(input_frame, text="Button 1", command=add_ip_address).pack(side="left")

# Dropdown Section
menu_frame = tk.Frame(left_frame)
menu_frame.pack()
tk.Label(menu_frame, text="Select Interface:").pack(side="left")
inter = interface.interface_info()
options = interface.get_interfaces_name(inter)
variable = tk.StringVar(left_frame)
variable.set(options[0])
interface_name = variable.get()
tk.OptionMenu(menu_frame, variable, *options).pack(side="left")
tk.Button(menu_frame, text="Button 2", command=probe_ip).pack(side="left")

selected_ip_addresses = []

# Listbox
listbox_frame = tk.Frame(left_frame)
listbox_frame.pack()
tk.Label(listbox_frame, text="IP Addresses:").pack()
listbox = tk.Listbox(listbox_frame)
listbox.pack()
listbox.bind("<Double-Button-1>", on_listbox_doubleclick)

# Scan Button
tk.Button(
    left_frame, text="Button 3", command=lambda: display_scan_results(ip_addresses)
).pack()

# --- Right Side ---
tk.Label(right_frame, text="Data Table").pack()
table = ttk.Treeview(right_frame)
table["columns"] = ("address", "port", "description")  # Updated column names
table.pack(expand=True, fill="both")

# Update the headers
table.heading("address", text="IP Address")
table.heading("port", text="Port")
table.heading("description", text="Description")

tk.Button(right_frame, text="Button").pack()

window.mainloop()
