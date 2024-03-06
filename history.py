import tkinter as tk
from tkinter import ttk
import customtkinter as ctk


class History:
    def __init__(self, master) -> None:
        self.master = master

    def show_popup(self) -> None:
        popup = ctk.CTkToplevel(self.master)
        popup.title(string="History Logs")
        popup.geometry(geometry_string="1024x768")
        popup.resizable(width=False, height=False)

        date_label = ctk.CTkLabel(master=popup, text="Select Date:")
        date_label.pack(pady=(10, 5))

        date_values: list[str] = [
            "29/02/2024",
            "28/02/2024",
            "27/02/2024",
            "26/02/2024",
            "25/02/2024",
            "24/02/2024",
            "23/02/2024",
            "22/02/2024",
            "21/02/2024",
            "20/02/2024",
        ]

        date_variable = ctk.StringVar(value="29/02/2024")
        date_combobox = ctk.CTkOptionMenu(
            master=popup, values=date_values, variable=date_variable
        )
        date_combobox.pack(pady=5)

        table_frame = ctk.CTkFrame(master=popup)
        table_frame.pack(pady=10, fill="both", expand=True)

        header_labels: list[str] = [
            "#",
            "IP Address",
            "Protocol",
            "Port",
            "Description",
            "Status",
        ]
        tree = ttk.Treeview(master=table_frame, columns=header_labels, show="headings")
        tree.pack(fill="both", expand=True)

        def resize_column(event) -> None:
            total_width: int = tree.winfo_width()
            for col in header_labels:
                tree.column(column=col, width=int(total_width / len(header_labels) - 5))

        popup.bind(sequence="<Configure>", func=resize_column)

        style = ttk.Style()
        style.theme_use(themename="default")
        style.configure(
            style="Treeview",
            background="#2a2d2e",
            foreground="white",
            rowheight=25,
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

        for col in header_labels:
            tree.heading(column=col, text=col, anchor="center")
            tree.column(column=col, anchor="center")

        sample_data: list = [
            (1, "127.0.0.1", "TCP", "135", "DCE endpoint resolution", "Open"),
            (2, "127.0.0.1", "TCP", "445", "Microsoft-DS", "OPEN"),
        ]

        for item in sample_data:
            tree.insert(parent="", index=tk.END, values=item)

        ctk.CTkButton(master=popup, text="Close", command=popup.destroy).pack(pady=15)


if __name__ == "__main__":
    root = ctk.CTk()
    root.geometry(geometry_string="1024x768")
    app = History(master=root)
    root.mainloop()
