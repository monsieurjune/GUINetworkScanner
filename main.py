import csv
import time;
from tkinter import messagebox
from tkinter.ttk import Treeview, Scrollbar, Style
from ttkwidgets import CheckboxTreeview
import json
from customtkinter import (
    CTk,
    CTkFrame,
    CTkLabel,
    CTkEntry,
    CTkButton,
    CTkComboBox,
    CTkCheckBox,
    CTkRadioButton,
    CTkToplevel,
    StringVar,
    IntVar,
)

passwd = r""

from utils import interface, scanner, probe, passwd

def find_addr_of_select_interface():
    select = network_interface_dropdown.get()
    for inter in network_interface_json["interface"]:
        if inter["name"] == select:
            return inter["addr"]
    return None

def probe_update():
    json_set = probe.get_ip_subset(
        interface_info=network_interface_json,
        interface_name=network_interface_dropdown.get(),
        subset_no=16,
    )
    ip_address_treeview.delete(*ip_address_treeview.get_children())
    ip_address_treeview.insert(
        parent="", index="end", iid=0, text="127.0.0.1", tags=("unchecked")
    )
    ip_address_treeview.update()

    j = 1
    select_ip = find_addr_of_select_interface()
    if select_ip is None:
        return
    for subset in json_set["subset"]:
        probe_result = probe.probe_subset(subset=subset, inter_addr=select_ip, passwd=passwd)
        for res in probe_result["addr_set"]:
            ip_address_treeview.insert(
                parent="", index="end", iid=j, text=res, tags=("unchecked")
            )
            ip_address_treeview.update()
            j += 1
        time.sleep(1)


def insert_ipaddr():
    member = ip_address_treeview.get_children()
    value = ip_address_entry.get()
    value_json = json.dumps({"name": network_interface_dropdown.get(), "addr_set": [value]})
    select_ip = find_addr_of_select_interface()

    result = probe.probe_subset(subset=json.loads(value_json), inter_addr=select_ip, passwd=passwd)
    if result is None:
        messagebox.showinfo(title="Error", message=f"{value} not found")
        return

    n = member.__len__()
    ip_address_treeview.insert(
        parent="",
        index="end",
        iid=n + 1,
        text=result["addr_set"][0],
        tags=("unchecked"),
    )


def scan():
    checked_iter = ip_address_treeview.get_checked()
    global scan_results

    mode = scan_mode_var.get()
    select_ip = find_addr_of_select_interface()
    if mode == 1:
        mode_str = "fast"
    elif mode == 2:
        mode_str = "full"
    else:
        mode_str = "No"

    scan_results.clear()
    for checked in checked_iter:
        check_ip = ip_address_treeview.item(item=checked)["text"]
        result_json = scanner.tcp_scan(
            ipaddr=check_ip,
            inter_addr=select_ip,
            tcp="tcp",
            mode=mode_str,
            passwd=passwd
        )
        scan_results.append(result_json)
    scan_result_tree.delete(*scan_result_tree.get_children())
    insert_data()


app = CTk()
app.geometry(geometry_string="960x720")
app.resizable(width=False, height=False)
app.title(string="Network Scanner Tool with Rust")


def login(window):
    password = password_entry.get()
    state = passwd.check_password(password)
    window.destroy()
    if state != 0:
        exit(255)

login_password = ""
top_level = CTkToplevel(master=app)
top_level.geometry(geometry_string="220x80")
top_level.resizable(width=False, height=False)
top_level.title(string="Login")
top_level.attributes("-topmost", True)

password_label = CTkLabel(master=top_level, text="Password")
password_label.grid(row=0, column=0, padx=5, pady=5, sticky="w")

password_entry = CTkEntry(master=top_level, placeholder_text="Enter Password...")
password_entry.grid(row=0, column=1, padx=5, pady=5, sticky="w")

login_button = CTkButton(master=top_level, text="Login", command=lambda: login(window=top_level))
login_button.grid(row=2, column=0, columnspan=2, padx=5, pady=5, sticky="nsew")


left_frame = CTkFrame(
    master=app,
    width=320,
    height=720,
    bg_color="#123456",
    fg_color="#123456",
    corner_radius=0,
    border_width=0,
)
left_frame.pack(side="left", fill="both", expand=True)

ip_address_section = CTkFrame(
    master=left_frame, bg_color="transparent", fg_color="#123456"
)
ip_address_section.pack()

ip_address_label = CTkLabel(
    master=ip_address_section,
    text="IP Address:",
    font=("JetBrains Mono", 13, "bold"),
)
ip_address_label.grid(row=0, column=0, padx=5, pady=(10, 0), sticky="w")

ip_address_entry = CTkEntry(
    master=ip_address_section,
    placeholder_text="Enter IP Address...",
    font=("JetBrains Mono", 11, "bold"),
    width=150,
    height=30,
)
ip_address_entry.grid(row=1, column=0, padx=5, pady=0, sticky="w")

add_ip_button = CTkButton(
    master=ip_address_section,
    text="Add IP Address",
    font=("JetBrains Mono", 12, "bold"),
    width=120,
    height=30,
    corner_radius=15,
    fg_color="#6B9FDC",
    command=lambda: insert_ipaddr(),
)
add_ip_button.grid(row=1, column=1, padx=5)

network_interface_section = CTkFrame(
    master=left_frame, bg_color="transparent", fg_color="#123456"
)
network_interface_section.pack()

network_interface_label = CTkLabel(
    master=network_interface_section,
    text="Network Interface:",
    font=("JetBrains Mono", 13, "bold"),
)
network_interface_label.grid(row=0, column=0, padx=5, pady=(10, 0), sticky="w")

network_interface_json = interface.interface_info()
network_interfaces: list[str] = interface.get_interfaces_name()
select_interface = StringVar(value=network_interfaces[0])
network_interface_dropdown = CTkComboBox(
    master=network_interface_section,
    values=network_interfaces,
    width=150,
    height=30,
    font=("JetBrains Mono", 11),
    dropdown_font=("JetBrains Mono", 13),
    state="readonly",
    variable=select_interface
)
network_interface_dropdown.grid(row=1, column=0, padx=5, pady=0, sticky="w")

probe_button = CTkButton(
    master=network_interface_section,
    text="Probe",
    font=("JetBrains Mono", 12, "bold"),
    width=120,
    height=30,
    corner_radius=15,
    fg_color="#A469C4",
    hover_color="#8069C4",
    command=lambda: probe_update(),
)
probe_button.grid(row=1, column=1, padx=5)

ip_address_list_frame = CTkFrame(
    master=left_frame, bg_color="transparent", fg_color="#123456"
)
ip_address_list_frame.pack()

ip_address_list_label = CTkLabel(
    master=ip_address_list_frame,
    text="IP Address List",
    font=("JetBrains Mono", 16, "bold"),
    anchor="center",
    compound="center",
)
ip_address_list_label.grid(row=0, column=0, columnspan=2, pady=(30, 0), sticky="nsew")

ip_address_treeview = CheckboxTreeview(
    master=ip_address_list_frame,
    show="tree",  # hide tree headings
)
ip_address_treeview.grid(row=1, column=0, columnspan=2, pady=(5, 0))

ip_address_list_scrollbar = Scrollbar(
    master=ip_address_list_frame,
    orient="vertical",
    command=ip_address_treeview.yview,
    style="TScrollbar",
    cursor="arrow",
)
ip_address_treeview.configure(yscroll=ip_address_list_scrollbar.set)
ip_address_list_scrollbar.grid(row=1, column=2, padx=0, pady=(5, 0), sticky="ns")

ip_addresses: list[str] = ["127.0.0.1"]
selected_ip_addresses: list[str] = []

for index, ip_address in enumerate(iterable=ip_addresses):
    ip_address_treeview.insert(
        parent="", index="end", iid=index, text=ip_address, tags=("unchecked",)
    )


def select_all():
    for child_iid in ip_address_treeview.get_children():
        ip_address_treeview.item(item=child_iid, tags=("checked",))


def unselect_all():
    for child_iid in ip_address_treeview.get_children():
        ip_address_treeview.item(item=child_iid, tags=("unchecked",))


def handle_checkbox_change(event):
    item_iid = ip_address_treeview.identify_row(event.y)
    item_text = ip_address_treeview.item(item=item_iid, option="text")

    if ip_address_treeview.tag_has(tagname="checked", item=item_iid):
        print(f"Selected IP Address: {item_text}")
        if item_text not in selected_ip_addresses:
            selected_ip_addresses.append(item_text)
    elif item_text in selected_ip_addresses:
        selected_ip_addresses.remove(item_text)


ip_address_check_all_button = CTkButton(
    master=ip_address_list_frame,
    text="Select All",
    bg_color="transparent",
    width=90,
    height=25,
    command=select_all,
)
ip_address_check_all_button.grid(row=2, column=0, padx=5, pady=5)

ip_address_uncheck_all_button = CTkButton(
    master=ip_address_list_frame,
    text="Unselect All",
    bg_color="transparent",
    fg_color="#CD6464",
    width=90,
    height=25,
    command=unselect_all,
)
ip_address_uncheck_all_button.grid(row=2, column=1, padx=5, pady=5)

ip_address_treeview.bind(sequence="<<TreeviewSelect>>", func=handle_checkbox_change)

scan_option_frame = CTkFrame(
    master=left_frame, bg_color="transparent", fg_color="#123456"
)
scan_option_frame.pack()
scan_option_frame.columnconfigure(index=0, weight=2)

scan_option_label = CTkLabel(
    master=scan_option_frame,
    text="Scan Option",
    font=("JetBrains Mono", 16, "bold"),
)
scan_option_label.grid(row=0, column=0, columnspan=2, pady=(15, 0), sticky="nsew")

scan_option_mode_label = CTkLabel(
    master=scan_option_frame,
    text="Scan Mode:",
    font=("JetBrains Mono", 13, "bold"),
)
scan_option_mode_label.grid(row=1, column=0, padx=15, pady=5, sticky="nsew")

scan_mode_var = IntVar()
scan_mode_fast = CTkRadioButton(
    master=scan_option_frame,
    text="Fast",
    font=("JetBrains Mono", 12, "bold"),
    variable=scan_mode_var,
    value=1,
)
scan_mode_fast.grid(row=2, column=0, padx=(50, 0), pady=5, sticky="nsew")
scan_mode_fast.select()

scan_mode_full = CTkRadioButton(
    master=scan_option_frame,
    text="Full",
    font=("JetBrains Mono", 12, "bold"),
    variable=scan_mode_var,
    value=2,
)
scan_mode_full.grid(row=3, column=0, padx=(50, 0), pady=5, sticky="nsew")

scan_button = CTkButton(
    master=scan_option_frame,
    text="Scan",
    text_color="#000000",
    font=("JetBrains Mono", 21, "bold"),
    width=150,
    height=50,
    corner_radius=25,
    bg_color="transparent",
    fg_color="#FEDCBA",
    command=lambda: scan(),
)
scan_button.grid(row=4, column=0, columnspan=2, padx=5, pady=15, sticky="nsew")


right_frame = CTkFrame(
    master=app,
    width=640,
    height=720,
    bg_color="#E3E3E3",
    fg_color="#E3E3E3",
)
right_frame.pack(side="right", fill="both", expand=True)

top_frame = CTkFrame(
    master=right_frame,
    width=640,
    height=60,
    bg_color="#ABCDEF",
    fg_color="#ABCDEF",
    corner_radius=0,
)
top_frame.pack(side="top", fill="x")

top_label = CTkLabel(
    master=top_frame,
    text="Scan Result",
    text_color="#000000",
    font=("JetBrains Mono", 25, "bold"),
)
top_label.pack(pady=10, padx=10, anchor="center")

scan_result_frame = CTkFrame(
    master=right_frame,
    width=620,
    height=500,
    bg_color="#FFFFFF",
    fg_color="#FFFFFF",
    corner_radius=0,
)
scan_result_frame.pack(pady=25, padx=0, anchor="center")

treeview_style = Style()
treeview_style.configure(
    style="Treeview",
    font=("JetBrains Mono", 9),
    rowheight=25,
)

#! TODO: This is just a dummy data, replace it with actual scan results
scan_results = [
    # {
    #     "ipaddr": "127.0.0.1",
    #     "tcp_ports": [
    #         {"port": 135, "status": "Open"},
    #         {"port": 445, "status": "Open"},
    #         {"port": 1462, "status": "Open"},
    #         {"port": 2179, "status": "Open"},
    #         {"port": 4808, "status": "Open"},
    #         {"port": 5040, "status": "Open"},
    #         {"port": 5432, "status": "Open"},
    #         {"port": 8974, "status": "Open"},
    #         {"port": 9080, "status": "Open"},
    #         {"port": 9100, "status": "Open"},
    #         {"port": 9180, "status": "Open"},
    #     ],
    # },
    # {
    #     "ipaddr": "192.168.1.1",
    #     "tcp_ports": [
    #         {"port": 135, "status": "Open"},
    #         {"port": 445, "status": "Open"},
    #         {"port": 1462, "status": "Open"},
    #         {"port": 2179, "status": "Open"},
    #         {"port": 9080, "status": "Open"},
    #         {"port": 9100, "status": "Open"},
    #         {"port": 9180, "status": "Open"},
    #     ],
    # }
]

scan_result_tree = Treeview(master=scan_result_frame, style="Treeview")
scan_result_tree.grid(row=0, column=0, sticky="nsew")

scan_result_tree["columns"] = ("port", "protocol", "description")
scan_result_tree.column(column="#0", width=110, minwidth=110, stretch=False)
scan_result_tree.column(column="port", width=60, minwidth=60)
scan_result_tree.column(column="protocol", width=60, minwidth=60)
scan_result_tree.column(column="description", width=270, minwidth=270)

scan_result_tree.heading(column="#0", text="IP Address", anchor="c")
scan_result_tree.heading(column="port", text="Port", anchor="c")
scan_result_tree.heading(column="protocol", text="Protocol", anchor="c")
scan_result_tree.heading(column="description", text="Description", anchor="c")


port_descriptions = {}
with open(file=r"ports_list/tcp.csv", mode="r") as port_list:
    reader = csv.reader(port_list)
    next(reader)
    for row in reader:
        protocol, port, description = row
        port_descriptions[port] = description


def insert_data():
    for i, ip_address in enumerate(iterable=scan_results):
        desp = "Open Port(s) : " + str(ip_address["tcp_ports"].__len__())
        ip_iid = scan_result_tree.insert(
            parent="", 
            index="end",
            text=ip_address["ipaddr"],
            value=("", "", desp)
        )
        
        for j, port_data in enumerate(iterable=ip_address["tcp_ports"]):
            port = str(port_data["port"])
            description = port_descriptions.get(port, "Unknown")
            scan_result_tree.insert(
            parent=ip_iid, index="end", values=(port, protocol, description)
        )

insert_data()


def item_selected(event):
    for selected_item in scan_result_tree.selection():
        item = scan_result_tree.item(item=selected_item)
        record = item["values"]
        messagebox.showinfo(title="Scan Result", message=", ".join(record))


# scan_result_tree.bind("<<TreeviewSelect>>", func=item_selected)

scrollbar = Scrollbar(
    master=scan_result_frame,
    orient="vertical",
    command=scan_result_tree.yview,
    style="TScrollbar",
    cursor="arrow",
)
scan_result_tree.configure(yscrollcommand=scrollbar.set)
scrollbar.grid(row=0, column=1, sticky="ns")

# scan_results = []
# for scan_result in scan_results:
#     scan_result_tree.insert(parent="", index="end", values=scan_result)


bottom_frame = CTkFrame(
    master=right_frame,
    width=640,
    height=90,
    bg_color="#ABCDEF",
    fg_color="#ABCDEF",
    corner_radius=0,
)
bottom_frame.pack(side="bottom", fill="x")

stop_button = CTkButton(
    master=bottom_frame,
    text="Stop",
    text_color="#000000",
    font=("JetBrains Mono", 25, "bold"),
    bg_color="transparent",
    fg_color="#CD6464",
    corner_radius=50,
    command=lambda: print("scan stopped!"),
)
stop_button.pack(pady=10, padx=10, anchor="center")


app.mainloop()
