import customtkinter as ctk


class Navbar(ctk.CTkFrame):
    def __init__(self, master):
        super().__init__(master)

        self.grid(row=0, column=0, columnspan=2, sticky="nsew")

        title_label = ctk.CTkLabel(
            master=self,
            text="Network Scanner Tool with Rust",
            font=ctk.CTkFont(size=30, weight="bold"),
        )
        title_label.grid(row=0, column=0, sticky="w", padx=5, pady=5)

        button_frame = ctk.CTkFrame(master=self)
        button_frame.grid(row=0, column=1, sticky="ne")

        ctk.CTkButton(master=button_frame, text="History").pack(
            side="left", padx=5, pady=5
        )
        ctk.CTkButton(master=button_frame, text="Home").pack(
            side="left", padx=5, pady=5
        )

        def switch_theme():
            self.master.color = self.master.color = (
                "light" if self.master.color == "dark" else "dark"
            )
            ctk.set_appearance_mode(self.master.color)

        ctk.CTkButton(master=button_frame, text="Theme", command=switch_theme).pack(
            side="left", padx=(100, 5), pady=5
        )
