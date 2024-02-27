import tkinter as tk
import customtkinter as ctk


class App(ctk.CTk):
    def __init__(self):
        super().__init__()

        self.title("Network Scanner Tool with Rust")
        self.geometry("1024x768")
        self.color = "dark"

        self.ip_addresses = []
        self.interfaces = []
        self.scan_result = []

        self.grid_rowconfigure(0, weight=0)
        self.grid_rowconfigure(1, weight=1)
        self.grid_columnconfigure(0, weight=0)
        self.grid_columnconfigure(1, weight=1)

        self.navbar()
        self.sidebar()
        self.main_content()

    def navbar(self):
        navbar_frame = ctk.CTkFrame(self)
        navbar_frame.grid(row=0, column=0, columnspan=2, sticky="nsew")

        title_label = ctk.CTkLabel(
            master=navbar_frame,
            text="Network Scanner Tool with Rust",
            font=ctk.CTkFont(size=30, weight="bold"),
        )
        title_label.grid(row=0, column=0, sticky="w", padx=5, pady=5)

        button_frame = ctk.CTkFrame(master=navbar_frame)
        button_frame.grid(row=0, column=1, sticky="ne")

        history_button = ctk.CTkButton(master=button_frame, text="History")
        history_button.pack(side="left", padx=5, pady=5)

        home_button = ctk.CTkButton(master=button_frame, text="Home")
        home_button.pack(side="left", padx=5, pady=5)

        def switch_theme():
            self.color = self.color = "light" if self.color == "dark" else "dark"
            ctk.set_appearance_mode(self.color)

        theme_button = ctk.CTkButton(
            master=button_frame, text="Theme", command=switch_theme
        )
        theme_button.pack(side="left", padx=(100, 5), pady=5)

    def sidebar(self):
        sidebar_frame = ctk.CTkFrame(master=self, width=330)
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

        def add_ip_address():
            new_ip_address = input_entry.get()
            input_entry.delete(0, tk.END)

            if new_ip_address:
                new_row = {
                    "#": len(self.ip_addresses) + 1,
                    "IP Address": new_ip_address,
                    "MAC Address": "KUAY:HEEE:TADD",
                }

            self.ip_addresses.append(new_row)
            self.ip_address_list()

        input_button = ctk.CTkButton(
            master=input_frame, text="Add IP Address", command=add_ip_address
        )
        input_button.grid(row=1, column=1, sticky="nsew", padx=5, pady=2)

        dropdown_row = ctk.CTkFrame(master=sidebar_frame)
        dropdown_row.grid(row=1, column=0, columnspan=2, sticky="nsew", pady=5)

        self.interface_list = ["--Select--", "WLAN", "vEthernet (Default Switch)"]
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
        self.ip_address_list()

    def main_content(self):
        content_frame = ctk.CTkFrame(self)
        content_frame.grid(row=1, column=1, sticky="nsew")
        self.scan_result_list(content_frame)

    def ip_address_list(self):
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

    def scan_result_list(self, content_frame):
        scan_result = [
            ["#", "Protocol", "Port", "Description", "State"],
            ["1", "TCP", "69", "KUAY", "OPEN"],
            ["2", "UDP", "420", "HEE", "FILTERED"],
            ["3", "TCP", "55555", "TAD", "OPEN"],
        ]

        # Header
        header_frame = ctk.CTkFrame(content_frame)
        header_frame.pack()
        for column_name in scan_result[0]:
            label = ctk.CTkLabel(
                master=header_frame,
                text=column_name,
                width=80,
                font=ctk.CTkFont(weight="bold", size=12),
            )
            label.pack(side="left", padx=5)

        for row in scan_result[1:]:
            row_frame = ctk.CTkFrame(content_frame)
            row_frame.pack()
            for item in row:
                label = ctk.CTkLabel(
                    master=row_frame,
                    text=str(item),
                    width=80,
                )
                label.pack(side="left", padx=5)


if __name__ == "__main__":
    app = App()
    app.mainloop()
