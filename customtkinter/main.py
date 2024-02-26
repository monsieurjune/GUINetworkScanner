import customtkinter as ctk

from navbar import Navbar
from sidebar import Sidebar
from content import Content


class App(ctk.CTk):
    def __init__(self):
        super().__init__()

        self.title("Network Scanner Tool with Rust")
        self.geometry("1024x768")
        self.color = "dark"

        self.navbar = Navbar(self, self)  # Pass a reference to App
        self.navbar.grid(row=0, column=0, columnspan=2, sticky="nsew")
        self.sidebar = Sidebar(self)
        self.sidebar.grid(row=1, column=0, sticky="nsew")

        self.main_content = Content(self)
        self.main_content.grid(row=1, column=1, sticky="nsew")

        self.grid_rowconfigure(0, weight=0)  # Navbar doesn't expand vertically
        self.grid_rowconfigure(1, weight=1)  # Main content area expands
        self.grid_columnconfigure(0, weight=0)  # Sidebar doesn't expand horizontally
        self.grid_columnconfigure(1, weight=1)  # Main content expands

    def switch_theme(self):
        if self.color == "dark":
            ctk.set_appearance_mode("light")
            self.color = "light"
        else:
            ctk.set_appearance_mode("dark")
            self.color = "dark"

        self.update_theme()

    def update_theme(self):
        pass


if __name__ == "__main__":
    app = App()
    app.mainloop()
