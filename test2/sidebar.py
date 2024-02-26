import tkinter as tk
import customtkinter as ctk

import subprocess


class Sidebar:
    def __init__(self, master):
        self.master = master
        self.create_sidebar()

        self.ip_addresses = []

    def create_sidebar(self):
        sidebar_frame = ctk.CTkFrame(master=self.master, width=330)
        sidebar_frame.grid(row=1, column=0, sticky="nsew")
        sidebar_frame.grid_columnconfigure(0, weight=0)

        input_frame = ctk.CTkFrame(master=sidebar_frame)
        input_frame.grid(row=0, column=0, columnspan=2, sticky="nsew", pady=5)

        tk.Label(master=input_frame, text="IP Address").grid(
            row=0,
            column=0,
            padx=5,
            pady=2,
        )
        input_entry = ctk.CTkEntry(
            master=input_frame,
            width=200,
            placeholder_text=("Enter IP Address ..."),
        )
        input_entry.grid(row=1, column=0, padx=5, pady=2)

        def ip_address_list():
            subprocess.run(".\target\debug\interface.exe", shell=True, check=True)

            for widget in self.table_frame.winfo_children():
                widget.destroy()

            header_labels = ["#", "IP Address", "MAC Address"]
            for col_num, col_name in enumerate(header_labels):
                label = ctk.CTkLabel(
                    self.table_frame,
                    text=col_name,
                    font=ctk.CTkFont(weight="bold"),
                )
                label.grid(row=0, column=col_num, padx=5, pady=2)

            for row_num, row_data in enumerate(self.ip_addresses):
                for col_num, (key, value) in enumerate(row_data.items()):
                    label = ctk.CTkLabel(self.table_frame, text=str(value))
                    label.grid(row=row_num + 1, column=col_num, padx=5, pady=2)

        def add_ip_address():
            new_ip_address = input_entry.get()
            input_entry.delete(0, tk.END)

            if new_ip_address:
                new_row = {
                    "#": len(self.master.ip_addresses) + 1,
                    "IP Address": new_ip_address,
                    "MAC Address": "KUAY:HEEE:TADD",
                }

            self.master.ip_addresses.append(new_row)
            self.master.ip_address_list()

        input_button = ctk.CTkButton(
            master=input_frame, text="Add IP Address", command=add_ip_address
        )
        input_button.grid(row=1, column=1, sticky="nsew", padx=5, pady=2)

        dropdown_row = ctk.CTkFrame(master=sidebar_frame)
        dropdown_row.grid(row=1, column=0, columnspan=2, sticky="nsew", pady=5)

        self.interface_list = [
            "Ethernet",
            "Wireless LAN",
            "Bluetooth",
            "Loopback",
        ]
        variable = tk.StringVar(sidebar_frame)
        variable.set(self.interface_list[0])

        dropdown = tk.OptionMenu(dropdown_row, variable, *self.interface_list)
        dropdown.grid(row=0, column=0, sticky="nsew", padx=5)
        dropdown_button = ctk.CTkButton(master=dropdown_row, text="Probe")
        dropdown_button.grid(row=0, column=1, sticky="nsew", padx=5)

        input_frame.grid_columnconfigure(0, weight=1)
        input_frame.grid_columnconfigure(1, weight=1)
        dropdown_row.grid_columnconfigure(0, weight=1)
        dropdown_row.grid_columnconfigure(1, weight=1)

        self.table_frame = ctk.CTkFrame(master=sidebar_frame)
        self.table_frame.grid(row=2, column=0, columnspan=2, sticky="nsew", pady=10)
        self.master.ip_address_list()
