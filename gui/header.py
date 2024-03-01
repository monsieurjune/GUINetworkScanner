import customtkinter as ctk

from history import History


class Header(ctk.CTkFrame):
    def __init__(self, master) -> None:
        super().__init__(master=master)

        self.grid(row=0, column=0, columnspan=2, sticky="nsew")
        self.grid_columnconfigure(index=0, weight=1)

        title_label = ctk.CTkLabel(
            master=self,
            text="Network Scanner Tool with Rust",
            font=ctk.CTkFont(family="JetBrains Mono", size=40, weight="bold"),
        )
        title_label.grid(row=0, column=0, sticky="w", padx=15, pady=20)

        history_button = ctk.CTkButton(
            master=self,
            text="History Logs",
            font=ctk.CTkFont(size=25, weight="bold"),
            height=69,
            command=self.show_history_popup,
        )
        history_button.grid(row=0, column=1, sticky="n", padx=10, pady=30)

    def show_history_popup(self) -> None:
        history_window = History(master=self.master)
        history_window.show_popup()


if __name__ == "__main__":
    root = ctk.CTk()
    root.geometry(geometry_string="1024x768")
    app = Header(master=root)
    root.mainloop()
