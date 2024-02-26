import customtkinter as ctk
from customtkinter import CTkLabel, CTkButton, CTkFrame, CTkFont


class Navbar(CTkFrame):
    def __init__(self, master, app):
        super().__init__(master)

        self.grid(row=0, column=0, columnspan=2, sticky="nsew")
        self.app = app

        self.create_title_section()
        self.create_button_section()

    def create_title_section(self):
        self.title_label = CTkLabel(
            master=self,
            text="Network Scanner Tool with Rust",
            font=CTkFont(size=24, weight="bold"),
        )
        self.title_label.grid(row=0, column=0, sticky="w", padx=5, pady=5)

    def create_button_section(self):
        self.button_frame = CTkFrame(master=self)
        self.button_frame.grid(row=0, column=1, sticky="ne")

        self.history_button = CTkButton(master=self.button_frame, text="History")
        self.history_button.pack(side="left", padx=5, pady=5)

        self.home_button = CTkButton(master=self.button_frame, text="Home")
        self.home_button.pack(side="left", padx=5, pady=5)

        self.theme_button = CTkButton(
            master=self.button_frame,
            text="Theme",
            command=self.switch_theme,
        )
        self.theme_button.pack(side="left", padx=5, pady=5)

    def switch_theme(self):
        if ctk.get_appearance_mode() == "dark":
            ctk.set_appearance_mode("light")
        else:
            ctk.set_appearance_mode("dark")

        self.app.update_theme()
