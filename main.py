from tkinter import messagebox
from tkinter.ttk import Treeview, Scrollbar
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
    StringVar,
)
from utils import (
    interface,
    ip_address,
    probe
)


# ip_addresses: list[str] = [f"192.168.1.{n}" for n in range(1, 17)]

# for index, ip_address in enumerate(iterable=ip_addresses):
#     ip_address_treeview.insert(
#         parent="", index="end", iid=index, text=ip_address, tags=("unchecked",)
#     )

def probe_update():
    json_set = probe.get_ip_subset(
        network_interface_json, 
        select_interface.get(), 
        16
    )
    j = 0
    ip_address_treeview.delete(*ip_address_treeview.get_children())
    ip_address_treeview.update()
    for subset in json_set["subset"]:
        probe_result = probe.probe_subset(subset)
        for res in probe_result["addr_set"]:
            ip_address_treeview.insert(
                parent="", index="end", iid=j, text=res, tags=("unchecked")
            )
            ip_address_treeview.update()
            j += 1
    pass

def insert_ipaddr():
    member = ip_address_treeview.get_children()
    n = member.__len__()
    value = ip_address_entry.get()
    value_json = json.dumps({
        'name': select_interface.get(),
        'addr_set': [value]
        })
    result = probe.probe_subset(json.loads(value_json))
    if result is None:
        messagebox.showinfo(title="Error", message=f"{value} not found")
        return
    ip_address_treeview.insert(
        parent="", index="end", iid=n+1, text=result["addr_set"][0], tags=("unchecked")
    )

app = CTk()
app.geometry(geometry_string="960x720")
app.resizable(width=False, height=False)
app.title(string="Network Scanner Tool with Rust")
app.columnconfigure(index=0, weight=1)

left_frame = CTkFrame(
    master=app,
    width=320,
    height=720,
    bg_color="transparent",
    fg_color="#123456",
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
ip_address_entry.grid(row=1, column=0, padx=5, pady=0)

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
    font=("JetBrains Mono", 13),
    dropdown_font=("JetBrains Mono", 13),
    state="readonly",
    variable=StringVar(value=network_interfaces[0]),
    
)
network_interface_dropdown.grid(row=1, column=0, padx=5, pady=0)

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
    master=ip_address_list_frame, orient="vertical", command=ip_address_treeview.yview
)
ip_address_treeview.configure(yscroll=ip_address_list_scrollbar.set)
ip_address_list_scrollbar.grid(row=1, column=2, padx=0, pady=(5, 0), sticky="ns")

ip_addresses: list[str] = [f"192.168.1.{n}" for n in range(1, 17)]

for index, ip_address in enumerate(iterable=ip_addresses):
    ip_address_treeview.insert(
        parent="", index="end", iid=index, text=ip_address, tags=("unchecked",)
    )

ip_address_check_all_button = CTkButton(
    master=ip_address_list_frame,
    text="Select All",
    bg_color="transparent",
    width=100,
    height=25,
    command=lambda: print("select all"),
)
ip_address_check_all_button.grid(row=2, column=0, padx=5, pady=5)

ip_address_uncheck_all_button = CTkButton(
    master=ip_address_list_frame,
    text="Unselect All",
    bg_color="transparent",
    fg_color="#CD6464",
    width=100,
    height=25,
    command=lambda: print("unselect all"),
)
ip_address_uncheck_all_button.grid(row=2, column=1, padx=5, pady=5)

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
scan_option_label.grid(row=0, column=0, columnspan=2, pady=(30, 0), sticky="nsew")

scan_option_protocol_label = CTkLabel(
    master=scan_option_frame,
    text="Protocol:",
    font=("JetBrains Mono", 13, "bold"),
)
scan_option_protocol_label.grid(row=1, column=0, padx=15, pady=5, sticky="nsew")

scan_protocols: list[str] = ["TCP", "UDP", "ICMP"]
scan_variables: list = []
scan_option_protocol = CTkCheckBox(
    master=scan_option_frame,
    text="TCP",
    font=("JetBrains Mono", 12, "bold"),
)
scan_option_protocol.grid(row=2, column=0, padx=5, pady=5, sticky="w")

scan_option_mode_label = CTkLabel(
    master=scan_option_frame,
    text="Scan Mode:",
    font=("JetBrains Mono", 13, "bold"),
)
scan_option_mode_label.grid(row=1, column=1, padx=15, pady=5, sticky="nsew")

scan_option_mode_radio = CTkRadioButton(
    master=scan_option_frame, text="Quick", font=("JetBrains Mono", 12, "bold")
)
scan_option_mode_radio.grid(row=2, column=1, padx=5, pady=5, sticky="w")

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
    command=lambda: print("scan started!"),
)
scan_button.grid(row=3, column=0, columnspan=2, padx=5, pady=35, sticky="nsew")


right_frame = CTkFrame(master=app, width=640, height=720, fg_color="#E3E3E3")
right_frame.pack(side="right", fill="both", expand=True)

top_frame = CTkFrame(
    master=right_frame,
    width=640,
    height=60,
    bg_color="transparent",
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
    bg_color="transparent",
    fg_color="#FFFFFF",
    corner_radius=0,
)
scan_result_frame.pack(pady=25, padx=0, anchor="center")

scan_result_tree_columns = ("ip_address", "protocol", "description", "status")
scan_result_tree = Treeview(
    master=scan_result_frame,
    columns=scan_result_tree_columns,
    show="headings",
)

scan_result_tree.column("ip_address", width=100, minwidth=100, stretch=False)
scan_result_tree.column("protocol", width=60, minwidth=60, stretch=False)
scan_result_tree.column("description", width=300, minwidth=300, stretch=False)
scan_result_tree.column("status", width=50, minwidth=50, stretch=False)

scan_result_tree.heading("ip_address", text="IP Address", anchor="w")
scan_result_tree.heading("protocol", text="Protocol", anchor="w")
scan_result_tree.heading("description", text="Description", anchor="w")
scan_result_tree.heading("status", text="Status", anchor="w")


def item_selected(event):
    for selected_item in scan_result_tree.selection():
        item = scan_result_tree.item(selected_item)
        record = item["values"]
        messagebox.showinfo(title="Information", message=",".join(record))


scan_result_tree.bind("<<TreeviewSelect>>", item_selected)
scan_result_tree.grid(row=0, column=0, sticky="nsew")

scrollbar = Scrollbar(
    master=scan_result_frame, orient="vertical", command=scan_result_tree.yview
)
scan_result_tree.configure(yscrollcommand=scrollbar.set)
scrollbar.grid(row=0, column=1, sticky="ns")

scan_results = [(f"192.168.1.{n}", "TCP", "MSRPC", "Open") for n in range(1, 20)]
for scan_result in scan_results:
    scan_result_tree.insert(parent="", index="end", values=scan_result)


bottom_frame = CTkFrame(
    master=right_frame,
    width=640,
    height=90,
    bg_color="transparent",
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
