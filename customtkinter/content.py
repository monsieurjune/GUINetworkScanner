from customtkinter import CTkFrame
from tkinter.ttk import Treeview


class Content(CTkFrame):
    def __init__(self, master):
        super().__init__(master)

        self.grid(row=1, column=1, sticky="nsew")
        self.create_scan_result_table()

    def create_scan_result_table(self):
        self.table_frame = CTkFrame(master=self)
        self.table_frame.pack(fill="both", expand=True)

        self.scan_result_table = Treeview(self.table_frame)
        self.scan_result_table.pack(fill="both", expand=True)

        self.scan_result_table["columns"] = (
            "Protocol",
            "Port",
            "Description",
            "State",
        )
        self.scan_result_table.column("#0", width=0, stretch=False)
        self.scan_result_table.column("Protocol", anchor="center")
        self.scan_result_table.column("Port", anchor="center")
        self.scan_result_table.column("Description", anchor="center")
        self.scan_result_table.column("State", anchor="center")

        self.scan_result_table.heading("#0", text="")
        self.scan_result_table.heading("Protocol", text="Protocol")
        self.scan_result_table.heading("Port", text="Port")
        self.scan_result_table.heading("Description", text="Description")
        self.scan_result_table.heading("State", text="State")

        initial_data = [
            ("TCP", "69", "KUAY", "OPEN"),
            ("UDP", "420", "HEE", "FILTERED"),
            ("TCP", "55555", "TAD", "OPEN"),
        ]
        for item in initial_data:
            self.scan_result_table.insert(parent="", index="end", values=item)

    def update_scan_results(self, new_data):
        for item in self.scan_result_table.get_children():
            self.scan_result_table.delete(item)

        for item in new_data:
            self.scan_result_table.insert(parent="", index="end", values=item)
