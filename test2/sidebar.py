import tkinter as tk
import customtkinter as ctk

from interfaces import interface_info


class Sidebar(ctk.CTkFrame):
    def __init__(self, master):
        super().__init__(master=master, width=330)

        self.grid(row=1, column=0, sticky="nsew")
        self.grid_columnconfigure(0, weight=0)

        self.interface_list = interface_info()
        self.ip_addresses = []

        self.input_frame = ctk.CTkFrame(master=self)
        self.input_frame.grid(row=0, column=0, columnspan=2, sticky="nsew", pady=5)

        tk.Label(master=self.input_frame, text="IP Address").grid(
            row=0, column=0, padx=5, pady=2
        )

        self.input_entry = ctk.CTkEntry(
            master=self.input_frame,
            width=200,
            placeholder_text=("Enter IP Address ..."),
        )
        self.input_entry.grid(row=1, column=0, padx=5, pady=2)

        ctk.CTkButton(
            master=self.input_frame, text="Add IP Address", command=self.add_ip_address
        ).grid(row=1, column=1, sticky="nsew", padx=5, pady=2)

        self.dropdown_row = ctk.CTkFrame(master=self)
        self.dropdown_row.grid(row=1, column=0, columnspan=2, sticky="nsew", pady=5)

        self.interface_variable = tk.StringVar(self)
        self.interface_variable.set(self.interface_list[0])

        tk.OptionMenu(
            self.dropdown_row, self.interface_variable, *self.interface_list
        ).grid(row=0, column=0, sticky="nsew", padx=5)

        ctk.CTkButton(
            master=self.dropdown_row, text="Probe", command=self.update_interface_list
        ).grid(row=0, column=1, sticky="nsew", padx=5)

        self.input_frame.grid_columnconfigure(0, weight=1)
        self.input_frame.grid_columnconfigure(1, weight=1)
        self.dropdown_row.grid_columnconfigure(0, weight=1)
        self.dropdown_row.grid_columnconfigure(1, weight=1)

        self.table_frame = ctk.CTkFrame(master=self)
        self.table_frame.grid(row=2, column=0, columnspan=2, sticky="nsew", pady=10)
        self.ip_address_list()

    def add_ip_address(self):
        new_ip_address = self.input_entry.get()
        self.input_entry.delete(0, tk.END)

        if new_ip_address:
            new_row = {
                "#": len(self.ip_addresses) + 1,
                "IP Address": new_ip_address,
                "MAC Address": "KUAY:HEEE:TADD",
            }
            self.ip_addresses.append(new_row)
            self.ip_address_list()

    def ip_address_list(self):
        for widget in self.table_frame.winfo_children():
            widget.destroy()

        header_labels = ["#", "IP Address", "MAC Address"]
        for col_num, col_name in enumerate(header_labels):
            ctk.CTkLabel(
                self.table_frame,
                text=col_name,
                font=ctk.CTkFont(weight="bold"),
            ).grid(row=0, column=col_num, padx=5, pady=2)

        for row_num, row_data in enumerate(self.ip_addresses):
            for col_num, (key, value) in enumerate(row_data.items()):
                ctk.CTkLabel(self.table_frame, text=str(value)).grid(
                    row=row_num + 1, column=col_num, padx=5, pady=2
                )

    def update_interface_list(self):
        interfaces = interface_info()
        self.interface_list = ["--Select--"] + interfaces
        self.interface_variable.set(self.interface_list[0])

        menu = self.dropdown_row.children["!optionmenu"]
        menu["menu"].delete(0, "end")
        for interface in self.interface_list:
            menu["menu"].add_command(
                label=interface, command=tk._setit(self.interface_variable, interface)
            )
