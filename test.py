# import tkinter as tk
# from tkinter import ttk

# root = tk.Tk()

# tree = ttk.Treeview(root, columns=("protocol", "description", "status"), height=10)

# tree.heading("#0", text="IP Address")
# tree.column("#0", width=200, stretch=tk.NO)

# tree.heading("protocol", text="Protocol")
# tree.column("protocol", width=150, stretch=tk.NO)

# tree.heading("description", text="Description")
# tree.column("description", width=150, stretch=tk.NO)

# tree.heading("status", text="Status")
# tree.column("status", width=150, stretch=tk.NO)

# tree.grid(row=0, column=0, sticky="nsew")

# data = [
#     (
#         "127.0.0.1",
#         "",
#         "",
#         [("135", "5 KB", "12/05/2023"), ("Personal", "2 KB", "12/03/2023")],
#     ),
#     ("Pictures", "", "", []),
#     ("Music", "", "", []),
# ]


# def insert_items(data, parent=""):
#     for item in data:
#         iid = tree.insert(parent, tk.END, text=item[0], values=item[1:])
#         if len(item) > 3 and isinstance(item[3], list):
#             insert_items(item[3], iid)


# insert_items(data)

# root.mainloop()


import tkinter as tk
from tkinter import ttk

scan_results = {
    "ipaddr": "127.0.0.1",
    "tcp_ports": [
        {
            "port": 135,
            "status": "Open",
        },
        {
            "port": 445,
            "status": "Open",
        },
    ],
}

# Window Setup
root = tk.Tk()
root.title("Scan Results")

# Treeview
scan_result_tree = ttk.Treeview(root)
scan_result_tree.pack()

# Columns
scan_result_tree["columns"] = "status"
scan_result_tree.column(
    "#0", width=110, minwidth=110, stretch=False
)  # IP Address Column
scan_result_tree.column("status", width=80, minwidth=80)

# Headings
scan_result_tree.heading("#0", text="IP Address")
scan_result_tree.heading("status", text="Status")

# Insert IP Address as Parent
ip_address = scan_results["ipaddr"]
ip_iid = scan_result_tree.insert(parent="", index="end", text=ip_address)

# Insert Ports as Children
for port_data in scan_results["tcp_ports"]:
    port = str(port_data["port"])
    status = port_data["status"]
    scan_result_tree.insert(parent=ip_iid, index="end", text=port, values=(status))

# Start the GUI
root.mainloop()
