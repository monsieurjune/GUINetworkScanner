import tkinter as tk
from tkinter import ttk
import customtkinter as ctk

from __content import Content
from utils.interface import interface_info


class Sidebar(ctk.CTkFrame):
    def __init__(self, master) -> None:
        super().__init__(master=master, width=330)

        if master is None:
            return

        self.grid(row=1, column=0, sticky="nsew")
        self.grid_columnconfigure(index=0, weight=0)

        self.column_width = 120

        self.interface_list = interface_info()
        self.ip_addresses = []
        self.selected_ip_address = []

        self.result_frame = ctk.CTkFrame(master=self)

        # Input Frame
        self.input_frame = ctk.CTkFrame(master=self, fg_color="transparent")
        self.input_frame.grid(row=0, column=0, columnspan=2, sticky="nsew", pady=5)
        self.input_frame.grid_columnconfigure(index=0, weight=1)

        # Input Title Label
        self.title_label = ctk.CTkLabel(
            master=self.input_frame,
            text="IP Address List",
            font=ctk.CTkFont(family=("JetBrains Mono"), size=33),
            fg_color="transparent",
        )
        self.title_label.grid(row=0, column=0, columnspan=2, padx=10, pady=10)

        # Input Entry
        self.input_entry = ctk.CTkEntry(
            master=self.input_frame,
            width=200,
            placeholder_text=("Enter IP Address ..."),
        )
        self.input_entry.grid(row=1, column=0, padx=5, pady=2)

        # Add IP Address Button
        ctk.CTkButton(
            master=self.input_frame, text="Add IP Address", command=self.add_ip_address
        ).grid(row=1, column=1, sticky="nsew", padx=5)

        # Dropdown Frame
        self.dropdown_row = ctk.CTkFrame(master=self)
        self.dropdown_row.grid(row=1, column=0, columnspan=2, sticky="nsew", pady=5)
        self.dropdown_row.grid_rowconfigure(index=0, weight=1)

        # Network Interface Dropdown Label
        ctk.CTkLabel(
            master=self.dropdown_row,
            text="─── Select Network Interface ───",
            font=ctk.CTkFont(family=("JetBrains Mono"), size=13),
            text_color=("gray10", "#abb2b9"),
        ).grid(row=0, column=0, columnspan=2)

        self.interface_list: list[str] = ["─Select Interface─"]
        self.interface_variable = tk.StringVar(master=self)
        self.interface_variable.set(value=self.interface_list[0])

        # Interface Combobox
        self.interface_combobox = ctk.CTkComboBox(
            master=self.dropdown_row,
            values=self.interface_list,
            variable=self.interface_variable,
            state="readonly",
            width=200,
        )
        self.interface_combobox.grid(row=1, column=0, padx=5, sticky="nsew")
        self.interface_variable.set(value=self.interface_list[0])

        # Probe button
        ctk.CTkButton(
            master=self.dropdown_row, text="Probe", command=self.update_interface_list
        ).grid(row=1, column=1, padx=5, sticky="nsew")

        # Table Frame
        self.table_frame = ctk.CTkFrame(master=self)
        self.table_frame.grid(row=2, column=0, columnspan=2, sticky="nsew", pady=10)
        self.grid_rowconfigure(index=2, weight=1)
        self.ip_address_list()

        # Scan Options Frame
        self.scan_frame = ctk.CTkFrame(
            master=self,
            bg_color="transparent",
            fg_color="transparent",
        )
        self.scan_frame.grid(row=3, column=0, columnspan=2, sticky="nsew", pady=10)
        self.scan_frame.grid_columnconfigure(index=0, weight=1)
        self.scan_frame.grid_columnconfigure(index=1, weight=1)

        self.scan_mode_title = ctk.CTkLabel(
            master=self.scan_frame,
            text="Scan Mode",
            font=ctk.CTkFont(family=("JetBrains Mono"), size=20),
            bg_color="transparent",
            fg_color="transparent",
            text_color=("gray10", "#abb2b9"),
        )
        self.scan_mode_title.grid(row=0, column=0, columnspan=2, padx=5, pady=(5, 15))

        # Scan Speed Ratio Buttons
        self.scan_mode = tk.StringVar(value="Fast")
        ctk.CTkRadioButton(
            master=self.scan_frame,
            text="Fast",
            variable=self.scan_mode,
            value="Fast",
        ).grid(row=1, column=0, padx=5, pady=2)

        ctk.CTkRadioButton(
            master=self.scan_frame,
            text="Full",
            variable=self.scan_mode,
            value="Full",
        ).grid(row=1, column=1, padx=5, pady=2)

        self.button_frame = ctk.CTkFrame(
            master=self, fg_color="transparent", bg_color="transparent"
        )
        self.button_frame.grid(
            row=4, column=0, columnspan=5, sticky="nsew", pady=(10, 0)
        )
        self.button_frame.grid_columnconfigure(index=0, weight=1)
        self.button_frame.grid_columnconfigure(index=1, weight=1)

        # Scan Button
        ctk.CTkButton(
            master=self.button_frame,
            text="Scan",
            command=self.scan,
        ).grid(row=0, column=0, columnspan=2, padx=25, pady=5, sticky="ew")

    def scan(self) -> None:
        global scan_state
        print("Scanning ...")
        self.content = Content(master=self.master)

    def add_ip_address(self) -> None:
        new_ip_address: str = self.input_entry.get()
        self.input_entry.delete(first_index=0, last_index=tk.END)

        if new_ip_address:
            new_row = {"#": len(self.ip_addresses) + 1, "IP Address": new_ip_address}
            self.ip_addresses.append(new_row)
            self.ip_address_list()

    def ip_address_list(self):
        global tree

        for widget in self.table_frame.winfo_children():
            widget.destroy()

        # Treeview Setup
        tree = ttk.Treeview(
            master=self.table_frame,
            columns=["#", "IP Address"],
            show="headings",
        )
        tree.pack(fill="both", expand=True)

        # Apply ttk Styles
        style = ttk.Style()
        style.theme_use(themename="default")
        style.configure(
            style="Treeview",
            background="#2a2d2e",
            foreground="white",
            rowheight=25,
            fieldbackground="#343638",
        )
        style.map(
            style="Treeview",
            background=[("selected", "#22559b")],
            foreground=[("selected", "yellow")],
        )
        style.configure(
            style="Treeview.Heading",
            background="#565b5e",
            foreground="white",
            relief="flat",
        )
        style.map(style="Treeview.Heading", background=[("active", "#3484F0")])

        # Define treeview columns
        tree.heading(column="#", text="#")
        tree.heading(column="IP Address", text="IP Address")

        # Populate treeview with data
        for row in self.ip_addresses:
            tree.insert(parent="", index="end", values=(row["#"], row["IP Address"]))

        # Adjust column widths
        tree.column(column="#", width=50, anchor="center")
        tree.column(column="IP Address", width=200, anchor="center")

        tree.configure(height=10)

        # Set the height of the treeview to 10 rows
        tree.bind(sequence="<ButtonRelease-1>", func=self.on_tree_select)
        tree.bind(sequence="<Double-Button-1>", func=self.select_ip_address)

    def select_ip_address(self, event):
        item = tree.focus()
        if not item:
            return

        selected_ip = tree.item(item)["values"][1]

        if selected_ip in self.selected_ip_address:
            self.selected_ip_address.remove(selected_ip)
        else:
            self.selected_ip_address.append(selected_ip)

        print("Selected IP Addresses:")
        for ip in self.selected_ip_address:
            print(ip)

    def on_tree_select(self, event):
        selected_items = tree.selection()

        for item in tree.get_children():
            tree.item(item, tags=[])

        for item in selected_items:
            tree.item(item, tags="selected")

    def update_interface_list(self) -> None:
        interfaces = interface_info()
        self.interface_list = interfaces
        self.interface_variable.set(value=self.interface_list[0])

        menu = self.dropdown_row.children["!optionmenu"]
        menu["menu"].delete(0, "end")
        for interface in self.interface_list:
            menu["menu"].add_command(
                label=interface,
                command=tk._setit(var=self.interface_variable, value=interface),
            )


if __name__ == "__main__":
    root = ctk.CTk()
    root.geometry(geometry_string="1024x768")
    app = Sidebar(master=root)
    root.mainloop()
