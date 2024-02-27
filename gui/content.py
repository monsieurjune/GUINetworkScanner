import customtkinter as ctk


class Content:
    def __init__(self, master):
        self.content_frame = ctk.CTkFrame(master)
        self.content_frame.grid(row=1, column=1, sticky="nsew")
        self.scan_result_list(self.content_frame)

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
