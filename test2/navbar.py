import customtkinter as ctk


class Navbar:
    def __init__(self, master):
        self.master = master
        self.create_navbar()

    def create_navbar(self):
        navbar_frame = ctk.CTkFrame(self.master)
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
