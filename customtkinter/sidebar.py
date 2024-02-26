from tkinter import Label, StringVar, OptionMenu
from tkinter.ttk import Treeview
from customtkinter import CTkFrame, CTkEntry, CTkButton


class Sidebar(CTkFrame):
    def __init__(self, master):
        super().__init__(master, width=330)

        self.grid(row=1, column=0, sticky="nsew")
        self.grid_columnconfigure(0, weight=0)

        self.create_input_section()
        self.create_interface_section()
        self.create_ip_table()

    def create_input_section(self):
        input_frame = CTkFrame(master=self)
        input_frame.grid(row=0, column=0, columnspan=2, sticky="nsew", pady=5)

        Label(master=input_frame, text="IP Address").grid(
            row=0, column=0, padx=5, pady=2
        )
        self.input_entry = CTkEntry(master=input_frame, width=200)
        self.input_entry.grid(row=1, column=0, padx=5, pady=2)

        input_button = CTkButton(
            master=input_frame, text="Add IP Address", command=self.add_ip_address
        )
        input_button.grid(row=1, column=1, sticky="nsew", padx=5, pady=2)

    def create_interface_section(self):
        dropdown_row = CTkFrame(master=self)
        dropdown_row.grid(row=1, column=0, columnspan=2, sticky="nsew", pady=5)

        self.interface_list = ["Ethernet", "Wireless LAN", "Bluetooth", "Loopback"]
        self.interface_variable = StringVar(self)
        self.interface_variable.set(self.interface_list[0])

        self.dropdown = OptionMenu(
            dropdown_row, self.interface_variable, *self.interface_list
        )
        self.dropdown.grid(row=0, column=0, sticky="nsew", padx=5)

        dropdown_button = CTkButton(master=dropdown_row, text="Probe")
        dropdown_button.grid(row=0, column=1, sticky="nsew", padx=5)

    def create_ip_table(self):
        self.table_frame = CTkFrame(master=self)
        self.table_frame.grid(row=2, column=0, columnspan=2, sticky="nsew", pady=10)

        self.ip_table = Treeview(self.table_frame)
        self.ip_table.pack(fill="both", expand=True)

        self.ip_table["columns"] = ("#", "IP Address", "MAC Address")
        self.ip_table.column("#0", width=0, stretch=False)
        self.ip_table.column("#", width=40, anchor="center")
        self.ip_table.column("IP Address", anchor="center")
        self.ip_table.column("MAC Address", anchor="center")

        self.ip_table.heading("#0", text="")  # Hide first index column
        self.ip_table.heading("#", text="#")
        self.ip_table.heading("IP Address", text="IP Address")
        self.ip_table.heading("MAC Address", text="MAC Address")

    def add_ip_address(self):
        pass
