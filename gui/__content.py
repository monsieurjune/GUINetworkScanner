import csv
import tkinter as tk
from tkinter import ttk
import customtkinter as ctk

from utils.ip_address import get_result


def read_port_csv(file_path):  # -> dict:
    port_map = {}
    with open(file_path, newline="") as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            port = int(row["port"])
            description = row["description"]
            port_map[port] = description
    return port_map


class Content:
    def __init__(self, master) -> None:
        self.content_frame = ctk.CTkFrame(master=master)
        self.content_frame.grid(row=1, column=1, sticky="nsew")
        self.content_frame.grid_columnconfigure(index=0, weight=1)

        self.column_width = 120
        self.scan_result_list(content_frame=self.content_frame)

    def scan_result_list(self, content_frame) -> None:
        title_label = ctk.CTkLabel(
            master=content_frame,
            text="Scan Result",
            font=ctk.CTkFont(family=("JetBrains Mono"), size=33),
            text_color="white",
        )
        title_label.pack(pady=15)

        header_labels: list[str] = ["#", "IP Address", "Port", "Description"]

        # Scan Result Table
        tree = ttk.Treeview(
            master=content_frame, columns=header_labels, show="headings"
        )
        tree.pack(fill="both", expand=True)

        # Table Styling
        style = ttk.Style()
        style.theme_use(themename="default")
        style.configure(
            style="Treeview",
            background="#2a2d2e",
            foreground="white",
            fieldbackground="#343638",
            bordercolor="#343638",
            borderwidth=0,
        )
        style.map(style="Treeview", background=[("selected", "#22559b")])
        style.configure(
            style="Treeview.Heading",
            background="#565b5e",
            foreground="white",
            relief="flat",
        )
        style.map(style="Treeview.Heading", background=[("active", "#3484F0")])

        # Define Columns
        for col in header_labels:
            tree.heading(column=col, text=col)
            tree.column(column=col, width=self.column_width, anchor="center")

        # Data
        data = get_result()
        print("Data:", data)
        ip_address = data.get("ipaddr", "No IP Address Found")
        tcp_port = data.get("tcp_port", [])

        print("IP Address:", ip_address)
        print("TCP Ports:", tcp_port)

        if not tcp_port:
            print("No TCP Ports Found")
            return

        port_map = read_port_csv("gui/utils/tcp.csv")

        for i, port in enumerate(tcp_port, start=1):
            description = port_map.get(port, "Unknown")
            print("Inserting:", i, ip_address, port, description)
            tree.insert(
                parent="",
                index=tk.END,
                values=[str(i), ip_address, str(port), description],
            )


if __name__ == "__main__":
    root = ctk.CTk()
    root.geometry(geometry_string="1024x768")
    app = Content(master=root)
    root.mainloop()
